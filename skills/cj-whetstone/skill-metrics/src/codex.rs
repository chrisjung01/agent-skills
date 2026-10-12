//! Codex session transcripts (`<sessions>/YYYY/MM/DD/rollout-<time>-<session id>.jsonl`) into the
//! events of `transcript`, so windows, metrics and `show` work as for Claude Code.
//!
//! A skill call is a user prompt that holds `$<skill>` followed by the `<skill>` block Codex
//! attaches for it; Codex attaches one only for a skill the user picked. The prompt becomes a
//! `Command` per attached skill and the block its `SkillText`, so the call is a `slash` call and
//! the prompt itself is no user text, like a slash command's arguments. A `$<skill>` in prose
//! without an attached block, agent text and the available-skills catalog (a developer message)
//! start nothing. Codex shows no activation for a skill the model reads on its own: such skills
//! are neither calls nor nested skills. A subagent's rollout starts with a replay of its parent's
//! history, the parent's skill calls included, and shows nothing of its own, so it yields no
//! events.
//!
//! Two skills attached to one prompt are two calls; the first ends where the second starts, so
//! its window is empty.
//!
//! Measured: user prompts and texts, interrupts, tokens, models, active duration and context.
//! Codex writes no denials, choice-question answers or tool errors the way Claude Code does, so
//! those counters stay unmeasured (`NULL` in the store).
//!
//! Lines that are not valid JSON (the half-written last line of a running session) are skipped.

use crate::transcript::{Context, Event, Kind, Request, Tokens};
use crate::version::without_frontmatter;
use serde_json::Value;
use std::path::Path;

/// `SkillCall::source` of a call from a Codex transcript.
pub const SOURCE: &str = "codex";
/// Codex writes its own context into user messages; such parts are no prompt.
const INJECTED_PREFIX: &str = "# AGENTS.md instructions";

/// The session id at the end of a rollout file's name; the whole stem for any other name.
pub fn session_id(path: &Path) -> String {
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    // A uuid is 36 characters.
    let id = stem.len().checked_sub(36).and_then(|at| stem.get(at..));
    id.unwrap_or(&stem).to_owned()
}

/// Whether `jsonl` is a Codex session: its first record is the session's `session_meta`.
pub fn is_rollout(jsonl: &str) -> bool {
    jsonl
        .lines()
        .find_map(|line| serde_json::from_str::<Value>(line).ok())
        .is_some_and(|entry| entry["type"] == "session_meta")
}

/// The transcript of one Codex session.
pub fn parse(session_id: &str, jsonl: &str) -> Vec<Event> {
    let mut parser = Parser {
        session_id,
        context: Context::default(),
        model: None,
        total: None,
        events: Vec::new(),
        prompt: None,
        meta: false,
        subagent: false,
    };
    for (line, entry) in jsonl.lines().enumerate() {
        if let Ok(entry) = serde_json::from_str::<Value>(entry) {
            parser.entry(line, &entry);
        }
    }
    parser.flush();
    if parser.subagent {
        return Vec::new();
    }
    parser.events
}

struct Parser<'a> {
    session_id: &'a str,
    context: Context,
    model: Option<String>,
    /// The session's token total at the last token count, to skip a repeated count.
    total: Option<Value>,
    events: Vec<Event>,
    /// The latest prompt and the skill blocks attached to it, held until the next event.
    /// ponytail: a collection between the prompt and its block (a few ms) stores the prompt in
    /// the previous call, whose longer window the store then keeps.
    prompt: Option<(Event, String, Vec<Event>)>,
    /// The first `session_meta` is the session's; a fork or subagent repeats its parent's after it.
    meta: bool,
    subagent: bool,
}

impl Parser<'_> {
    fn entry(&mut self, line: usize, entry: &Value) {
        let payload = &entry["payload"];
        let timestamp = entry["timestamp"].as_str().unwrap_or_default().to_owned();
        // Codex before 0.145 writes no message ids; the line's ordinal is stable while the file
        // only grows. ponytail: a fork of such a session gets other keys for its parent's calls
        // and the full scan stores them twice; no such fork was seen, skip its replay if one is.
        let ordinal = entry["ordinal"].as_u64().unwrap_or(line as u64);
        let uuid = payload["id"]
            .as_str()
            .map_or_else(|| format!("{}:{ordinal}", self.session_id), str::to_owned);
        let kind = match (entry["type"].as_str(), payload["type"].as_str()) {
            (Some("session_meta"), _) if !self.meta => {
                self.meta = true;
                self.subagent = !payload["source"]["subagent"].is_null();
                self.context = Context {
                    version: string(&payload["cli_version"]),
                    entrypoint: string(&payload["originator"]),
                    cwd: string(&payload["cwd"]),
                    git_branch: string(&payload["git"]["branch"]),
                };
                return;
            }
            (Some("session_meta"), _) => return,
            (Some("turn_context"), _) => {
                self.model = string(&payload["model"]).or(self.model.take());
                self.context.cwd = string(&payload["cwd"]).or(self.context.cwd.take());
                return;
            }
            (Some("response_item"), Some("message")) => match payload["role"].as_str() {
                Some("user") => return self.user_message(uuid, timestamp, &payload["content"]),
                Some("assistant") => {
                    let text = texts(&payload["content"], "output_text").join("\n");
                    if text.trim().is_empty() {
                        return;
                    }
                    Kind::AgentText(text)
                }
                _ => return,
            },
            (Some("response_item"), Some("function_call" | "custom_tool_call")) => {
                let input = match &payload["arguments"] {
                    Value::String(arguments) => serde_json::from_str(arguments)
                        .unwrap_or_else(|_| Value::String(arguments.clone())),
                    _ => payload["input"].clone(),
                };
                Kind::ToolUse {
                    id: string(&payload["call_id"]).unwrap_or_default(),
                    name: string(&payload["name"]).unwrap_or_default(),
                    input,
                }
            }
            (Some("event_msg"), Some("turn_aborted")) if payload["reason"] == "interrupted" => {
                Kind::Interrupt
            }
            (Some("event_msg"), Some("token_count")) => {
                let Some(request) = self.request(&uuid, &payload["info"]) else {
                    return;
                };
                let mut event = self.event(uuid, timestamp, Kind::Other);
                event.request = Some(request);
                return self.push(event);
            }
            _ => return,
        };
        let event = self.event(uuid, timestamp, kind);
        self.push(event);
    }

    /// A prompt, a skill block for the prompt before it, or context Codex injected.
    fn user_message(&mut self, uuid: String, timestamp: String, content: &Value) {
        let parts = texts(content, "input_text");
        if let Some((skill, text)) = parts.first().and_then(|part| skill_block(part)) {
            let Some((prompt, prompt_text, _)) = &self.prompt else {
                return;
            };
            if !prompt_text.contains(&format!("${skill}")) {
                return;
            }
            let command = self.event(uuid.clone(), prompt.timestamp.clone(), Kind::Command(skill));
            let mut loaded = self.event(
                format!("{uuid}:text"),
                timestamp,
                Kind::SkillText {
                    text,
                    tool_use_id: None,
                },
            );
            loaded.parent_uuid = Some(uuid);
            if let Some((_, _, blocks)) = &mut self.prompt {
                blocks.extend([command, loaded]);
            }
            return;
        }
        let typed: Vec<&str> = parts
            .into_iter()
            .filter(|part| !injected(part))
            .map(str::trim)
            .filter(|part| !part.is_empty())
            .collect();
        if typed.is_empty() {
            return;
        }
        let text = typed.join("\n");
        let event = self.event(uuid, timestamp, Kind::UserPrompt(text.clone()));
        self.flush();
        self.prompt = Some((event, text, Vec::new()));
    }

    /// The tokens of the request a count reports; `None` for a count that repeats the last total.
    fn request(&mut self, uuid: &str, info: &Value) -> Option<Request> {
        let total = &info["total_token_usage"];
        if total.is_null() || self.total.as_ref() == Some(total) {
            return None;
        }
        self.total = Some(total.clone());
        let last = &info["last_token_usage"];
        let count = |key: &str| last[key].as_i64().unwrap_or(0);
        let cache_read = count("cached_input_tokens");
        Some(Request {
            message_id: uuid.to_owned(),
            // `<synthetic>` is listed as no model; a count before any turn has none.
            model: self.model.clone().unwrap_or_else(|| "<synthetic>".into()),
            tokens: Tokens {
                // OpenAI counts cached input inside the input; Claude Code apart from it.
                input: (count("input_tokens") - cache_read).max(0),
                output: count("output_tokens"),
                cache_read,
                cache_write: count("cache_write_input_tokens"),
            },
        })
    }

    fn event(&self, uuid: String, timestamp: String, kind: Kind) -> Event {
        Event {
            uuid,
            parent_uuid: None,
            timestamp,
            kind,
            request: None,
            context: self.context.clone(),
        }
    }

    fn push(&mut self, event: Event) {
        self.flush();
        self.events.push(event);
    }

    /// The held prompt: its commands and skill texts, or the prompt alone without any.
    fn flush(&mut self) {
        let Some((prompt, _, blocks)) = self.prompt.take() else {
            return;
        };
        if blocks.is_empty() {
            self.events.push(prompt);
        } else {
            // Commands first: each starts its call, the texts follow inside the last window.
            let (commands, loaded): (Vec<_>, Vec<_>) = blocks
                .into_iter()
                .partition(|event| matches!(event.kind, Kind::Command(_)));
            self.events.extend(commands.into_iter().chain(loaded));
        }
    }
}

/// `<skill>\n<name>…</name>\n<path>…</path>\n<SKILL.md>\n</skill>`: the skill and its text in the
/// shape Claude Code loads, the folder line first and the body without frontmatter.
fn skill_block(text: &str) -> Option<(String, String)> {
    let rest = text.strip_prefix("<skill>\n<name>")?;
    let (name, rest) = rest.split_once("</name>\n<path>")?;
    let (path, content) = rest.split_once("</path>\n")?;
    let content = content.trim_end();
    let content = content.strip_suffix("</skill>").unwrap_or(content);
    let folder = path.strip_suffix("/SKILL.md").unwrap_or(path);
    Some((
        name.to_owned(),
        format!("{folder}\n\n{}", without_frontmatter(content)),
    ))
}

/// `<environment_context>`, `<turn_aborted>`, `<image …>`, the AGENTS.md block and the like.
/// ponytail: a prompt the user starts with a tag counts as injected; check the tag name if one is.
fn injected(part: &str) -> bool {
    let part = part.trim_start();
    part.starts_with(INJECTED_PREFIX)
        || part
            .strip_prefix('<')
            .is_some_and(|rest| rest.starts_with(|c: char| c.is_ascii_lowercase() || c == '/'))
}

fn texts<'a>(content: &'a Value, kind: &str) -> Vec<&'a str> {
    content
        .as_array()
        .into_iter()
        .flatten()
        .filter(|part| part["type"] == kind)
        .filter_map(|part| part["text"].as_str())
        .collect()
}

fn string(value: &Value) -> Option<String> {
    value.as_str().map(str::to_owned)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::window::skill_calls;

    const CODEX: &str = include_str!("../tests/fixtures/codex-session.jsonl");
    const SESSION: &str = "01a1c0de-0000-7000-8000-000000000001";

    #[test]
    fn the_session_id_ends_the_file_name() {
        assert_eq!(
            session_id(Path::new(&format!(
                "s/2026/10/12/rollout-2026-10-12T10-00-00-{SESSION}.jsonl"
            ))),
            SESSION
        );
        assert_eq!(session_id(Path::new("short.jsonl")), "short");
    }

    #[test]
    fn a_prompt_with_its_attached_skill_is_one_call_and_mentions_none() {
        let calls = skill_calls(SESSION, &parse(SESSION, CODEX), &[]);
        let summary: Vec<_> = calls
            .iter()
            .map(|c| {
                (
                    c.skill.as_str(),
                    c.start_uuid.as_str(),
                    c.start_kind,
                    &c.started_at[11..23],
                    &c.ended_at[11..23],
                    c.user_prompts,
                )
            })
            .collect();
        assert_eq!(
            summary,
            [
                // The prompt before it only mentions skills, so does the agent and the catalog.
                (
                    "cj-implement",
                    "msg_s1",
                    "slash",
                    "10:01:00.000",
                    "10:03:20.100",
                    1
                ),
                (
                    "cj-review-loop",
                    "msg_s2",
                    "slash",
                    "10:10:00.000",
                    "10:10:30.100",
                    0
                ),
            ]
        );
        assert_eq!(
            calls[0].text,
            "/home/x/skills/cj-implement\n\n# Implement\n\nBuild the ticket."
        );
        assert!(calls.iter().all(|c| c.nested.is_empty()));
    }

    #[test]
    fn an_older_codex_without_message_ids_keys_the_call_by_its_line() {
        let without_ids = CODEX.replace("\"id\":\"msg_s1\",", "");
        assert_ne!(without_ids, CODEX);
        let calls = skill_calls(SESSION, &parse(SESSION, &without_ids), &[]);
        assert_eq!(calls[0].start_uuid, format!("{SESSION}:12"));
    }

    #[test]
    fn a_skill_block_needs_the_mention_in_its_prompt() {
        let unnamed = CODEX.replace("$cj-implement https", "Bitte https");
        let calls = skill_calls(SESSION, &parse(SESSION, &unnamed), &[]);
        assert_eq!(
            calls.iter().map(|c| c.skill.as_str()).collect::<Vec<_>>(),
            ["cj-review-loop"]
        );
    }

    #[test]
    fn a_subagent_rollout_replays_its_parent_and_yields_nothing() {
        let meta = |source: &str| {
            format!(
                "{{\"timestamp\":\"2026-10-12T10:20:00.000Z\",\"type\":\"session_meta\",\
                 \"payload\":{{\"id\":\"sub\",\"source\":{source},\"cli_version\":\"9\"}}}}\n"
            )
        };
        let subagent = meta(r#"{"subagent":{"thread_spawn":{"parent_thread_id":"p"}}}"#);
        assert!(parse("sub", &format!("{subagent}{CODEX}")).is_empty());
        // A forked session keeps its own metadata, not the parent's that follows it.
        let events = parse("fork", &format!("{}{CODEX}", meta("\"cli\"")));
        assert_eq!(events[0].context.version.as_deref(), Some("9"));
    }

    #[test]
    fn two_skills_on_one_prompt_are_two_calls() {
        let second = CODEX
            .lines()
            .find(|line| line.contains("\"id\":\"msg_s2\""))
            .unwrap()
            .replace("msg_s2", "msg_s1b");
        let both = CODEX.replace("$cj-implement https", "$cj-review-loop $cj-implement https");
        let at = both.find("{\"timestamp\":\"2026-10-12T10:01:05").unwrap();
        let both = format!("{}{second}\n{}", &both[..at], &both[at..]);
        let calls = skill_calls(SESSION, &parse(SESSION, &both), &[]);
        let summary: Vec<_> = calls
            .iter()
            .map(|c| (c.skill.as_str(), c.span.len()))
            .collect();
        assert_eq!(summary[..2], [("cj-implement", 1), ("cj-review-loop", 11)]);
    }

    #[test]
    fn a_repeated_token_count_is_counted_once() {
        let events = parse(SESSION, CODEX);
        let requests: Vec<_> = events.iter().filter_map(|e| e.request.as_ref()).collect();
        assert_eq!(requests.len(), 4);
        assert_eq!(
            requests[1].tokens,
            Tokens {
                input: 400,
                output: 50,
                cache_read: 600,
                cache_write: 0
            }
        );
        assert_eq!(requests[1].model, "gpt-6.1-sol");
    }

    #[test]
    fn injected_context_is_no_prompt() {
        assert!(injected("<environment_context>\n</environment_context>"));
        assert!(injected("# AGENTS.md instructions for /r"));
        assert!(injected("</image>"));
        assert!(!injected("[$cj-implement](/x/SKILL.md) los"));
        assert!(!injected("<3 danke"));
    }
}
