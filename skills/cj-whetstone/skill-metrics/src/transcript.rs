//! Claude Code transcript lines (JSONL) into typed events.
//!
//! Entries of an unknown type and lines that are not valid JSON (the half-written last line of a
//! running session) are skipped. A session's own transcript also skips subagent entries kept
//! inline (`isSidechain`); a subagent's transcript consists of nothing else.

use serde_json::Value;

const KNOWN_TYPES: [&str; 4] = ["user", "assistant", "system", "attachment"];
const SKILL_TEXT_PREFIX: &str = "Base directory for this skill";
const INTERRUPT_PREFIX: &str = "[Request interrupted by user";
/// The answer Claude Code writes when the user closes the question dialog.
const DISMISSED_PREFIX: &str = "[User dismissed";
/// In a denial's tool result, the user's reason follows this.
const DENIAL_REASON_MARKER: &str = "To tell you how to proceed, the user said:\n";
/// What Claude Code writes after the marker when the user picks "chat about this" on a question;
/// the user typed nothing yet.
const CLARIFY_PREFIX: &str = "The user wants to clarify these questions.";
const PR_URL_PREFIX: &str = "https://github.com/";
/// Starts of user entries that the user did not type.
const NOT_TYPED: [&str; 5] = [
    "<local-command-",
    "<bash-stdout",
    "<bash-stderr",
    "<task-notification",
    "<command-",
];

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    /// A slash command the user typed, name without the slash (`cj-review-loop`, `clear`).
    Command(String),
    /// Skill text Claude Code loaded. `tool_use_id` is set when the model loaded it with the `Skill`
    /// tool; without it, a slash command (the entry's parent) loaded it.
    SkillText {
        text: String,
        tool_use_id: Option<String>,
    },
    /// The model called the `Skill` tool. It loaded a skill only when a `SkillText` with this id follows.
    SkillTool {
        id: String,
        skill: String,
    },
    /// The model called a tool other than `Skill`.
    ToolUse {
        id: String,
        name: String,
        input: Value,
    },
    /// Text the model wrote to the user; `show` prints it, extraction never stores it.
    AgentText(String),
    ToolResult(ToolResult),
    /// The user stopped the model (`[Request interrupted by user]`, also "... for tool use").
    Interrupt,
    /// A prompt entry, with the text the user typed in it; empty when Claude Code wrote it all.
    UserPrompt(String),
    Other,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolResult {
    pub tool_use_id: String,
    /// `toolDenialKind` as written (`user-rejected`, `permission-rule`, `cancelled`, ...): the tool
    /// did not run because the user or a permission rule rejected it.
    pub denial: Option<String>,
    /// What the user typed when rejecting the tool call.
    pub denial_reason: Option<String>,
    /// The tool ran and failed. A denial is never an error.
    pub error: bool,
    /// One per question of an answered `AskUserQuestion`.
    pub answers: Vec<Answer>,
    /// `https://github.com/<owner>/<repo>/pull/<number>` in the result text, in order.
    pub pr_urls: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Answer {
    /// The first option, alone.
    Recommended,
    /// Other options only.
    Options,
    /// Text that is no option, alone or among picked options; holds that text only.
    OwnWords(String),
    /// No answer to this question, or the user closed the dialog.
    Skipped,
}

#[derive(Debug, Clone)]
pub struct Event {
    pub uuid: String,
    pub parent_uuid: Option<String>,
    pub timestamp: String,
    pub kind: Kind,
    /// The API request of an assistant entry. Claude Code repeats it on every entry of a message.
    pub request: Option<Request>,
    pub context: Context,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub message_id: String,
    pub model: String,
    pub tokens: Tokens,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Tokens {
    pub input: i64,
    pub output: i64,
    pub cache_read: i64,
    pub cache_write: i64,
}

impl std::ops::AddAssign for Tokens {
    fn add_assign(&mut self, other: Tokens) {
        self.input += other.input;
        self.output += other.output;
        self.cache_read += other.cache_read;
        self.cache_write += other.cache_write;
    }
}

/// What Claude Code ran as and where, as written on each entry.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Context {
    pub version: Option<String>,
    pub entrypoint: Option<String>,
    pub cwd: Option<String>,
    pub git_branch: Option<String>,
}

/// The transcript of a session.
pub fn parse(jsonl: &str) -> Vec<Event> {
    parse_entries(jsonl, false)
}

/// The transcript of a subagent (`<session>/subagents/*.jsonl`).
pub fn parse_subagent(jsonl: &str) -> Vec<Event> {
    parse_entries(jsonl, true)
}

fn parse_entries(jsonl: &str, keep_sidechain: bool) -> Vec<Event> {
    jsonl
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .filter(|entry| keep_sidechain || entry["isSidechain"] != true)
        .filter_map(|entry| event(&entry))
        .collect()
}

fn event(entry: &Value) -> Option<Event> {
    let entry_type = entry["type"].as_str()?;
    if !KNOWN_TYPES.contains(&entry_type) {
        return None;
    }
    Some(Event {
        uuid: entry["uuid"].as_str()?.to_owned(),
        parent_uuid: entry["parentUuid"].as_str().map(str::to_owned),
        timestamp: entry["timestamp"].as_str()?.to_owned(),
        kind: match entry_type {
            "user" => user_kind(entry),
            "assistant" => {
                let content = &entry["message"]["content"];
                tool_use(content)
                    .or_else(|| agent_text(content))
                    .unwrap_or(Kind::Other)
            }
            "attachment" => queued_prompt(&entry["attachment"]).unwrap_or(Kind::Other),
            _ => Kind::Other,
        },
        request: (entry_type == "assistant")
            .then(|| request(&entry["message"]))
            .flatten(),
        context: Context {
            version: string(&entry["version"]),
            entrypoint: string(&entry["entrypoint"]),
            cwd: string(&entry["cwd"]),
            git_branch: string(&entry["gitBranch"]),
        },
    })
}

fn string(value: &Value) -> Option<String> {
    value.as_str().map(str::to_owned)
}

fn request(message: &Value) -> Option<Request> {
    let usage = &message["usage"];
    let count = |key: &str| usage[key].as_i64().unwrap_or(0);
    Some(Request {
        message_id: message["id"].as_str()?.to_owned(),
        model: message["model"].as_str()?.to_owned(),
        tokens: Tokens {
            input: count("input_tokens"),
            output: count("output_tokens"),
            cache_read: count("cache_read_input_tokens"),
            cache_write: count("cache_creation_input_tokens"),
        },
    })
}

fn user_kind(entry: &Value) -> Kind {
    let content = &entry["message"]["content"];
    let Some(text) = text_of(content) else {
        return tool_result(entry, content).unwrap_or(Kind::Other);
    };
    if entry["isMeta"] == true {
        return if text.starts_with(SKILL_TEXT_PREFIX) {
            let tool_use_id = entry["sourceToolUseID"].as_str().map(str::to_owned);
            Kind::SkillText { text, tool_use_id }
        } else {
            Kind::Other
        };
    }
    let trimmed = text.trim_start();
    if trimmed.starts_with(INTERRUPT_PREFIX) {
        return Kind::Interrupt;
    }
    if trimmed.starts_with("<command-") {
        if let Some(name) = command_name(trimmed) {
            return Kind::Command(name);
        }
    }
    if entry["isCompactSummary"] == true
        || trimmed.is_empty()
        || NOT_TYPED.iter().any(|start| trimmed.starts_with(start))
    {
        return Kind::Other;
    }
    Kind::UserPrompt(typed(trimmed).to_owned())
}

/// A message the user sent while a turn ran. Claude Code queues task notifications and messages
/// from other agents (`isMeta`) the same way; those are no prompt.
fn queued_prompt(attachment: &Value) -> Option<Kind> {
    if attachment["type"] != "queued_command"
        || attachment["commandMode"] != "prompt"
        || attachment["isMeta"] == true
    {
        return None;
    }
    let text = text_of(&attachment["prompt"])?;
    let trimmed = text.trim_start();
    (!trimmed.is_empty() && !NOT_TYPED.iter().any(|start| trimmed.starts_with(start)))
        .then(|| Kind::UserPrompt(typed(trimmed).to_owned()))
}

/// A prompt without the `<system-reminder>` blocks Claude Code puts in front of it. Empty when
/// what is left is a CI monitor event.
/// ponytail: two known wrappers; add a prefix here when real transcripts show another.
fn typed(prompt: &str) -> &str {
    let mut rest = prompt;
    while let Some(after) = rest.strip_prefix("<system-reminder>") {
        rest = after
            .split_once("</system-reminder>")
            .map_or("", |(_, after)| after)
            .trim_start();
    }
    if rest.starts_with("<ci-monitor-event") {
        ""
    } else {
        rest.trim_end()
    }
}

/// Claude Code writes each content block of an assistant message as its own entry.
fn tool_use(content: &Value) -> Option<Kind> {
    let block = content
        .as_array()?
        .iter()
        .find(|b| b["type"] == "tool_use")?;
    let id = block["id"].as_str()?.to_owned();
    let name = block["name"].as_str()?.to_owned();
    let input = &block["input"];
    Some(if name == "Skill" {
        Kind::SkillTool {
            id,
            skill: input["skill"].as_str()?.to_owned(),
        }
    } else {
        Kind::ToolUse {
            id,
            name,
            input: input.clone(),
        }
    })
}

/// The text blocks of an assistant entry; thinking is no text.
fn agent_text(content: &Value) -> Option<Kind> {
    let text: Vec<&str> = content
        .as_array()?
        .iter()
        .filter(|b| b["type"] == "text")
        .filter_map(|b| b["text"].as_str())
        .collect();
    let text = text.join("\n");
    (!text.trim().is_empty()).then_some(Kind::AgentText(text))
}

/// Claude Code writes each tool result as its own user entry.
fn tool_result(entry: &Value, content: &Value) -> Option<Kind> {
    let block = content
        .as_array()?
        .iter()
        .find(|b| b["type"] == "tool_result")?;
    let denial = string(&entry["toolDenialKind"]);
    let text = match &block["content"] {
        Value::String(text) => text.clone(),
        blocks => text_of(blocks).unwrap_or_default(),
    };
    let denial_reason = denial
        .as_ref()
        .and_then(|_| text.split_once(DENIAL_REASON_MARKER))
        .map(|(_, reason)| reason.trim())
        .filter(|reason| !reason.is_empty() && !reason.starts_with(CLARIFY_PREFIX))
        .map(str::to_owned);
    Some(Kind::ToolResult(ToolResult {
        tool_use_id: block["tool_use_id"].as_str()?.to_owned(),
        error: denial.is_none() && block["is_error"] == true,
        denial,
        denial_reason,
        answers: answers(&entry["toolUseResult"]),
        pr_urls: pr_urls(&text),
    }))
}

/// `AskUserQuestion` writes its questions with their options and the answers by question text.
fn answers(result: &Value) -> Vec<Answer> {
    let (Some(questions), Some(answers)) = (result["questions"].as_array(), result.get("answers"))
    else {
        return Vec::new();
    };
    questions
        .iter()
        .map(|question| {
            let labels: Vec<&str> = question["options"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|option| option["label"].as_str())
                .collect();
            let (picked, own_words) = match &answers[question["question"].as_str().unwrap_or("")] {
                Value::String(text) if text.starts_with(DISMISSED_PREFIX) => {
                    return Answer::Skipped
                }
                Value::String(text) if question["multiSelect"] == true => {
                    joined_labels(text, &labels)
                }
                Value::String(text) => (
                    vec![text.as_str()],
                    (!labels.contains(&text.as_str())).then(|| text.clone()),
                ),
                Value::Array(texts) => {
                    let picked: Vec<&str> = texts.iter().filter_map(Value::as_str).collect();
                    let own_words: Vec<&str> = picked
                        .iter()
                        .copied()
                        .filter(|p| !labels.contains(p))
                        .collect();
                    (
                        picked,
                        (!own_words.is_empty()).then(|| own_words.join(", ")),
                    )
                }
                _ => return Answer::Skipped,
            };
            if let Some(own_words) = own_words {
                Answer::OwnWords(own_words)
            } else if labels.first().is_some_and(|first| picked == [*first]) {
                Answer::Recommended
            } else {
                Answer::Options
            }
        })
        .collect()
}

/// The labels in a multi-select answer that arrived as one string, joined with ", ", and the text
/// left that is no label, if any. Labels can hold ", " themselves, so whole labels are taken out,
/// longest first, and only where they fill a part: "No" is no label inside "Not now".
fn joined_labels<'a>(text: &str, labels: &[&'a str]) -> (Vec<&'a str>, Option<String>) {
    let mut rest = text.to_owned();
    let mut by_length = labels.to_vec();
    by_length.sort_by_key(|label| std::cmp::Reverse(label.len()));
    let picked = by_length
        .into_iter()
        .filter(|label| {
            let whole_part = |at: usize| {
                let before = rest[..at].trim_end();
                let after = rest[at + label.len()..].trim_start();
                (before.is_empty() || before.ends_with(','))
                    && (after.is_empty() || after.starts_with(','))
            };
            let found = rest
                .match_indices(*label)
                .map(|(at, _)| at)
                .find(|&at| whole_part(at));
            match found.filter(|_| !label.is_empty()) {
                Some(at) => {
                    rest.replace_range(at..at + label.len(), "");
                    true
                }
                None => false,
            }
        })
        .collect();
    let own_words: Vec<&str> = rest
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect();
    (
        picked,
        (!own_words.is_empty()).then(|| own_words.join(", ")),
    )
}

fn pr_urls(text: &str) -> Vec<String> {
    let name = |part: &str| {
        !part.is_empty()
            && part
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || "-_.".contains(c))
    };
    let mut urls: Vec<String> = Vec::new();
    for (at, _) in text.match_indices(PR_URL_PREFIX) {
        let mut parts = text[at + PR_URL_PREFIX.len()..].splitn(4, '/');
        let (Some(owner), Some(repo), Some("pull"), Some(rest)) =
            (parts.next(), parts.next(), parts.next(), parts.next())
        else {
            continue;
        };
        let number: String = rest.chars().take_while(char::is_ascii_digit).collect();
        if name(owner) && name(repo) && !number.is_empty() {
            urls.push(format!("{PR_URL_PREFIX}{owner}/{repo}/pull/{number}"));
        }
    }
    urls
}

/// The text of a message, or `None` when it carries a tool result.
fn text_of(content: &Value) -> Option<String> {
    match content {
        Value::String(text) => Some(text.clone()),
        Value::Array(blocks) if blocks.iter().all(|b| b["type"] != "tool_result") => Some(
            blocks
                .iter()
                .filter_map(|b| b["text"].as_str())
                .collect::<Vec<_>>()
                .join("\n"),
        ),
        _ => None,
    }
}

fn command_name(text: &str) -> Option<String> {
    let (_, rest) = text.split_once("<command-name>/")?;
    let (name, _) = rest.split_once("</command-name>")?;
    Some(name.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pull_request_urls_are_cut_after_the_number() {
        assert_eq!(
            pr_urls("https://github.com/o/r.s/pull/7/files, see https://github.com/o/r/pull/8."),
            [
                "https://github.com/o/r.s/pull/7",
                "https://github.com/o/r/pull/8"
            ]
        );
        assert!(
            pr_urls("https://github.com/o/r/issues/7 https://github.com/o/r/pull/x").is_empty()
        );
        assert!(pr_urls("https://github.com/o r/x/pull/7").is_empty());
    }

    #[test]
    fn the_typed_text_of_a_prompt_drops_what_claude_code_put_there() {
        let reminder = "<system-reminder>\nTask started.\n</system-reminder>\n";
        assert_eq!(
            typed(&format!("{reminder}{reminder} open a PR\n")),
            "open a PR"
        );
        assert_eq!(
            typed(&format!(
                "{reminder}<ci-monitor-event>red</ci-monitor-event>"
            )),
            ""
        );
        assert_eq!(typed("<system-reminder> never closed"), "");
        assert_eq!(
            typed("keep <system-reminder> in the middle"),
            "keep <system-reminder> in the middle"
        );
    }

    #[test]
    fn a_multi_select_answer_can_be_an_array() {
        let result = serde_json::json!({
            "questions": [{"question": "Q", "multiSelect": true,
                           "options": [{"label": "A"}, {"label": "B"}]}],
            "answers": {"Q": ["A"]}
        });
        assert_eq!(answers(&result), [Answer::Recommended]);
    }

    #[test]
    fn whitespace_typed_as_an_answer_still_counts_as_own_words() {
        let result = serde_json::json!({
            "questions": [{"question": "Q", "options": [{"label": "A"}]},
                          {"question": "R", "multiSelect": true, "options": [{"label": "A"}]}],
            "answers": {"Q": " ", "R": ["A", " "]}
        });
        assert_eq!(
            answers(&result),
            [Answer::OwnWords(" ".into()), Answer::OwnWords(" ".into())]
        );
    }

    #[test]
    fn a_dismissed_question_is_skipped() {
        let result = serde_json::json!({
            "questions": [{"question": "Q", "options": [{"label": "A"}]}],
            "answers": {"Q": "[User dismissed — do not proceed, wait for next instruction]"}
        });
        assert_eq!(answers(&result), [Answer::Skipped]);
    }

    #[test]
    fn a_joined_multi_select_answer_keeps_labels_with_commas_whole() {
        let labels = ["Red, green", "Blue", "Red"];
        assert_eq!(
            joined_labels("Red, green", &labels),
            (vec!["Red, green"], None)
        );
        assert_eq!(
            joined_labels("Blue, Red, green", &labels),
            (vec!["Red, green", "Blue"], None)
        );
        assert_eq!(
            joined_labels("Blue, and more, Red", &labels),
            (vec!["Blue", "Red"], Some("and more".into()))
        );
        assert_eq!(
            joined_labels("Yes, Not now", &["Yes", "No"]),
            (vec!["Yes"], Some("Not now".into()))
        );
    }
}
