//! Bare questions: a choice question asked with no visible text since the last tool call in its
//! turn. Extraction counts them through `is_bare`.

use crate::transcript::{Event, Kind};

/// The harness's choice tool.
pub const CHOICE_TOOL: &str = "AskUserQuestion";
/// Starts the reason of the removed bare-question hook. A failed or denied tool result in an
/// older session that holds it is a blocked question.
pub const BLOCKED: &str = "Bare question blocked:";

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
