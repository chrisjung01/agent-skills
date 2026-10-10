//! The events of one session into skill calls, each with its window and nested skills.

use crate::metrics::Metrics;
use crate::transcript::{Event, Kind};
use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct SkillCall {
    /// Uuid of the message that starts the skill call; the row key.
    pub start_uuid: String,
    pub skill: String,
    /// `slash` when the user typed the slash command, `model` for a `Skill` tool call.
    pub start_kind: &'static str,
    pub session_id: String,
    pub started_at: String,
    pub ended_at: String,
    pub user_prompts: i64,
    /// The skill text Claude Code loaded.
    pub text: String,
    /// Filled by `version::Versions::fill`.
    pub skill_version: Option<String>,
    /// In load order.
    pub nested: Vec<NestedSkill>,
    /// The window's events, as indices into the session's events.
    pub span: Range<usize>,
    /// Filled by `metrics::measure`.
    pub metrics: Metrics,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NestedSkill {
    pub skill: String,
    pub loaded_at: String,
    pub text: String,
    pub skill_version: Option<String>,
}

/// A skill slash command starts a skill call. A `Skill` tool call is a nested skill when a skill
/// text loaded in the window names it, and starts a skill call otherwise. Both count only when
/// Claude Code loaded skill text for them; built-in commands and forked skills load none.
/// The window runs until the next skill call, `/clear` or the end of the events.
///
/// Every skill a subagent loads is nested in the window its subagent started in.
pub fn skill_calls(session_id: &str, events: &[Event], subagents: &[Vec<Event>]) -> Vec<SkillCall> {
    let texts = loaded_texts(events);
    let mut calls: Vec<SkillCall> = Vec::new();
    // Loaded skill texts of the current window; empty outside a window.
    let mut window_texts: Vec<&str> = Vec::new();
    for (index, event) in events.iter().enumerate() {
        let load = match &event.kind {
            Kind::Command(name) => texts.get(event.uuid.as_str()).map(|t| (name, *t, true)),
            Kind::SkillTool { id, skill } => texts.get(id.as_str()).map(|t| (skill, *t, false)),
            _ => None,
        };
        if let Some((skill, text, by_user)) = load {
            let named = !by_user && window_texts.iter().any(|t| names(body(t), skill));
            if let Some(call) = calls.last_mut().filter(|_| named) {
                call.nested.push(NestedSkill {
                    skill: skill.clone(),
                    loaded_at: event.timestamp.clone(),
                    text: text.to_owned(),
                    skill_version: None,
                });
                window_texts.push(text);
            } else {
                calls.push(SkillCall {
                    start_uuid: event.uuid.clone(),
                    skill: skill.clone(),
                    start_kind: if by_user { "slash" } else { "model" },
                    session_id: session_id.to_owned(),
                    started_at: event.timestamp.clone(),
                    ended_at: event.timestamp.clone(),
                    user_prompts: 0,
                    text: text.to_owned(),
                    skill_version: None,
                    nested: Vec::new(),
                    span: index..index,
                    metrics: Metrics::default(),
                });
                window_texts = vec![text];
            }
        }
        if matches!(&event.kind, Kind::Command(name) if name == "clear") {
            window_texts.clear();
        }
        let Some(call) = calls.last_mut().filter(|_| !window_texts.is_empty()) else {
            continue;
        };
        call.span.end = index + 1;
        // Entries are not always in timestamp order, so the window ends at the latest one.
        if event.timestamp > call.ended_at {
            call.ended_at = event.timestamp.clone();
        }
        if matches!(event.kind, Kind::UserPrompt(_)) {
            call.user_prompts += 1;
        }
    }
    for subagent in subagents {
        nest_subagent_skills(&mut calls, subagent);
    }
    calls
}

/// Skill text by what loaded it: the slash command's uuid or the `Skill` tool call's id.
fn loaded_texts(events: &[Event]) -> HashMap<&str, &str> {
    events
        .iter()
        .filter_map(|e| match &e.kind {
            Kind::SkillText { text, tool_use_id } => {
                let loader = tool_use_id.as_deref().or(e.parent_uuid.as_deref())?;
                Some((loader, text.as_str()))
            }
            _ => None,
        })
        .collect()
}

fn nest_subagent_skills(calls: &mut [SkillCall], subagent: &[Event]) {
    let Some(first) = subagent.iter().map(|e| &e.timestamp).min() else {
        return;
    };
    let Some(call) = calls
        .iter_mut()
        .rfind(|c| c.started_at <= *first && *first <= c.ended_at)
    else {
        return;
    };
    let texts = loaded_texts(subagent);
    let before = call.nested.len();
    for event in subagent {
        if let Kind::SkillTool { id, skill } = &event.kind {
            if let Some(text) = texts.get(id.as_str()) {
                call.nested.push(NestedSkill {
                    skill: skill.clone(),
                    loaded_at: event.timestamp.clone(),
                    text: (*text).to_owned(),
                    skill_version: None,
                });
            }
        }
    }
    if call.nested.len() > before {
        call.nested.sort_by(|a, b| a.loaded_at.cmp(&b.loaded_at));
    }
}

/// Skill text without its first line, which holds the skill's own folder path.
pub fn body(text: &str) -> &str {
    text.split_once('\n').map_or("", |(_, body)| body)
}

/// Whether `text` names the skill as a whole name: `cj-review` does not match inside
/// `cj-review-loop` or `x:cj-review`.
fn names(text: &str, skill: &str) -> bool {
    let part_of_name = |c: char| c.is_alphanumeric() || c == '-' || c == '_';
    text.match_indices(skill).any(|(at, _)| {
        let before = text[..at].chars().next_back();
        let after = text[at + skill.len()..].chars().next();
        !before.is_some_and(|c| part_of_name(c) || c == ':') && !after.is_some_and(part_of_name)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transcript::{parse, parse_subagent};

    const SESSION: &str = include_str!("../tests/fixtures/session.jsonl");
    const NESTING: &str = include_str!("../tests/fixtures/nesting.jsonl");
    const SUBAGENT: &str = include_str!("../tests/fixtures/nesting-subagent.jsonl");

    fn summary(calls: &[SkillCall]) -> Vec<(&str, &str, &str, &str, i64)> {
        calls
            .iter()
            .map(|c| {
                (
                    c.skill.as_str(),
                    c.start_uuid.as_str(),
                    &c.started_at[11..19],
                    &c.ended_at[11..19],
                    c.user_prompts,
                )
            })
            .collect()
    }

    fn nested(call: &SkillCall) -> Vec<(&str, &str)> {
        call.nested
            .iter()
            .map(|n| (n.skill.as_str(), &n.loaded_at[11..19]))
            .collect()
    }

    #[test]
    fn slash_commands_with_skill_text_start_windows() {
        let calls = skill_calls("s1", &parse(SESSION), &[]);
        assert_eq!(
            summary(&calls),
            vec![
                // Ignores the unknown entry type, tool results, the interrupt, /model and its output.
                ("cj-review-loop", "u02", "10:01:00", "10:06:00", 2),
                // Ends at /clear.
                ("cj-ci-triage", "u12", "10:10:00", "10:12:00", 1),
                // Loaded by the model outside any window; the sidechain entry after it is not its own.
                ("unslop", "u18", "10:22:00", "10:22:00", 0),
            ]
        );
        assert!(calls.iter().all(|c| c.session_id == "s1"));
    }

    #[test]
    fn loaded_skill_text_is_kept() {
        let events = parse(SESSION);
        let text = events.iter().find_map(|e| match &e.kind {
            Kind::SkillText {
                text,
                tool_use_id: None,
            } => Some(text.as_str()),
            _ => None,
        });
        assert!(text
            .unwrap()
            .ends_with("Call the Skill tool with \"cj-reviewing\"."));
    }

    #[test]
    fn running_session_is_extracted_as_far_as_it_goes() {
        let lines: Vec<&str> = SESSION.lines().collect();
        let partial = format!("{}\n{}", lines[..14].join("\n"), &lines[14][..40]);
        let calls = skill_calls("s1", &parse(&partial), &[]);
        assert_eq!(
            summary(&calls)[1],
            ("cj-ci-triage", "u12", "10:10:00", "10:10:00", 0)
        );
    }

    #[test]
    fn skill_text_without_its_loader_starts_nothing() {
        let without_loaders: String = SESSION
            .lines()
            .filter(|l| !l.contains("<command-name>/cj-") && !l.contains("\"name\":\"Skill\""))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(skill_calls("s1", &parse(&without_loaders), &[]).is_empty());
    }

    #[test]
    fn skills_the_window_names_are_nested_and_others_start_a_call() {
        let calls = skill_calls("s2", &parse(NESTING), &[]);
        assert_eq!(
            summary(&calls),
            vec![
                // The prompts before and after the nested loads count here.
                ("cj-review-loop", "n01", "10:00:00", "10:05:00", 2),
                // Named by no loaded text, so it ends the review-loop window. code-review loaded no
                // text (forked skill) and is neither nested nor a call.
                ("cj-ci-triage", "n08a", "10:06:00", "10:08:00", 1),
            ]
        );
        assert_eq!(
            nested(&calls[0]),
            // unslop is named by cj-reviewing, which was nested earlier in the window.
            [
                ("cj-reviewing", "10:01:00"),
                ("cj-project-context", "10:03:00"),
                ("unslop", "10:04:00")
            ]
        );
        assert!(calls[1].nested.is_empty());
        // The loaded texts stay on the call and its nested skills, for the skill version.
        let folder = |text: &str| {
            text.lines()
                .next()
                .unwrap()
                .rsplit('/')
                .next()
                .unwrap()
                .to_owned()
        };
        assert_eq!(folder(&calls[0].text), "cj-review-loop");
        assert_eq!(folder(&calls[0].nested[2].text), "unslop");
    }

    #[test]
    fn skills_a_subagent_loads_nest_in_the_window_it_started_in() {
        let calls = skill_calls("s2", &parse(NESTING), &[parse_subagent(SUBAGENT)]);
        assert_eq!(
            nested(&calls[0]),
            [
                ("cj-reviewing", "10:01:00"),
                ("cj-filing-issues", "10:02:40"),
                ("cj-project-context", "10:03:00"),
                ("unslop", "10:04:00")
            ]
        );
        assert!(calls[0].nested[1].text.contains("/cj-filing-issues\n"));
        // A subagent's entries never start a call or count as the session's prompts.
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].user_prompts, 2);

        // A subagent that started outside every window nests nowhere.
        let before_all = parse_subagent(&SUBAGENT.replace("T10:02:", "T09:02:"));
        let calls = skill_calls("s2", &parse(NESTING), &[before_all]);
        assert_eq!(calls[0].nested.len(), 3);
        assert!(calls[1].nested.is_empty());
    }

    #[test]
    fn the_skill_text_path_does_not_name_its_own_skill() {
        // The model loads cj-review-loop again; only the path line of its text names it.
        let reload = NESTING
            .replace("\"skill\":\"cj-reviewing\"", "\"skill\":\"cj-review-loop\"")
            .replace("Notes in cj-review-loop/NOTES.md.", "");
        let calls = skill_calls("s2", &parse(&reload), &[]);
        assert_eq!(
            calls.iter().map(|c| c.skill.as_str()).collect::<Vec<_>>(),
            ["cj-review-loop", "cj-review-loop", "cj-ci-triage"]
        );
    }

    #[test]
    fn a_name_matches_only_as_a_whole_name() {
        assert!(names("Call \"cj-review\".", "cj-review"));
        assert!(names("cj-review", "cj-review"));
        assert!(!names("Read cj-review-loop/NOTES.md", "cj-review"));
        assert!(!names("superpowers:cj-review", "cj-review"));
        assert!(!names("my-cj-review", "cj-review"));
        assert!(names(
            "Use superpowers:writing-skills.",
            "superpowers:writing-skills"
        ));
        assert!(!names("Use writing-skills.", "superpowers:writing-skills"));
    }
}
