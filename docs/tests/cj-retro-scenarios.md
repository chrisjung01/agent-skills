# Scenarios: cj-retro

Paper exercises built from a real run (October 2026). The agent under test describes what it would do; it posts nothing and edits nothing. Each scenario runs once against the current skill text (baseline) and once against the changed one.

## 1. A spec finding whose issue another skill wrote

**Setup:** A `cj-implement` session built ticket #204. Its acceptance criterion "the graphic tests run against the module" left open which of about 30 scene tests count as graphic tests; the agent drew the line itself. No `## Spec` comment exists; the ticket body came from an epic. The local transcripts show that #204's body was drafted in a session that ran a skill outside the skills repo.

**Prompt:** "/cj-retro"

**Baseline (real run):** The finding named `cj-to-spec` without checking and routed it to `/cj-whetstone cj-to-spec`; the whetstone run found no `cj-to-spec` call behind the issue and changed nothing.

**Expected with skill:** Before routing, the agent finds the skill that wrote the issue (search the transcripts for the issue title or a body line), names it, reports the finding as outside the skills repo, and does not name `/cj-whetstone`.

## 2. A skill finding whetstone can find later

**Setup:** Same session. A second finding is about `cj-implement` itself: its 400-line size rule counted lines that were only moved. The user answers "Alle" to the question which findings to do.

**Prompt:** "/cj-retro"

**Expected with skill:** The report names `/cj-whetstone cj-implement` and quotes the finding with its evidence (the moment in the session, the skill line), so a later whetstone run reads it in this retro window.

## Results (2026-10-11)

- Baseline, scenario 1: named `cj-to-tickets` "by inference from the ticket itself", searched no transcript ("nothing tells me to verify where a ticket came from"), routed to `/cj-whetstone cj-to-tickets`.
- With skill, scenarios 1 and 2: searched the transcripts for the issue title, named the outside skill and reported it only; quoted the `cj-implement` finding with the moment and the skill line under `/cj-whetstone cj-implement`.
- Left open: the category list has no slot for a skill rule that misfires; such findings reach the report only through step 5.
