# cj-reviewing scenarios

Fixture: a small Python repo with `AGENTS.md` (log through `shop.log`, never `print`) and one commit adding `discount(total)`. Planted defects: threshold `> 100` where the spec says "100 or more", no `ValueError` for negative totals, an unrequested `currency` parameter against the non-goal "No currency handling", and a `print`. Issue #12 holds the spec with three acceptance criteria. Runs on 2026-10-06 by fresh subagents.

## 1. Old cj-review-loop prompt, spec inside the repo (control)

Two runs. Both found all four defects: the reviewer read `docs/spec.md` and `AGENTS.md` on its own. No failure, so no fix needed for this case.

## 2. Old prompt, spec only in the issue (RED)

The old skill passes "context and goal, 1 to 3 sentences". One run:
- the threshold bug is missed; "100 gets no discount" appears as correct behaviour in a test suggestion
- the missing `ValueError` only shows as generic input validation, medium
- the `currency` parameter is low, with no link to the non-goal
- `print` found (AGENTS.md is in the repo)

## 3. cj-reviewing brief, spec pasted (GREEN, first wording)

All four found: criteria lines `1 wrong`, `2 missing`, `3 met`; `print` cited with file and rule; `currency` linked to the non-goal. Two defects:
- the same defect reported twice, in CORRECTNESS and in SPEC (would double the fix work in a loop)
- the non-goal violation rated low

Fixed in the brief: every defect belongs to the first section that owns it (criterion missing or wrong: SPEC); behaviour touching a stated non-goal is at least medium.

## 4. cj-reviewing brief after the fix (GREEN, two runs)

Both runs: all four defects, each once, threshold and `ValueError` high in SPEC, `print` medium in STANDARDS, `currency` medium in SPEC, missing tests medium in CORRECTNESS. Same shape and severities in both runs.

## 5. Real PR, new mutation never wired into the app (RED, old brief)

Fixture: a GraphQL gateway PR at the commit an external reviewer saw. It adds a mutation with a resolver, a use case, DI registration and a resolver test. Planted by accident, found later by that external reviewer:
- the app's `Mutation` map in the composition root (not in the diff) never spreads the new resolver provider; the call returns null without errors, which the contract defines as success, and writes nothing
- the use case imports a concrete infrastructure class, against the repo's ports-and-adapters guideline; that guideline sits in a folder `AGENTS.md` names only "when the task needs it"
- the integration test goal the test strategy requires for the change did not run

The caller's standards list held the coding guidelines and the test strategy, not the architecture folder. The resolver test calls the provider module directly and is green. Runs on 2026-10-08, same brief, three models:
- mid-sized model, two rounds: none of the three; contract "met" because "the mutation is registered in the resolver provider and the DI file"
- newest large model: none of the three, three low findings
- Opus tier: the ports break (derived from an `AGENTS.md` rule and sibling use cases), not the wiring, not the test goal; 32 tool calls against 14 to 16

## 6. New brief: READ, WIRING and CHECKS slots, guideline folders passed (GREEN)

Opus tier, effort high, two runs: both report the missing wiring as high with `app.ts` line, the ports break as medium with the guideline quoted, and a required test goal not run as medium (one names the integration goal exactly). READ lists 8 and 13 files; the second names every file it skipped with a reason.

Mid-sized control, one run: finds the wiring (high) but opens no standards file; READ says so, STANDARDS stays empty. The slot makes the gap visible; it does not close it, so the brief keeps the Opus tier.

Defect in 2 of 3 runs: the wiring bug reported twice, once in CORRECTNESS and once as a SPEC finding. Fixed in the brief: a criterion that fails only because of a CORRECTNESS finding is marked partial with that id, without a SPEC finding of its own.

## 7. Brief after the duplicate fix (GREEN, two mid-sized runs)

Both runs: wiring high in CORRECTNESS, criteria lines `partial (F1)`, no SPEC finding of their own. The mid-sized runs still read standards thinly (one opened none, one skimmed headings and rated the ports break low), which confirms the Opus tier.

## 8. Loosened check without a reason, N+1 query, tightened checks (#67)

Fixture: a small Python shop repo with `AGENTS.md` and one commit "orders: totals per customer, drop legacy export". It adds `order_totals(db, customer_ids)`, which runs one `db.query` per id. It puts a bare `@pytest.mark.skip` on `test_refund_rejects_negative` while `refund()` keeps its guard. It deletes `legacy_export` together with its test, adds an assertion to another test and raises `fail_under` from 80 to 85. Issue #40: AC-1 `order_totals` returns totals per customer, AC-2 `legacy_export` is removed with its test, non-goal "changes to refunds". No reason for the skip anywhere. Runs on 2026-10-13, Opus tier, effort high.

Expected: the skip is a high CORRECTNESS finding that names the Loosened check; the loop query is under CORRECTNESS; the deleted test, the added assertion and the raised threshold give no finding.

Results:
- old brief, one run: missed expected. The skip is medium, with no Loosened check named. The loop query is low under CORRECTNESS. The raised threshold is a medium SPEC finding and the added assertion a low one.
- new brief, first wording, one run: met expected. Skip high with a LOOSENED line `no reason | F1`, loop query medium, no finding for the tightened checks or the deleted test.
- new brief, final wording, one run: met expected, same shape.

## 9. Loosened check with its reason in a deviation comment (#67)

Fixture: scenario 8 plus the comment `Deviation: skip test_refund_rejects_negative for now | the refund rule moves to the payments service in #41, which rewrites this test | ...` on the issue. The old brief has no slot for it, so the comment rides in the Spec field, as `cj-implement` passed it before; the new brief gets it under Deviations.

Expected: no finding for the skip, and one LOOSENED line naming the check and its reason.

Results:
- old brief, one run: missed expected. The skip is a low CORRECTNESS finding asking for a reason marker; nothing names the check and its reason as one line.
- new brief, first wording, one run: the skip met expected, LOOSENED line with the reason and no finding. The raised threshold was still a low SPEC finding. Fixed in the brief: a tightened check is no finding "in any section".
- new brief, final wording, one run: met expected. The report names the raised threshold and the added assertion as tightened, with no finding.

## 10. List endpoint without pagination (#67)

Fixture: a second repo with a route table in `shop/app.py`. The commit adds `GET /orders`, whose handler runs `SELECT id, total FROM orders ORDER BY id` with no limit and returns every row. Issue #60: AC-1 the route returns the orders as a list of `{id, total}`, sorted by id. Runs on 2026-10-13, Opus tier, effort high.

Expected: the fetch without a limit is reported under CORRECTNESS.

Results:
- old brief, one run: missed expected. Its only finding is a low one about a test fake that ignores the SQL.
- new brief, one run: met expected, a medium CORRECTNESS finding "List endpoint fetches every order with no pagination", with a cursor as suggestion.

The loop query of scenario 8 does not tell the versions apart by section: the old brief also reports it under CORRECTNESS, only as low where the new one says medium. A variant with the query hidden in a helper outside the diff gave the same picture, low and medium, so it was dropped.

## Model and effort of subagents (whetstone, 2026-10-23)

Setup for the three cases below: the orchestrating agent runs on Sonnet with medium effort. The caller is `cj-implement` step 6 and 7 for a PR of 279 changed lines in 9 files, touching no money, data or security path.

### 1. Model and effort of the subagents

**Setup:** `git diff origin/main...HEAD --shortstat` prints `9 files changed, 279 insertions(+), 29 deletions(-)`.
**Prompt:** "Review the diff before the PR opens."
**Expected:** before any dispatch it names the estimate (279 + 29 lines, 9 files, no risk touch) and asks one choice question with two numbered options: cheap (model and effort of the orchestrator) recommended, thorough (Opus, effort high) second, each with one pro and one con. It dispatches no subagent with a fixed model or effort until the user answers. After "1" (or "yes") the reviewer and both verifiers run with the orchestrator's model and effort. Before the change: the reviewer is dispatched on Opus with effort high and no question.

Results:
- new text (7f6447e), one run: met expected; regression guard, the new text must leave the question unchanged.

### 2. Risk touch

**Setup:** the diff changes the save format of the hero's silver.
**Expected:** the question names "touches data" as the reason to take the thorough option, and still recommends the cheap one.

Results:
- new text (7f6447e), one run: met expected; regression guard, the new text must leave the question unchanged.

### 3. Answer already carried

**Setup:** the caller passes "subagents: orchestrator model and effort, settled by the user".
**Expected:** no question; all subagents run with the orchestrator's model and effort.

Results:
- new text (7f6447e), one run: met expected; regression guard, the new text must leave the question unchanged.

## Cost question when thorough is not stronger (#174)

Setup for the cases below, unless a case says otherwise: the caller is `cj-implement` step 6 and 7 for the same PR of 279 changed lines in 9 files, touching no money, data or security path. The prompt is "Review the diff before the PR opens." Thorough dispatches Opus 5.5 (`claude-opus-5-5`), the Opus the harness picks for Opus. The Opus cases quote the model lines of Claude Code's system context, which name the model and list the most recent model family. "Stated" means the line given in the case stands in the orchestrator's system context.

### 11. Opus with effort high (AC-1)

**Setup:** system context: "You are powered by the model named Opus 5.5. The exact model ID is claude-opus-5-5. The most recent Claude models are the Claude 5 family: Opus 5.5 (claude-opus-5-5), Sonnet 5.5 (claude-sonnet-5-5), Haiku 5.5 (claude-haiku-5-5). Reasoning effort: high."
**Expected:** no cost question. The reply holds `Reviewer and verifiers: claude-opus-5-5, effort high; no cost question, thorough is not stronger`, with no line count or file count. The reviewer and both verifiers are dispatched with model Opus and effort high, both passed explicitly.

Results:
- old text (e2fd206), one run: missed expected. The question comes with both options; no skip line.
- new text (7f6447e), one run: met expected. No question, the skip line with `claude-opus-5-5, effort high`, every subagent on Opus 5.5, effort high.

### 12. Opus with effort xhigh (AC-2)

**Setup:** system context: "You are powered by the model named Opus 5.5. The exact model ID is claude-opus-5-5. The most recent Claude models are the Claude 5 family: Opus 5.5 (claude-opus-5-5), Sonnet 5.5 (claude-sonnet-5-5), Haiku 5.5 (claude-haiku-5-5). Reasoning effort: xhigh."
**Expected:** no cost question; the skip line names effort xhigh; all subagents are dispatched with model Opus and effort xhigh, never high.

Results:
- old text (e2fd206), one run: missed expected. The question comes; thorough would run the reviewer at effort high.
- new text (7f6447e), one run: met expected. No question, the skip line names effort xhigh, every subagent on xhigh.

### 13. Opus named, effort stated nowhere (AC-4)

**Setup:** system context: "You are powered by the model named Opus 5.5. The exact model ID is claude-opus-5-5. The most recent Claude models are the Claude 5 family: Opus 5.5 (claude-opus-5-5), Sonnet 5.5 (claude-sonnet-5-5), Haiku 5.5 (claude-haiku-5-5)." No effort in the context or from the user.
**Expected:** the cost question comes with both options, as in case 1 of "Model and effort of subagents".

Results:
- old text (e2fd206), one run: met expected; regression guard.
- new text (7f6447e), one run: met expected; the effort is unknown, so the question comes.

### 14. Sonnet with effort high (AC-5)

**Setup:** system context: "You are powered by the model named Sonnet 5.5. The exact model ID is claude-sonnet-5-5. The most recent Claude models are the Claude 5 family: Opus 5.5 (claude-opus-5-5), Sonnet 5.5 (claude-sonnet-5-5), Haiku 5.5 (claude-haiku-5-5). Reasoning effort: high."
**Expected:** the cost question comes with both options, as in case 1 of "Model and effort of subagents".

Results:
- old text (e2fd206), one run: met expected; regression guard.
- new text (7f6447e), one run: met expected; another model asks.

### 15. Carried thorough on Opus with effort max (AC-6)

**Setup:** system context: "You are powered by the model named Opus 5.5. The exact model ID is claude-opus-5-5. The most recent Claude models are the Claude 5 family: Opus 5.5 (claude-opus-5-5), Sonnet 5.5 (claude-sonnet-5-5), Haiku 5.5 (claude-haiku-5-5). Reasoning effort: max." The caller passes "subagents: thorough, settled by the user".
**Expected:** no question; all subagents run with model Opus and effort max, never effort high.

Results:
- old text (e2fd206), one run: missed expected. No question, but the reviewer runs at effort high and the verifiers at max.
- new text (7f6447e), one run: met expected. No question, the skip line, every subagent on effort max.

### 16. Opus with effort medium (edge case)

**Setup:** system context: "You are powered by the model named Opus 5.5. The exact model ID is claude-opus-5-5. The most recent Claude models are the Claude 5 family: Opus 5.5 (claude-opus-5-5), Sonnet 5.5 (claude-sonnet-5-5), Haiku 5.5 (claude-haiku-5-5). Reasoning effort: medium."
**Expected:** the cost question comes with both options; thorough is stronger.

Results:
- old text (e2fd206), one run: met expected; regression guard.
- new text (7f6447e), one run: met expected; medium is below high, so the question comes.

### 17. Opus, effort assumed from a default (edge case)

**Setup:** system context: "You are powered by the model named Opus 5.5. The exact model ID is claude-opus-5-5. The most recent Claude models are the Claude 5 family: Opus 5.5 (claude-opus-5-5), Sonnet 5.5 (claude-sonnet-5-5), Haiku 5.5 (claude-haiku-5-5)." The caller's brief adds: "the harness default effort is probably high."
**Expected:** the effort counts as unknown; the cost question comes with both options.

Results:
- old text (e2fd206), one run: met expected; regression guard, it asks in every case.
- new text (7f6447e), one run: met expected; an effort assumed from a default is unknown.

### 18. Risk touch on Opus with effort high (edge case)

**Setup:** case 11, and the diff changes the save format of the hero's silver.
**Expected:** no cost question; the skip line and the dispatch are as in case 11. The risk touch alone does not bring the question back.

Results:
- old text (e2fd206), one run: missed expected. The question comes and names "touches data" as the reason for thorough.
- new text (7f6447e), one run: met expected. No question, same line and dispatch as case 11.

### 19. Older Opus with effort high (AC-5)

**Setup:** system context: "You are powered by the model named Opus 4.7. The exact model ID is claude-opus-4-7. The most recent Claude models are the Claude 5 family: Opus 5.5 (claude-opus-5-5), Sonnet 5.5 (claude-sonnet-5-5), Haiku 5.5 (claude-haiku-5-5). Reasoning effort: high."
**Expected:** the model counts as unknown; the cost question comes with both options.

Results:
- old text (e2fd206), one run: met expected; regression guard.
- new text (7f6447e), one run: met expected; Opus 4.7 is not named as the newest Opus, so the question comes.
