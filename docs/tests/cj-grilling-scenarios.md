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
