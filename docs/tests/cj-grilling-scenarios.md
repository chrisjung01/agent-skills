# Scenarios: cj-grilling

Skill under test: `cj-grilling`.

## 1. A question that rests on a found PR links it

**Setup:** while grilling issue #5490 ("tenant-specific checkout rules"), the agent found merged PR https://github.com/acme/app/pull/4321 "TenantConfig for one tenant" by a colleague: it added a per-tenant config object, was reverted two weeks later, and is the obvious precedent.
**Prompt:** ask the next question: reuse that approach or build a new rule engine.
**Expected:** the question links the PR (the URL) and says in one line what it did. A question that names the colleague's approach without a link or summary fails.
**Origin:** sessions 6250afa1 ("kannst du mir mal kurz den PR von <colleague> zeigen?") and d68f497a ("kannst du mir mal die Links geben?").

**Variant used for the runs:** the PR appears only in research notes from 20 turns earlier, as "merged-then-reverted #4321 (a colleague) added a TenantConfig for one customer"; the question is "how are tenant rules modelled". Handing the URL in the prompt made the baseline pass.

## Runs

- 2026-10-09, before the change: variant three runs. All three wrote a bare `#4321` without a URL (one summary line each). Fail.
- 2026-10-09, after the change: variant three runs. All three gave `https://github.com/acme/app/pull/4321` with one line on what it did. Pass.
- 2026-10-14, after #109 (template replaced by the `cj-asking` pointer, URL rule kept in the pointer sentence): variant two runs. Both gave `https://github.com/acme/app/pull/4321` with one line on what it did, as reply text, no picker call. Pass.

## 2. A handed-back decision is asked again

**Setup:** repo `acme/notes-app`. The agent is grilling issue #212 "Archive old notes". Settled so far: notes older than a year are archived, not deleted. The agent asked "Who can restore an archived note?" with A) the owner only (recommended), B) the owner and every collaborator, C) nobody, archive is final.
**Prompt:** the user answers "whatever you think is best".
**Expected:** the agent does not record A as the decision. It asks the same decision again with two concrete options, each with its consequence, and waits. A reply that records "owner only" and moves on to the next decision fails.
**Origin:** agent-skills#61, adapted from addyosmani/agent-skills `interview-me`.

## 3. The agent checks it can predict the user before closing

**Setup:** repo `acme/notes-app`, issue #212 "Archive old notes". Decided: notes older than a year are archived; the owner restores them; archived notes are hidden from search; the archive job runs nightly. Each was answered with the recommended option except search, where the user picked "hidden" over the recommended "shown with a label" and said "people complain about clutter". Not asked: whether shared notes that a collaborator edited last month count as old by their creation date or by their last edit. The agent sees no further open decision of its own.
**Prompt:** the user answers the nightly-job question with "A". (Write the agent's next reply.)
**Expected:** before it closes, the agent checks whether it can predict the user's answers to its next three questions. The age of a recently edited shared note is one it cannot predict, because the user already overruled a recommendation once on clutter. It asks that decision before the summary. A reply that goes straight to the summary of decisions fails.
**Origin:** agent-skills#61, adapted from addyosmani/agent-skills `interview-me`.

**Variant used for the runs:** the prompt says only that notes can be shared and collaborators edit them; it does not name the unasked decision, so the agent has to find it.

## Runs for scenarios 2 and 3

- 2026-10-13, before the change: one run each. Scenario 2 recorded "owner only" because the user left it to the agent and moved on: missed expected. Scenario 3 went straight to the summary of four decisions: missed expected.
- 2026-10-13, after the change: one run each, on the text before the review fixes (second hand-back, predicted answers in the summary). Scenario 2 recorded nothing and asked the restore decision again with two options: met expected. Scenario 3 named its next three questions, predicted two, and asked the shared-note decision it could not predict before the summary: met expected.
- 2026-10-13, again on the text of review round 2: one run each. Scenario 2 asked again with two options, and after a second hand-back recorded "owner only" as delegated, marked for the summary: met expected. Scenario 3 asked the unpredicted age decision before the summary and kept two predicted answers for it: met expected.
- 2026-10-13, scenario 3 on the current Done-when text, two turns: asked the unpredicted age rule first, recorded the read-only rule as predicted, then asked the collaborator decision: met expected. It read "after two such rounds" as round 2, so the stop now names a third round.

## 4. Non-functional quality never discussed is asked before close

**Setup:** repo `acme/notes-app`, issue #230 "Export a note as PDF". Decided: 1) one note per export, 2) the button sits in the note's menu, 3) A4 with images, 4) a failed export shows an error with a retry button, 5) done when the PDF opens in the system viewer, 6) no new library, the existing print module renders it, 7) nothing about exports is stored, 8) the menu entry shows a spinner until the PDF opens. Every answer took the recommended option. Never discussed: how large a note may be, how long an export may take, what happens to the app meanwhile. The agent ran the "predict the user" check: its next three decisions (file name, page margins, dark-mode colours) are all predicted from earlier answers.
**Prompt:** the user answers decision 8 with "1". (Write the agent's next reply.)
**Expected:** the agent asks about non-functional quality (note size, export time, app responsiveness) in a turn of its own, one question, before any closing summary. A reply that shows the closing summary fails.
**Origin:** agent-skills#74, test seam case 1.

## 5. A skill-text change rules UX flow out with a fact

**Setup:** repo `acme/agent-skills`, a repo of agent skills (Markdown instructions, no app, no screens). Issue #88 "cj-review: name the severity scale in the reviewer prompt". Decided: 1) the reviewer prompt names the scale by its four levels, 2) only `skills/cj-review/SKILL.md` changes, 3) no new term, the glossary's *Severity* is used, 4) a finding without a level counts as low, 5) the prompt stays under 20 lines, 6) done when a scenario run shows a finding with its level, 7) old review comments stay as they are. The agent ran the "predict the user" check: its next three decisions are all predicted.
**Prompt:** the user answers decision 7 with "1". (Write the agent's next reply.)
**Expected:** the closing summary shows a coverage table above the numbered decision list, seven rows: functional scope, domain and data model, UX flow, non-functional quality attributes, edge cases and error handling, constraints and tradeoffs, completion signals. UX flow reads Out of scope with the fact that the change touches skill text only. No question about UX flow is asked. Clear rows name decision numbers; no row reads Partial or Missing. A summary without the table, or a question about UX flow, fails.
**Origin:** agent-skills#74, test seam case 2. Its "no UX question" part is a regression guard.

## 6. A UI change asks UX flow instead of ruling it out

**Setup:** repo `acme/notes-app`, issue #241 "Delete several notes at once". Decided: 1) any notes the user owns can be selected, 2) deleted notes go to the trash for 30 days, 3) a note shared with others is removed only for the owner's view after a warning, 4) deleting 500 notes finishes within two seconds, 5) a failed delete keeps the remaining notes selected and names the failed ones, 6) done when the trash shows the deleted notes, 7) no new database table. Never discussed: how the user enters selection mode, selects, and confirms. The agent ran the "predict the user" check: its next three decisions (trash sort order, undo toast duration, selection limit) are all predicted.
**Prompt:** the user answers decision 7 with "1". (Write the agent's next reply.)
**Expected:** the agent asks about the UX flow (entering selection mode, selecting, confirming) in a turn of its own before any closing summary. A reply that closes, or that rates UX flow Out of scope, fails.
**Origin:** agent-skills#74, test seam case 3.

## Runs for scenarios 4 to 6

- 2026-10-19, before the change, skill text of 8a68c26: one run each. Scenario 4 (setup with decisions 7 and 8) went straight to the summary with three predicted decisions: missed expected. Scenario 5 showed the summary without a coverage table: missed expected. Scenario 6 showed the summary and never asked how the user selects and confirms: missed expected.
- 2026-10-19, after the change, skill text of 0211c8e: one run each, scenario 6 twice. Scenario 4 asked about the time limit for a large export before any summary: met expected. Scenario 5 showed seven rows above the numbered list, UX flow Out of scope because only skill text changes, no UX question: met expected. Scenario 6 rated UX flow Partial and asked how the user gets from selection to deletion, both runs: met expected.
- 2026-10-19, scenario 3 on the skill text of 0211c8e: asked which date makes a shared, recently edited note "older than a year" before any summary: met expected.
- An earlier text without "they answer the category … predicted decisions alone never make a row Clear" rated UX flow in scenario 6 Clear from decisions that only touched it; that sentence was added for it.
