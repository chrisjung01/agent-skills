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

**Choice question**:
A question to the user written as reply text, one per message: what is decided and the facts it rests on with their sources. A confirm or approval question has the numbered options `1) Yes` and `2) No`; real alternatives get `1)` `2)` `3)`, each with its meaning and one consequence, and a recommendation that "yes" accepts. Never asked through the harness's picker.
_Avoid_: Picker question, Decision question

**Question series**:
The choice questions one run asks the user one after another. Its size is the questions already asked plus every open question known so far, those still waiting on an answer and the closing confirm included; a single question is no series.
_Avoid_: Question round, Batch (questions stay one per message)

**Bare question**:
A question the agent asked through the picker with no visible text since the last tool call in the same turn; thinking does not count as visible. Counted in older sessions only.
_Avoid_: Empty question, Question without context

**User text**:
Text the user wrote in a window, stored redacted with its kind: `prompt` (what the user typed), `denial reason` (what the user said when rejecting a tool call) or `own-word answer` (the words of a question answer that are no option). A message the user queued while a turn ran is a prompt. Each user text carries a `correction` flag and a `strong` flag from the old hook's patterns; a flag marks text to read, not a verdict. Never agent text, tool output or skill text.
_Avoid_: Quote, Transcript text
_Code_: `UserText`

**Rate**:
The share of skill calls with at least one event of a kind (correction, interrupt, denial, tool error, bare question), shown with its 95 % Wilson interval. A call whose parser did not measure the kind stays out of the denominator.
_Avoid_: Count, Frequency (events per call; events inside one call are not independent)

## Spec-driven flow

**Fix**:
A change that restores already intended behaviour. A newly introduced rule is a feature, even when the issue calls it a fix.
_Avoid_: Bugfix (for a new rule)

**Acceptance**:
The user's manual check of a whole spec, every acceptance criterion and edge-case row with the user's observation, run once on the last open PR of the spec before it merges. Posted as an `## Acceptance` comment on the spec's issue. A spec whose Acceptance line says verified is enough gets no check by hand; its `## Acceptance` comment only records the kill issue and deviations.
_Avoid_: Verification (the agent's evidence per PR), Sign-off, Playtest

**Acceptance line**:
The last `Acceptance:` line of a spec, `Acceptance: by hand` or `Acceptance: verified is enough`, with the trigger that set it. It decides in every tier whether Acceptance checks the spec by hand.
_Avoid_: Acceptance level, Sign-off flag

**Verification**:
The agent's evidence per PR that each acceptance criterion and edge-case row of its ticket is met, from a test, a check or a manual observation; it becomes the Verified block of the PR body.
_Avoid_: Acceptance (the user's check of the whole spec)

**Tier**:
The amount of flow a change runs, picked from its stakes: small, normal, or big or risky. The README names each tier's signs and commands; the grill proposes one and the user confirms it, and `cj-implement` asks once for a stake on an issue without a spec. A confirmed tier stands on the issue as a `Tier: <tier>, confirmed` line; the latest one counts.
_Avoid_: Lane, Size class, Track

**Coverage table**:
The table `cj-grilling` shows before it closes a session: one row per category of a plan, each rated Clear, Partial, Missing or Out of scope.
_Avoid_: Clarify gate (a mandatory step, which this is not), Checklist

**Text others read**:
Text a skill writes for someone who is not the user in this session: a PR body, an issue, a spec or review comment, a review reply, a team-chat message, a docs page. It leaves the session through the `cj-posting` gate. Its opposite is chat text, the reply the user reads in the terminal.
_Avoid_: Outbound text, Text for others

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

## Scenarios

**Scenario**:
A paper exercise for one skill, built from a real session: setup, prompt and expected behaviour, run by a fresh agent against the skill text before and after a change. A skill's scenarios sit in `docs/tests/<skill>-scenarios.md`, one per heading; the rules are in `docs/tests/README.md`.
_Avoid_: Test case (nothing runs it automatically), Eval, Scenario file (the file holds several)

**Results line**:
The record of a scenario's runs under Results: "met expected" or "missed expected" for the skill text before and after the change, each naming the commit the run read. A regression guard's line calls it one.
_Avoid_: Outcome (one half only), Result section (one line can cover several cases)
