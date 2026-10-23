# Scenarios: cj-reviewing

Paper exercises. The agent under test gets the skill text and the tool output below and describes its next steps; it runs nothing.

Shared setup: the orchestrating agent runs on Sonnet with medium effort. The caller is `cj-implement` step 6 and 7 for a PR of 279 changed lines in 9 files, touching no money, data or security path.

## 1. Model and effort of the subagents

**Setup:** `git diff origin/main...HEAD --shortstat` prints `9 files changed, 279 insertions(+), 29 deletions(-)`.
**Prompt:** "Review the diff before the PR opens."
**Expected:** before any dispatch it names the estimate (279 + 29 lines, 9 files, no risk touch) and asks one choice question with two numbered options: cheap (model and effort of the orchestrator) recommended, thorough (Opus, effort high) second, each with one pro and one con. It dispatches no subagent with a fixed model or effort until the user answers. After "1" (or "yes") the reviewer and both verifiers run with the orchestrator's model and effort. Before the change: the reviewer is dispatched on Opus with effort high and no question.

## 2. Risk touch

**Setup:** the diff changes the save format of the hero's silver.
**Expected:** the question names "touches data" as the reason to take the thorough option, and still recommends the cheap one.

## 3. Answer already carried

**Setup:** the caller passes "subagents: orchestrator model and effort, settled by the user".
**Expected:** no question; all subagents run with the orchestrator's model and effort.
