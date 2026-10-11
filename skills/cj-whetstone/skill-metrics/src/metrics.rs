//! What happened in each window: tokens, models, active duration, where Claude Code ran, tools,
//! denials, questions, pull requests and the user's own text.

use crate::question::{is_bare, CHOICE_TOOL};
use crate::transcript::{Answer, Context, Event, Kind, Tokens, ToolResult};
use crate::user_text::{UserText, DENIAL_REASON, OWN_WORD_ANSWER, PROMPT};
use crate::window::SkillCall;
use std::collections::{BTreeMap, HashMap, HashSet};

/// Longest gap between two entries that counts toward the active duration.
const IDLE_CAP_MS: i64 = 5 * 60 * 1000;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Metrics {
    /// Main-agent requests in the window, each counted once. This is what the transcript holds;
    /// the session's `cost-state` also counts requests Claude Code never writes there (side calls
    /// such as Haiku's), so it can be higher.
    pub tokens: Tokens,
    /// Models of the window's requests, in order of first use. A request whose message began in
    /// the previous window lists its model here but counts its tokens there.
    pub models: Vec<String>,
    /// Gaps between entries summed, each capped at `IDLE_CAP_MS`.
    pub active_ms: i64,
    /// Of the entry that starts the skill call.
    pub context: Context,
    /// Tool calls in the window, the `Skill` call that starts it included.
    pub tool_calls: i64,
    /// Tool results in the window that report a failure. Denials are not among them.
    pub tool_errors: i64,
    /// Count by `toolDenialKind`.
    pub denials: BTreeMap<String, i64>,
    /// Interrupt markers only; a `cancelled` denial is a denial.
    pub interrupts: i64,
    /// Questions of answered `AskUserQuestion` calls.
    pub questions: i64,
    pub recommended_answers: i64,
    /// Choice tool calls, however many questions each holds, and those of them that were bare
    /// (the hook blocked them, or no agent text came since the last tool call).
    pub choice_questions: i64,
    pub bare_questions: i64,
    pub own_word_answers: i64,
    pub subagents: i64,
    /// Shell calls that ran `git commit`, `git push` or `gh pr create` without a tool error.
    pub commits: i64,
    pub pushes: i64,
    pub prs_created: i64,
    /// Pull request URLs in the window's tool results, once each, in order of first appearance.
    pub pr_urls: Vec<String>,
    /// Prompts, denial reasons and own-word answers, in window order. Never agent text, tool
    /// output or skill text.
    pub user_texts: Vec<UserText>,
}

/// Fills the metrics of one session's skill calls, in window order, from the session's events.
pub fn measure(calls: &mut [SkillCall], events: &[Event]) {
    // A message can span two windows; its request counts in the first.
    let mut counted_requests: HashSet<&str> = HashSet::new();
    // A shell call counts where it was made, its result may come later.
    let results: HashMap<&str, &ToolResult> = events
        .iter()
        .filter_map(|e| match &e.kind {
            Kind::ToolResult(result) => Some((result.tool_use_id.as_str(), result)),
            _ => None,
        })
        .collect();
    // Only a choice tool's result can be a blocked question; other output may quote the reason.
    let choice_ids: HashSet<&str> = events
        .iter()
        .filter_map(|e| match &e.kind {
            Kind::ToolUse { id, name, .. } if name == CHOICE_TOOL => Some(id.as_str()),
            _ => None,
        })
        .collect();
    for call in calls {
        let (start, window) = (call.span.start, &events[call.span.clone()]);
        let metrics = &mut call.metrics;
        if let Some(start) = window.first() {
            metrics.context = start.context.clone();
        }
        // Latest entry time so far. An entry earlier than it adds nothing.
        let mut last_ms: Option<i64> = None;
        for (offset, event) in window.iter().enumerate() {
            if let Some(ms) = millis(&event.timestamp) {
                if let Some(last) = last_ms {
                    metrics.active_ms += (ms - last).clamp(0, IDLE_CAP_MS);
                }
                last_ms = last_ms.max(Some(ms));
            }
            count_event(metrics, &event.kind, &results, &choice_ids);
            if let Kind::ToolUse { id, name, .. } = &event.kind {
                if name == CHOICE_TOOL {
                    let blocked = results.get(id.as_str()).is_some_and(|r| r.blocked);
                    metrics.choice_questions += 1;
                    metrics.bare_questions +=
                        i64::from(blocked || is_bare(&events[..start + offset]));
                }
            }
            let Some(request) = &event.request else {
                continue;
            };
            if counted_requests.insert(&request.message_id) {
                metrics.tokens += request.tokens;
            }
            // Claude Code writes its own error messages as assistant entries of this model.
            if request.model != "<synthetic>" && !metrics.models.contains(&request.model) {
                metrics.models.push(request.model.clone());
            }
        }
    }
}

/// Tools, denials, interrupts, answers, pull requests and user text of one event.
fn count_event(
    metrics: &mut Metrics,
    kind: &Kind,
    results: &HashMap<&str, &ToolResult>,
    choice_ids: &HashSet<&str>,
) {
    match kind {
        Kind::SkillTool { .. } => metrics.tool_calls += 1,
        Kind::ToolUse { id, name, input } => {
            metrics.tool_calls += 1;
            // None while the result is still to come.
            let failed = results
                .get(id.as_str())
                .map(|r| r.denial.is_some() || r.error);
            // A subagent may still run when the transcript ends, so only a failure rules it out.
            if name == "Agent" && failed != Some(true) {
                metrics.subagents += 1;
            }
            let command = input["command"].as_str().filter(|_| name == "Bash");
            if let Some(command) = command.filter(|_| failed == Some(false)) {
                let actions = shell_actions(command);
                metrics.commits += i64::from(actions.commit);
                metrics.pushes += i64::from(actions.push);
                metrics.prs_created += i64::from(actions.pr_create);
            }
        }
        // A question the bare-question hook blocked counts as bare, not as a denial.
        Kind::ToolResult(result)
            if result.blocked && choice_ids.contains(result.tool_use_id.as_str()) => {}
        Kind::ToolResult(result) => {
            if let Some(denial) = &result.denial {
                *metrics.denials.entry(denial.clone()).or_default() += 1;
            }
            if let Some(reason) = &result.denial_reason {
                metrics
                    .user_texts
                    .extend(UserText::new(DENIAL_REASON, reason));
            }
            metrics.tool_errors += i64::from(result.error);
            for answer in &result.answers {
                metrics.questions += 1;
                match answer {
                    Answer::Recommended => metrics.recommended_answers += 1,
                    Answer::OwnWords(text) => {
                        metrics.own_word_answers += 1;
                        metrics
                            .user_texts
                            .extend(UserText::new(OWN_WORD_ANSWER, text));
                    }
                    Answer::Options | Answer::Skipped => {}
                }
            }
            for url in &result.pr_urls {
                if !metrics.pr_urls.contains(url) {
                    metrics.pr_urls.push(url.clone());
                }
            }
        }
        Kind::Interrupt => metrics.interrupts += 1,
        Kind::UserPrompt(text) => metrics.user_texts.extend(UserText::new(PROMPT, text)),
        _ => {}
    }
}

#[derive(Debug, Default, PartialEq)]
struct ShellActions {
    commit: bool,
    push: bool,
    pr_create: bool,
}

/// Whether a shell command runs `git commit`, `git push` and `gh pr create`. Only the start of a
/// command counts (after `&&`, `|`, `;`, a newline or a parenthesis, variable assignments
/// skipped), and heredoc bodies are text, so a commit message that mentions `git push` is no push.
/// ponytail: a separator inside quotes still splits, and `env git` or `bash -c` hide the command;
/// parse shell words if that ever miscounts.
fn shell_actions(command: &str) -> ShellActions {
    let mut found = ShellActions::default();
    for segment in without_heredocs(command).split(['&', '|', ';', '\n', '(', ')']) {
        let mut words = segment.split_whitespace().skip_while(|w| w.contains('='));
        match words.next() {
            Some("git") => {
                // Options before the subcommand; `-C` and `-c` take a value.
                while let Some(word) = words.next() {
                    if word == "-C" || word == "-c" {
                        words.next();
                    } else if !word.starts_with('-') {
                        found.commit |= word == "commit";
                        found.push |= word == "push";
                        break;
                    }
                }
            }
            Some("gh") => {
                found.pr_create |= words.next() == Some("pr") && words.next() == Some("create");
            }
            _ => {}
        }
    }
    found
}

/// The command's lines without heredoc bodies (`<<EOF`, `<<-'EOF'`, ... up to the line `EOF`).
fn without_heredocs(command: &str) -> String {
    let mut kept = String::new();
    let mut end: Option<String> = None;
    for line in command.lines() {
        if let Some(word) = &end {
            if line.trim() == word {
                end = None;
            }
            continue;
        }
        if let Some((_, after)) = line.split_once("<<") {
            let word: String = after
                .trim_start_matches('-')
                .trim_start()
                .trim_start_matches(['\'', '"'])
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            end = Some(word).filter(|w| !w.is_empty());
        }
        kept.push_str(line);
        kept.push('\n');
    }
    kept
}

/// Milliseconds since the Unix epoch of a UTC timestamp as Claude Code writes it
/// (`2026-10-01T10:00:00.000Z`). Anything else, an offset like `+02:00` included, is `None`.
fn millis(timestamp: &str) -> Option<i64> {
    if !timestamp.ends_with('Z') {
        return None;
    }
    let number = |range: std::ops::Range<usize>| timestamp.get(range)?.parse::<i64>().ok();
    let (year, month, day) = (number(0..4)?, number(5..7)?, number(8..10)?);
    let (hour, minute, second) = (number(11..13)?, number(14..16)?, number(17..19)?);
    let fraction = number(20..23).unwrap_or(0);
    // Days from civil, Howard Hinnant's algorithm.
    let y = if month <= 2 { year - 1 } else { year };
    let era = y.div_euclid(400);
    let year_of_era = y - era * 400;
    let day_of_year = (153 * ((month + 9) % 12) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    let days = era * 146_097 + day_of_era - 719_468;
    Some(((days * 24 + hour) * 60 + minute) * 60_000 + second * 1000 + fraction)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transcript::parse;
    use crate::window::skill_calls;

    const USAGE: &str = include_str!("../tests/fixtures/usage.jsonl");
    const TOOLS: &str = include_str!("../tests/fixtures/tools.jsonl");

    fn measured(jsonl: &str) -> Vec<Metrics> {
        let events = parse(jsonl);
        let mut calls = skill_calls("s3", &events, &[]);
        measure(&mut calls, &events);
        calls.into_iter().map(|c| c.metrics).collect()
    }

    #[test]
    fn requests_count_once_and_idle_gaps_are_capped() {
        let metrics = measured(USAGE);
        let (implement, triage) = (&metrics[0], &metrics[1]);
        // m1 is written twice; m4 spans both windows and counts in the first; <synthetic> is no model.
        assert_eq!(
            implement.tokens,
            Tokens {
                input: 16,
                output: 27,
                cache_read: 108,
                cache_write: 14
            }
        );
        assert_eq!(implement.models, ["claude-opus-5-5", "claude-sonnet-5-5"]);
        // 1 min, a 40-minute gap counted as 5, then 30 s, 30 s and 1 min.
        assert_eq!(implement.active_ms, 8 * 60 * 1000);
        assert_eq!(
            triage.tokens,
            Tokens {
                input: 2,
                output: 2,
                cache_read: 2,
                cache_write: 2
            }
        );
        assert_eq!(triage.models, ["claude-opus-5-5"]);
        assert_eq!(triage.active_ms, 60 * 1000);
    }

    #[test]
    fn early_and_unreadable_timestamps_add_no_time() {
        // p07 runs an hour behind; the gaps around it count from 10:41:30 to 10:43 as before.
        let early = USAGE.replace(
            "\"uuid\":\"p07\",\"parentUuid\":\"p06\",\"timestamp\":\"2026-10-03T10:42:00.000Z\"",
            "\"uuid\":\"p07\",\"parentUuid\":\"p06\",\"timestamp\":\"2026-10-03T09:42:00.000Z\"",
        );
        assert_ne!(early, USAGE);
        assert_eq!(measured(&early)[0].active_ms, 8 * 60 * 1000);
        // The starting entry's time does not parse; counting starts at the next entry.
        let unreadable = USAGE.replace(
            "\"uuid\":\"p01\",\"parentUuid\":null,\"timestamp\":\"2026-10-03T10:00:00.000Z\"",
            "\"uuid\":\"p01\",\"parentUuid\":null,\"timestamp\":\"10 o'clock\"",
        );
        assert_ne!(unreadable, USAGE);
        assert_eq!(measured(&unreadable)[0].active_ms, 8 * 60 * 1000);
    }

    #[test]
    fn the_context_comes_from_the_starting_entry() {
        let metrics = measured(USAGE);
        assert_eq!(metrics[1].context.git_branch.as_deref(), Some("feature"));
        assert_eq!(
            metrics[0].context,
            Context {
                version: Some("2.1.290".into()),
                entrypoint: Some("cli".into()),
                cwd: Some("/home/x/repo".into()),
                git_branch: Some("main".into()),
            }
        );
    }

    #[test]
    fn tools_denials_questions_and_prs_are_counted() {
        let metrics = measured(TOOLS);
        let (implement, triage) = (&metrics[0], &metrics[1]);
        let tools = |m: &Metrics| {
            (
                m.tool_calls,
                m.tool_errors,
                m.interrupts,
                m.subagents,
                m.commits,
                m.pushes,
                m.prs_created,
            )
        };
        // The rejected push is a tool error; the amend never got a result, so it did not run.
        assert_eq!(tools(implement), (10, 1, 1, 1, 1, 1, 1));
        // Denials are no tool errors, and the cancelled one is no interrupt.
        assert_eq!(
            implement.denials.iter().collect::<Vec<_>>(),
            [
                (&"cancelled".to_owned(), &1),
                (&"permission-rule".to_owned(), &1),
                (&"user-rejected".to_owned(), &1)
            ]
        );
        // Four questions: the recommended option, own words, two of three options, no answer.
        assert_eq!(
            (
                implement.questions,
                implement.recommended_answers,
                implement.own_word_answers
            ),
            (4, 1, 1)
        );
        assert_eq!(implement.pr_urls, ["https://github.com/o/r/pull/7"]);
        // The Skill tool call that starts the window counts in it.
        assert_eq!(tools(triage), (3, 1, 0, 0, 0, 0, 0));
        assert!(triage.denials.is_empty());
        assert_eq!(
            triage.pr_urls,
            [
                "https://github.com/o/r/pull/8",
                "https://github.com/o/r/pull/7"
            ]
        );
    }

    #[test]
    fn a_failed_agent_call_starts_no_subagent() {
        let failed = TOOLS.replace(
            "\"tool_use_id\":\"a1\",",
            "\"tool_use_id\":\"a1\",\"is_error\":true,",
        );
        assert_ne!(failed, TOOLS);
        assert_eq!(measured(&failed)[0].subagents, 0);
        // Still running when the transcript ends: no result yet.
        let running: String = TOOLS
            .lines()
            .filter(|l| !l.contains("\"uuid\":\"x22\""))
            .map(|l| format!("{l}\n"))
            .collect();
        assert_eq!(measured(&running)[0].subagents, 1);
    }

    #[test]
    fn only_a_command_start_counts_as_git_or_gh() {
        let actions = |command| {
            let a = shell_actions(command);
            (a.commit, a.push, a.pr_create)
        };
        assert_eq!(actions("git commit -m x"), (true, false, false));
        assert_eq!(
            actions("cd r && git -C /r -c a=b push; gh pr create --draft"),
            (false, true, true)
        );
        assert_eq!(
            actions("GIT_EDITOR=: git commit --amend\ngit push"),
            (true, true, false)
        );
        assert_eq!(
            actions("git commit -m \"say git push and gh pr create\""),
            (true, false, false)
        );
        assert_eq!(actions("echo git push"), (false, false, false));
        assert_eq!(actions("gh pr view 7"), (false, false, false));
        assert_eq!(actions("git log --format=%s"), (false, false, false));
        // Heredoc bodies are text: a plan file that lists commands, a commit message.
        assert_eq!(
            actions("cat > plan.md <<'PLAN_EOF'\ngit commit -m x\ngit push\nPLAN_EOF\nls"),
            (false, false, false)
        );
        assert_eq!(
            actions("git commit -F - <<-EOF\n\tgh pr create next\n\tEOF\ngit push"),
            (true, true, false)
        );
        assert_eq!(actions("grep x <<< \"git push\""), (false, false, false));
    }

    #[test]
    fn timestamps_become_epoch_milliseconds() {
        assert_eq!(millis("1970-01-01T00:00:00.000Z"), Some(0));
        assert_eq!(millis("2000-03-01T00:00:00.000Z"), Some(951_868_800_000));
        assert_eq!(millis("2026-10-08T18:40:02.697Z"), Some(1_791_484_802_697));
        // Across a leap day and a year end.
        let day = 86_400_000;
        let between = |a: &str, b: &str| millis(b).unwrap() - millis(a).unwrap();
        assert_eq!(
            between("2024-02-28T00:00:00.000Z", "2024-03-01T00:00:00.000Z"),
            2 * day
        );
        assert_eq!(
            between("2026-12-31T23:59:59.500Z", "2027-01-01T00:00:00.000Z"),
            500
        );
        assert_eq!(millis("not a time"), None);
        assert_eq!(millis("2026-10-08T18:40:02.697+02:00"), None);
    }
}
