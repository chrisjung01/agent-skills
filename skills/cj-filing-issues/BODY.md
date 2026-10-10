# Issue body

Read before drafting any issue. The repo's template sets the headings and their order; this file sets what goes under them. No template: the headings of the repo's recent issues of the same kind, in their language; no such issues: the kind's points below. A template section with nothing true to say gets "n/a", never filler.

## By kind

**Bug** (and UX finding)
- What happens, for whom (role from `glossary`), since when.
- Steps, numbered, marked by origin: "run", "traced in code, not run", or "as reported, not reproduced". Never steps nobody ran, traced or reported.
- Expected / actual, one line each.
- Source: permalink, error-tracker link, support reference; app or variant, platform, version.
- Suspected code: `file:line`, when known. This is the one kind that may name files.
- Regression test: which test would catch it.
- Impact in one sentence, with the tier when the caller has a tier scale.

**Flaky test**
- The test or flow name, the failure pattern (same step and selector, or different steps), and the observed rate with its run links.
- Suspected cause when known: timing, shared test data, device, selector.
- Done means: a fix, or the test quarantined with a reason the user decided on.

**Task or feature** (also a child of an epic, a follow-up)
- Problem: what is wrong or missing today, from the user's or the team's point of view, one to three sentences. Not the solution.
- Solution: what changes, in behaviour, not implementation steps.
- Non-goals: what this deliberately leaves out. At least one when the scope has an obvious neighbour.
- Acceptance criteria: 2 to 7 checkboxes. Each one observable by someone who did not write the code ("A total of exactly 100 gets 10 % off", "the order list shows the delivery date"). Never "works as expected", "is tested", "is clean", "handles errors".
- Prerequisites and risks: named issues or decisions it depends on; one line per real risk.
- Open questions, explicit, when the issue cannot be built without an answer.
- Affected entities and modules by name, without paths.
- Decisions that apply, linked (ADR, decisions file, glossary terms).
- Out: code file paths, line numbers, code snippets, step-by-step implementation (links to specs, decisions and docs are fine). The spec written when the issue is picked up holds the how. Test: someone with only this issue can write that spec without asking.

**Epic**
- Goal and why, one paragraph.
- The children by wave, linked, with the blocked-by edges visible (the tracker's sub-issues do the rest).
- Open questions, explicit. Never a silent assumption.
- Non-goals.

## Writing

- Every sentence carries a fact the reader needs. Delete sentences that restate the title, announce what follows ("This issue aims to", "In this ticket we"), or summarise what the lists already say.
- Concrete over general: the screen, the number, the role, the term from the glossary. One term per concept, the glossary's.
- Lists only for things that are lists. No bold label at the start of every line, no headings for one line of text, no emojis.
- Budgets: a bug body fits on one screen without its logs; a task's problem is at most three sentences; an epic body links to its plan instead of repeating it.
- Nothing invented: no steps nobody ran or traced, no numbers without a source, no example identifiers that do not exist.
- Then run the `unslop` skill over title and body (cj-posting does this at the gate; do it before showing).

## Check before showing

- [ ] Every acceptance criterion could be checked by someone else, with a yes or no.
- [ ] The problem is stated without the solution.
- [ ] Every template section is filled with something true or marked "n/a".
- [ ] No sentence survives that only restates another one.
- [ ] Relations from Find are in the body (related) or set as edges (parent, blocked by, blocks).
