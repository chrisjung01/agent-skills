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
