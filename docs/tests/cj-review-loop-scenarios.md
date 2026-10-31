# cj-review-loop scenarios

Skill under test: `cj-review-loop` with `cj-reviewing` (issue #184). Fixture: a small repo with a branch `feat/x` one commit ahead of `main`; the diff has one medium finding on the first round and none on the second. The orchestrator is Sonnet 5.5 with effort medium unless a case says otherwise. Each case: the agent writes the arguments it passes to `cj-reviewing`, the question it asks if any, and the final report.

## 1. No argument (AC-1)

**Prompt:** "/cj-review-loop"
**Expected:** base is `main`; no cost question; the reviewer of every round runs on `claude-sonnet-5-5`, effort medium. The report holds `Reviewer and verifiers: claude-sonnet-5-5, effort medium`.

## 2. `high` (AC-3)

**Prompt:** "/cj-review-loop high"
**Expected:** no question; the reviewer of each round (round 2 included) on Opus with effort high; base is `main`, not `high`. The report line says Opus, effort high and names no `/cj-review-loop high` sentence.

## 3. `high` on Opus with effort xhigh (AC-4)

**Setup:** the orchestrator is Opus 5.5 with effort xhigh, named in its system context as the newest Opus.
**Prompt:** "/cj-review-loop high"
**Expected:** no question; every round's reviewer on `claude-opus-5-5`, effort xhigh, never high.

## 4. Report after a run without `high` (AC-6, AC-8)

**Prompt:** "/cj-review-loop develop"
**Expected:** base `develop`; the final report holds the `Reviewer and verifiers:` line and ends with one sentence naming `/cj-review-loop develop high`. The escalation report would name none.

## 5. Argument order and base (AC-9)

**Prompt:** "/cj-review-loop high develop" and then "/cj-review-loop develop high".
**Expected:** both read `high` as the model argument and `develop` as the base.

## 6. A base branch named `high` (edge case)

**Prompt:** "/cj-review-loop high" where the only other branch is called `high`.
**Expected:** `high` is read as the model argument; the base falls back to the default branch.

## Results
