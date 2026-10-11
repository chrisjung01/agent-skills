//! Bare questions: a choice question asked with no visible text since the last tool call in its
//! turn. The PreToolUse hook blocks them and extraction counts them, both through `is_bare`.

use crate::transcript::{self, Event, Kind};
use serde_json::{json, Value};
use std::time::{Duration, Instant};
use std::{fs, thread};

/// The harness's choice tool.
pub const CHOICE_TOOL: &str = "AskUserQuestion";
/// Starts the hook's reason. A failed or denied tool result that holds it is a blocked question.
pub const BLOCKED: &str = "Bare question blocked:";
const REASON: &str = "Bare question blocked: the user sees no text before this question, \
     only the dialog; thinking is not shown. Write the list or the context the question refers \
     to as visible text first, then ask again.";
/// How long the hook waits for Claude Code to write the question's own entry.
const WAIT: Duration = Duration::from_secs(3);

/// Whether a choice question asked after `before` is bare: walking back, a tool call, a tool
/// result, a prompt or loaded skill text comes before any agent text. Thinking and text of
/// whitespace only are no agent text.
pub fn is_bare(before: &[Event]) -> bool {
    for event in before.iter().rev() {
        match event.kind {
            Kind::AgentText(_) => return false,
            Kind::Other => {}
            _ => return true,
        }
    }
    true
}

/// The hook's answer to a PreToolUse payload: the deny decision for a bare choice question, else
/// `None`. Anything it cannot judge passes, so a broken hook never stops the agent.
pub fn hook(payload: &str) -> Option<String> {
    let payload: Value = serde_json::from_str(payload).ok()?;
    // Subagents cannot ask the user, and their questions count in no window.
    if payload["tool_name"] != CHOICE_TOOL || payload["agent_id"].is_string() {
        return None;
    }
    let (path, id) = (
        payload["transcript_path"].as_str()?,
        payload["tool_use_id"].as_str()?,
    );
    // Claude Code writes the turn's entries a moment after the call starts.
    let deadline = Instant::now() + WAIT;
    loop {
        let events = transcript::parse(&String::from_utf8_lossy(&fs::read(path).ok()?));
        let asked = events
            .iter()
            .position(|e| matches!(&e.kind, Kind::ToolUse { id: i, .. } if i == id));
        if let Some(at) = asked {
            return is_bare(&events[..at]).then(|| {
                json!({"hookSpecificOutput": {
                    "hookEventName": "PreToolUse",
                    "permissionDecision": "deny",
                    "permissionDecisionReason": REASON,
                }})
                .to_string()
            });
        }
        if Instant::now() >= deadline {
            return None;
        }
        thread::sleep(Duration::from_millis(50));
    }
}
