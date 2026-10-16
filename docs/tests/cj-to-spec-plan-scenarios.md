# Optional Plan scenarios

Source: [issue #72 Spec](https://github.com/chrisjung01/agent-skills/issues/72#issuecomment-6223364816).

All cases start with `/cj-to-spec <issue>` and an already grilled decision list; use the no-issue form where stated. Approve ordinary test seams and publications unless stated otherwise. Answers are conditional on the flow asking: never supply an unsolicited instruction to create a Plan. Simulate writes only.

## Results

Fresh-agent baseline read the original skill and SPEC.md before production edits. Cases 1–18 **missed expected**; cases 19–28 **met expected** and are regression controls, not evidence for the new change. Downstream Plan cases fail because the old path is absent; independently check each after editing. The original working glossary already contained the user’s uncommitted Plan definition; the committed base did not.

These paper scenarios observe proposed actions, not live GitHub writes. Run `bash scripts/validate.sh` separately for structural rules. Record fresh-agent after outcomes before claiming the change is verified.

## 1. Hard-to-reverse change alone
- Setup and answers: One ticket irreversibly replaces an identifier format; no other trigger.
- Expected: Name the trigger, offer create/skip, and wait.
- Before: **missed expected**. Old flow records merge danger and continues to reader/Spec approval without offering Plan.

## 2. New data model alone
- Setup and answers: Introduce a reversible transient domain model; no storage or untried tools.
- Expected: New model alone prompts create/skip.
- Before: **missed expected**. Old flow records entity decisions and continues without a Plan offer.

## 3. Persistent storage alone
- Setup and answers: Persist the existing model through already tested APIs; change is reversible.
- Expected: Storage alone prompts create/skip.
- Before: **missed expected**. Old flow records data decisions and continues without a Plan offer.

## 4. Untried tool behavior alone
- Setup and answers: Only uncertainty is delayed-write tool behavior; no other trigger.
- Expected: Offer Plan and retain the hard-case experiment requirement.
- Before: **missed expected**. Old hard-case experiment check applies, but no Plan offer appears.

## 5. Wait for the choice
- Setup and answers: Use case 2; provide no answer to the intended Plan question.
- Expected: Pause before continuing until create or skip is answered.
- Before: **missed expected**. Old flow never asks the question and advances to reader/Spec approval.

## 6. Several triggers
- Setup and answers: New persistent model requires a hard-to-reverse migration.
- Expected: One create/skip question names the applicable triggers.
- Before: **missed expected**. Old flow asks zero Plan questions.

## 7. Decision and experiment contents
- Setup and answers: Two risky decisions and one completed scratch experiment; answer create.
- Expected: Each decision has choice/reason/remaining risk; experiment has assumption/procedure/result; no implementation steps or writing-plans dependency.
- Before: **missed expected**. Old Spec records reasons/seams but produces no dedicated Plan or complete experiment record.

## 8. No experiments needed
- Setup and answers: New model uses established behavior; no experiment needed; answer create.
- Expected: Record decisions and risks without inventing experiment results.
- Before: **missed expected**. Old flow offers or produces no Plan.

## 9. Approval, actual link, and publication order
- Setup and answers: Existing issue, hard-to-reverse change, several tickets; create and approve both texts; simulated Plan write returns https://github.com/example/project/issues/8#issuecomment-100.
- Expected: cj-posting approves Plan, publish ## Plan, put actual link/findings in Spec, approve/publish Spec, then slice.
- Before: **missed expected**. Old flow approves/publishes Spec directly and slices without Plan approval/comment/link.

## 10. Plan findings inform first approved Spec
- Setup and answers: Chosen Plan experiment changes an observable outcome; user settles it through grilling.
- Expected: Incorporate the settled finding before first Spec approval/publication.
- Before: **missed expected**. Old flow has no Plan preparation checkpoint; ordinary checks do not provide the required Plan findings path.

## 11. Plan publication not approved
- Setup and answers: Triggered draft; create, then reject Plan publication.
- Expected: Do not publish a Spec claiming or linking an unpublished Plan.
- Before: **missed expected**. Old Plan approval path is absent; it seeks Spec approval instead.

## 12. Plan publication fails
- Setup and answers: Triggered draft; create and approve Plan; simulated comment API fails.
- Expected: Do not invent a Plan URL or publish a Spec implying successful Plan publication.
- Before: **missed expected**. Old flow attempts no Plan write and cannot reach its failure handling.

## 13. Explicit skip
- Setup and answers: Triggered draft; answer skip; existing checks pass.
- Expected: Create/publish no Plan and proceed after receiving the skip decision.
- Before: **missed expected**. Old no-Plan outcome occurs without offering or receiving the required skip choice.

## 14. No issue, chosen Plan
- Setup and answers: No issue exists; triggered draft; create and approve required publications.
- Expected: Approved problem issue first, Plan comment second, linked Spec comment third.
- Before: **missed expected**. Old step 6 creates an issue with Spec body and no Plan comment.

## 15. Revision introduces risk
- Setup and answers: Published Spec; requested revision introduces persistent storage.
- Expected: Offer create/skip before publishing revision.
- Before: **missed expected**. Old revision shortcut obtains approval and edits Spec without offering Plan.

## 16. Revision changes existing risk
- Setup and answers: Published Spec/Plan; revision changes a risky decision within the same trigger category; answer create.
- Expected: Approved Plan covers revised decisions; revised Spec links to it.
- Before: **missed expected**. Old revision path edits Spec and adds change note without Plan/link handling.

## 17. Revision skips with unresolved experiment
- Setup and answers: Published Spec revision adds untried delayed-write behavior; skip; experiment unavailable.
- Expected: Return unresolved decision to grilling and publish no revision.
- Before: **missed expected**. Old revision shortcut does not explicitly rerun hard-case checks; it only requires approval/edit/change note.

## 18. Documentation agreement
- Setup and answers: Compare skill, README tier guidance, glossary, and skill inventory.
- Expected: Same optional triggers/timing, Plan defined as decision record, and no new skill.
- Before: **missed expected**. Old skill/README omit Plan; original working glossary already has uncommitted definition, while committed base lacks it.

## 19. No trigger, one ticket
- Setup and answers: Reversible ordinary change; established tools/data; one ticket.
- Expected: No Plan proposal or comment.
- Before: **met expected (control)**. Old flow proceeds through ordinary Spec path without Plan.

## 20. More than three tickets without trigger
- Setup and answers: Four bounded changes require separate context windows but have no technical trigger.
- Expected: Ticket count alone proposes/publishes no Plan.
- Before: **met expected (control)**. Old count controls challenge record and slicing, not Plan.

## 21. Successful hard-case experiment
- Setup and answers: Untried behavior succeeds on scratch copy including delayed write; rerun with create and skip after editing.
- Expected: Observed hard-case evidence satisfies the requirement.
- Before: **met expected (control)**. Old check accepts observed evidence and remaining checks proceed.

## 22. Experiment disproves approach
- Setup and answers: Scratch experiment loses delayed output; rerun with create and skip after editing.
- Expected: Return affected decision to grilling before publication.
- Before: **met expected (control)**. Old unresolved/conflicting decision returns to grilling; no unresolved Spec publication.

## 23. Experiment inconclusive
- Setup and answers: Scratch run fails to establish delayed-write assumption; rerun with create and skip after editing.
- Expected: Return unresolved decision to grilling.
- Before: **met expected (control)**. Old required hard-case check remains unmet and open question goes to grilling.

## 24. Experiment unavailable or only planned
- Setup and answers: Required environment unavailable; only a future procedure exists; rerun with create and skip after editing.
- Expected: Do not treat planned experiment as observed evidence; return to grilling.
- Before: **met expected (control)**. Old Not tried rule sends the open question to grilling.

## 25. Experiment changes shared state
- Setup and answers: Try would change live repository settings; rerun with create and skip after editing.
- Expected: Do not run the try; preserve unresolved decision for grilling.
- Before: **met expected (control)**. Old SPEC.md forbids the try and treats it as an open question.

## 26. No issue, no Plan
- Setup and answers: No issue and no trigger; additionally rerun triggered-plus-skip after editing.
- Expected: Preserve filing/posting approvals and create issue with Spec body.
- Before: **met expected (control)**. Old no-trigger path creates issue with Spec body; explicit skip is separately tested in case 13.

## 27. Resume unchanged published Spec
- Setup and answers: Existing Spec/Plan; no new or changed technical risk.
- Expected: No repeated Plan question or duplicate Plan.
- Before: **met expected (control)**. Old posted-spec shortcut advances to remaining slicing/next step without Plan question/write.

## 28. Existing Spec approval and slicing order
- Setup and answers: Several tickets; ordinary Spec checks pass; approve Spec.
- Expected: Spec posting precedes separately approved slicing.
- Before: **met expected (control)**. Old cj-posting handles Spec first, with a separate later slice decision.

## After results

2026-10-16, one fresh agent on the updated text: all 28 cases **met expected**. Proposed actions, grouped by shared evidence:

- 1–6: named each independent trigger, offered one create/skip choice, and stopped without an answer.
- 7–8: recorded each choice, reason and risk plus observed experiment details; stated no experiments needed when appropriate.
- 9–12: approved/published Plan before Spec, used the returned comment link, incorporated findings, and stopped for denied approval or failed publication.
- 13–14: skip created no Plan; create without an issue filed the approved problem first, then Plan and Spec comments.
- 15–17: new or changed revision risks prompted again; revised decisions got a linked Plan; unavailable evidence blocked skipped-Plan revision publication.
- 18: skill, README and glossary agreed; no skill was added.
- 19–20: no technical trigger meant no Plan, including four tickets.
- 21–25: both create and skip kept observed hard-case evidence mandatory; failed, inconclusive or unavailable evidence returned to grilling; no live mutation ran.
- 26–28: no-Plan new issue kept Spec body; unchanged resume did not repeat Plan; slicing stayed after separately approved Spec publication.

The same fresh agent ran existing `cj-to-spec-scenarios.md` cases 3, 4 and 18–23: all **met expected**. It retained comment-only publication for existing issues, ticket-count reporting, separate slice approval, one-PR fallback, resume handling, challenge records, safe experiments and no branch decisions in specs. These are regression controls, not new-change evidence. All outcomes are paper simulations, not live writes.

## Condition mutation results

2026-10-16, a separate agent applied each text inversion in a detached scratch worktree on `a3ff86e`, simulated the named case, and restored the files. The tree ended clean and was removed. These are manual paper traces, not automated executable tests.

Single-location inversions produced these missed expectations, or red results:

- M1–M4: exclude each trigger independently; cases 1–4 omit the required offer.
- M5: offer with no trigger; case 19 asks an unnecessary question.
- M6: ticket count becomes a trigger; case 20 offers a Plan for four tickets alone.
- M7: continue without an answer; case 5 advances before the choice.
- M9: draft on skip instead of create; case 7 loses the chosen Plan drafting path.
- M11: invent results when no experiment is needed; case 8 fabricates evidence.
- M12: successful evidence becomes unresolved; case 21 returns unnecessarily to grilling.
- M19: absent-issue path creates Spec body first; case 14 loses problem → Plan → Spec order.
- M20: Spec-body filing requires create; case 26 loses the no-Plan filing path.
- M21: revisions check only changed existing risks; case 15 misses newly introduced storage.
- M22: revisions check only new risks; case 16 misses changed decisions in an existing trigger category.

Other single-location inversions remained green because duplicate safeguards still enforced the original rule. The agent reran those by coherently inverting one logical condition across its repeated mentions in the skill, SPEC checks or resume path. Each following case then missed expected, or turned red:

- C8: one question per trigger in both mentions; case 6 asks three questions.
- C10: skip drafts/publishes Plan with its drafting condition changed too; case 13 creates a Plan after skip.
- C24/C13: failed/inconclusive evidence establishes the assumption in both files; cases 22/23 proceed instead of grilling under either choice.
- C14: unavailable evidence counts as complete; case 24 proceeds without the needed environment.
- C15: planned procedure counts as observed/completed evidence in both files; case 24 accepts a future procedure.
- C16: skip exempts the evidence check throughout draft/revision paths; cases 17 and 22–25 permit unresolved publication. Case 21 still passes because its evidence succeeds.
- C17/C18: denied approval/failed posting permits placeholder Plan links; cases 11/12 publish a Spec implying an unpublished Plan.
- C23: unchanged resume re-enters the Plan choice and permits duplication; case 27 prompts again and creates another Plan.
- C25: both files permit live shared-state experiments; case 25 proposes the prohibited setting change.
- C26: create authorizes publication and the caller waives separate posting approval; case 9 omits full-Plan approval.

The approval/dependency inversions detect a caller-contract violation, not an end-to-end publication failure: dependency skills were not edited, and no real writes ran. Original surviving mutations show redundant protection; they are not claimed as red single-location tests. Copy and layout checks have no condition to invert.
