//! Registers whetstone's SessionEnd hook in Codex's `hooks.json`, beside the hooks already there.

use serde_json::{json, Value};

/// Appends a SessionEnd group running `command`. Codex keeps its trust per hook position, so the
/// hooks already there keep theirs. False when a handler already runs `command`.
/// ponytail: matched by the exact command, so a second checkout adds its own hook; both collect
/// into the same rows.
pub fn add(config: &mut Value, command: &str) -> Result<bool, String> {
    check(config)?;
    let groups = config
        .as_object_mut()
        .unwrap()
        .entry("hooks")
        .or_insert(json!({}))
        .as_object_mut()
        .unwrap()
        .entry("SessionEnd")
        .or_insert(json!([]))
        .as_array_mut()
        .unwrap();
    if groups.iter().any(|group| runs(group, command)) {
        return Ok(false);
    }
    groups.push(json!({"hooks": [{"type": "command", "command": command}]}));
    Ok(true)
}

/// Removes every SessionEnd handler running `command`, and a group or list that leaves empty.
/// ponytail: groups after a removed one move up a position, so Codex asks to trust them again.
pub fn remove(config: &mut Value, command: &str) -> Result<bool, String> {
    check(config)?;
    let Some(groups) = config
        .pointer_mut("/hooks/SessionEnd")
        .and_then(Value::as_array_mut)
    else {
        return Ok(false);
    };
    let mut removed = false;
    groups.retain_mut(|group| {
        let Some(handlers) = group.get_mut("hooks").and_then(Value::as_array_mut) else {
            return true;
        };
        let before = handlers.len();
        handlers.retain(|handler| handler["command"] != command);
        removed |= handlers.len() < before;
        handlers.len() == before || !handlers.is_empty()
    });
    if removed && groups.is_empty() {
        config["hooks"]
            .as_object_mut()
            .unwrap()
            .remove("SessionEnd");
    }
    Ok(removed)
}

fn runs(group: &Value, command: &str) -> bool {
    group["hooks"]
        .as_array()
        .is_some_and(|handlers| handlers.iter().any(|handler| handler["command"] == command))
}

/// The shape both edits rely on; anything else is the user's to fix.
fn check(config: &Value) -> Result<(), String> {
    if !config.is_object() {
        return Err("is not a JSON object".into());
    }
    // An explicit null counts as malformed: the edits would have to replace it.
    let hooks = match config.get("hooks") {
        None => return Ok(()),
        Some(hooks @ Value::Object(_)) => hooks,
        Some(_) => return Err("has a \"hooks\" that is not an object".into()),
    };
    match hooks.get("SessionEnd") {
        None | Some(Value::Array(_)) => Ok(()),
        Some(_) => Err("has a \"SessionEnd\" that is not a list".into()),
    }
}
