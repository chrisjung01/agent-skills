---
name: cj-review-loop
description: Use when the user's own branch should get an independent review, either one pass after finishing a feature or before merging ("review my branch", "einmal reviewen"), or reviewed and fixed in rounds until no high or medium findings remain ("/cj-review-loop", "review loop", "review and fix", "review until green"). Not for someone else's PR; that is cj-review-teammate-pr.
---

# Review loop

A fresh reviewer judges the branch diff; you assess each finding and the user sees every one before you touch code. Two modes:

- **once**: one review, nothing fixed, the user decides. Default when the user asks for a review or one pass.
- **loop**: fix high and medium, run the checks, review again until nothing high or medium is open. Default when the user asks to fix, and when another skill calls this one.

The review runs even when the change "is simple".

## Setup

1. Base: user argument, otherwise the default branch. `BASE=$(git merge-base <base> HEAD)`. Empty `git diff $BASE`: stop and say so.
2. Requirements: the plan, the issue with its acceptance criteria verbatim, or a one-paragraph description of what the change should do.
3. Checks (loop mode): Call the Skill tool with "cj-project-context" for: commands. None found: tell the user once that you work without checks.
4. Ledger, in the todo list or your head: per finding `id, section, severity, location, title, status, attempts`.
5. Call the Skill tool with "cj-reviewing" for the reviewer brief and the severity scale. Each round gets a new reviewer that sees only the ledger, never your history.

Done when base, requirements, checks (or their absence) and the reviewer brief are in hand.

## once

1. Dispatch the reviewer.
2. Show all findings unchanged, by section, with your one-line take per high and medium: agree, disagree with reason, or unsure.
3. Ask one question as a choice: fix all high and medium, decide per finding (then one finding per question: fix, ignore, or something else), or switch to loop mode. Change and commit nothing before the answers.

Done when the user has decided per finding.

## loop (max 5 rounds)

1. **Review.** Dispatch the reviewer with the ledger.
2. **Assess.** Show every finding, low included. Per high and medium:
   - legitimate: fix it, `attempts += 1`
   - wrong or not applicable: `rejected`, with a concrete reason from code or tests; when in doubt, fix
   - low: ledger only
   Fix the root cause: grep the other callers before adding a guard in one place. Change only what the findings name.
3. **Check.** Run every check from setup. Red: repair first; a fix that breaks a check counts as another attempt.
4. **Stop?**
   - done: no new and no `still_open` high or medium
   - escalate: a finding still open after 3 attempts; stop at once with the escalation report
   - round limit: after 5 rounds, stop with everything still open
   Otherwise the next round.

Report with the templates in [REPORT.md](REPORT.md): the escalation report on escalate, the final report otherwise. Write it scannable: the first line is the result or the decision needed. One sentence per bullet, at most 25 words. A list has 3 to 5 items, never more than 7; a longer one splits under two headings or goes onto a page. Three or more items go one per line, never inside a sentence. No nested lists, and no tables unless this skill names one; code, log and command blocks and the `Next:` block keep their own shape. Bold only as a label at the start of a line, code font for paths and commands. Headings name the topic in the reader's terms, not generic labels like Scope or Process. No made-up abbreviations: write glossary terms out unless the glossary defines the abbreviation as the term; code identifiers, paths and issue or PR numbers stay. Before a question, the reply text holds the full list with details, numbered; with nothing to list, one line of context is enough. A question has 3 options, a fourth only as a real alternative, the recommended one first; with only one real option, the proposal and something else. Each option names its item and says in full words what it means and its one consequence, so the user decides without scrolling up. Tables keep their shape.

Done when one stop condition holds and its report is shown.

## Hand-off

Everything the user reads goes through the `unslop` skill. The changes stay uncommitted in the working tree unless the calling skill says otherwise; commit and push are the user's call. Call the Skill tool with "cj-posting" before any push.
