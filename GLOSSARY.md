# Agent skills

Terms for how skills run and how their use is measured from Claude Code and Codex transcripts.

## Skill metrics

**Skill call**:
One use of a skill, started by a user slash command (in Codex: a prompt with `$<skill>` that Codex attached the skill to) or by a model `Skill` tool call that is not a nested skill. Codex shows no activation for a skill the model reads on its own, so a Codex session holds only user-started calls. One row in the metrics database.
_Avoid_: Invocation, Run, Session (a session can hold several skill calls)
_Code_: `SkillCall`

**Start kind**:
How a skill call started: `slash` when the user typed the skill's slash command (or picked it with `$<skill>` in Codex), `model` when the model called the `Skill` tool outside any window whose loaded skill texts name the skill.
_Avoid_: Trigger

**Source**:
The tool whose transcript a skill call came from: `claude-code` or `codex`.
_Avoid_: Origin, Agent
_Code_: `SkillCall::source`

**Window**:
The part of a session that belongs to a skill call: from its start until the next skill call, `/clear`, or the end of the session.
_Avoid_: Span, Range

**Nested skill**:
A skill the model loads inside a window whose loaded skill texts already name it (for example `cj-reviewing` inside `cj-review-loop`, `unslop` inside `cj-posting`). A skill a subagent loads is a nested skill of the window the subagent started in. It is listed on the skill call and does not end the window.
_Avoid_: Child call, Sub-skill

**Active duration**:
The time of a window counted with every gap between two transcript entries capped at five minutes.
_Avoid_: Duration (the uncapped wall-clock time)

**Skill version**:
The git commit whose `SKILL.md` content matches the text loaded in the session, or `uncommitted` when no commit matches.
_Avoid_: Skill revision, `git log --before` commit

**Parser version**:
The version of the extraction logic that produced a row; rows from an older parser version are re-extracted.

**Denial**:
A tool call that did not run because the user or a permission rule rejected it, recorded with its kind.
_Avoid_: Tool error (a tool that ran and failed)

**Tool error**:
A tool call that ran and returned an error.
_Avoid_: Denial

**Bare question**:
A choice question the agent asks with no visible text since the last tool call in the same turn; thinking does not count as visible. A question the bare-question hook blocked is a bare question, not a denial.
_Avoid_: Empty question, Question without context

**User text**:
Text the user wrote in a window, stored redacted with its kind: `prompt` (what the user typed), `denial reason` (what the user said when rejecting a tool call) or `own-word answer` (the words of a question answer that are no option). A message the user queued while a turn ran is a prompt. Each user text carries a `correction` flag and a `strong` flag from the old hook's patterns; a flag marks text to read, not a verdict. Never agent text, tool output or skill text.
_Avoid_: Quote, Transcript text
_Code_: `UserText`

**Rate**:
The share of skill calls with at least one event of a kind (correction, interrupt, denial, tool error, bare question), shown with its 95 % Wilson interval. A call whose parser did not measure the kind stays out of the denominator.
_Avoid_: Count, Frequency (events per call; events inside one call are not independent)

## Spec-driven flow

**Acceptance**:
The user's manual check of a whole spec, every acceptance criterion and edge-case row with the user's observation, run once on the last open PR of the spec before it merges. Posted as an `## Acceptance` comment on the spec's issue.
_Avoid_: Verification (the agent's evidence per PR), Sign-off, Playtest

**Verification**:
The agent's evidence per PR that each acceptance criterion and edge-case row of its ticket is met, from a test, a check or a manual observation; it becomes the Verified block of the PR body.
_Avoid_: Acceptance (the user's check of the whole spec)

## Review

**Loosened check**:
A change in a diff that makes an existing check verify less: a test skipped, or deleted while the code it tests stays, an assertion removed, a suppression comment added (`@ts-ignore`, `eslint-disable`), a threshold lowered, an error swallowed by an empty `catch`. A tightened check needs no comment; a loosened one is a finding unless the spec, a deviation comment or the commit message gives its reason.
_Avoid_: Bar-lowering (metaphor), Suppression (one kind only), Weakened test (checks are not only tests)

**Static analysis**:
Answers about code taken from a tool instead of from reading it: who calls or imports a symbol, and which findings the project's linter already blocks in CI. Each answer names its source: a code index, the project's analyzer or `grep`. Which sources count as checked is defined in `cj-static-analysis`.
_Avoid_: Linting (one source only), Code search (`grep` only)

**Orphaned symbol**:
A symbol a diff adds outside test code, or whose last caller outside tests it removes, that still exists afterwards and that nothing outside tests calls or imports afterwards. A symbol that existed before the diff without such a caller is not one. Reported as a low finding from a checked source, as a question from an unchecked one, unless the review already reports it as an unwired entry point.
_Avoid_: Dead code (also covers code unused before the diff), Unused export (one kind only)
