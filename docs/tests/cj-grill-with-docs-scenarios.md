# Scenarios: cj-grill-with-docs

Skill under test: `cj-grill-with-docs`.

## 1. The closing reply tells the user what to type next

**Setup:** issue #5091 is grilled to the end; the user confirmed the decision list. The conversation is in German.
**Prompt:** "Passt so." (write the closing reply).
**Expected:** the reply ends with an instruction the user can follow without interpreting it: run `/cj-to-spec <issue>` now, in this session, and do not `/clear` first. A line that only names the command with a reason ("Nächster Schritt in dieser Session, vor jedem /clear: /cj-to-spec …") fails: it reads as "do this, then clear" or "clear, then do this".
**Origin:** session 8d055489, the user: "Was soll das? Das sind keine klaren Anweisungen."

## Runs

- 2026-10-09, before the change: one run. "Nächster Schritt: `/cj-to-spec …` hier in dieser Session, vor `/clear` oder einer Kompaktierung." Fail.
- 2026-10-09, after the change: one run. "Nächster Schritt: Führ jetzt `/cj-to-spec …` aus, in dieser Session. Vorher kein `/clear`: …" Pass.

## 2. A risk sign makes the proposal big

**Setup:** issue #640 "Store the card's last four digits for faster checkout": add a column to the payments table and show the digits on the checkout screen. No comments, no `## Challenge` comment.
**Prompt:** "/cj-grill-with-docs #640" (write the first question).
**Expected:** the first question proposes the big tier, names the signs it found (money, data), and offers normal and small as the other options. A first question about the feature itself, or no tier question, fails.

## 3. A typo makes the proposal small

**Setup:** issue #641 "Typo on the login button: 'Sing in' should read 'Sign in'". No comments.
**Prompt:** "/cj-grill-with-docs #641" (write the first question).
**Expected:** the first question proposes the small tier, names the sign (a copy change whose expected text the issue states), and offers normal and big as the other options.

## 4. Any other feature makes the proposal normal

**Setup:** issue #642 "Sort the task list by due date": a toggle above the list, no money, data migration, security or irreversible step, one screen.
**Prompt:** "/cj-grill-with-docs #642" (write the first question).
**Expected:** the first question proposes the normal tier and offers big and small as the other options.

## 5. The challenge runs before the first grilling question

**Setup:** as scenario 2; the user answered "1" and confirmed the big tier.
**Prompt:** "1" (write the next message).
**Expected:** the next message works on the challenge (core problem, goal link, data, smaller lever, or a gate question) as `CHALLENGE.md` says. A question about how to build the feature (column type, screen layout) before the verdict fails.

## 6. The verdict is posted, then grilling goes on

**Setup:** as scenario 5; every gate closed, the verdict is "build differently" with the smaller lever "show the card brand only, no new column", kill criterion "checkout time not down 5 % by 2026-12-31".
**Prompt:** "weiter" (write the next message).
**Expected:** a `## Challenge` comment in the `CHALLENGE-REPORT.md` format is drafted and shown for posting through `cj-posting`; once posted, the same run asks its first grilling question with the verdict and the smaller lever as settled. A reply that names `/cj-grill-with-docs` as a next step, or grills without a posted or drafted verdict, fails.

## 7. A blocking verdict stops the run

**Setup:** the user typed `/cj-grill-with-docs #640` (issue as in scenario 2) and confirmed the big tier. No separate challenge command was typed. The goal-link gate stays open after its question, and the gates end in the verdict "validate first": a fake-door button for two weeks.
**Prompt:** "keine Quelle" (write the messages until the run ends).
**Expected:** the run posts that verdict, asks no grilling question at any point, and ends with the validation step and who runs it under "Next:". A grilling question, a hint to run another command for the challenge, or `/cj-to-spec` as next step, fails.

## 8. The normal tier gets no challenge

**Setup:** as scenario 4; the user confirmed the normal tier, the grill ran to the end.
**Prompt:** "Passt so." (write the closing reply with the decision list).
**Expected:** no challenge question was asked, no line recommends a challenge, and the decision list holds no "Not challenged" line.

## 9. Security turning up while grilling reopens the tier

**Setup:** as scenario 4, normal tier confirmed. While grilling, the user answers: "The sort order must also be shared with other users through a public link without login."
**Prompt:** that answer (write the next question).
**Expected:** the next question proposes the big tier again, naming security. An ordinary grilling question fails.

## 9b. A blocking verdict from a late challenge ends the run

**Setup:** as scenario 9, three grilling decisions still open. The user answers yes to the big tier. No separate challenge command is typed. The gates end in the verdict "don't build", and the user agrees to post it.
**Prompt:** "ja" (write the messages until the run ends).
**Expected:** the challenge runs and its verdict is posted before any further grilling question; after that, no grilling question and no `/cj-to-spec`; the reply ends with the next step for "don't build" under "Next:".

## 10. A rerun with a Tier line asks no tier question

**Setup:** issue #642 has the grill's question comment from an earlier run, holding `Tier: normal, confirmed` and two answered questions.
**Prompt:** "/cj-grill-with-docs #642" (write the first question).
**Expected:** no tier question, no challenge and no line recommending one; the first question grills what is still open.

## 11. A rerun after a blocking challenge comment ends with its next step

**Setup:** issue #640 has a `## Challenge` comment, the latest one, with "Recommendation: validate first" and no later comment.
**Prompt:** "/cj-grill-with-docs #640" (write the reply).
**Expected:** no tier question, no new challenge, no grilling question; the reply ends with the validation step from that verdict.

## 12. The small tier asks no grilling question

**Setup:** as scenario 3; the user confirmed the small tier.
**Prompt:** "1" (write the reply).
**Expected:** no grilling question; the reply ends with `/cj-implement #641` under "Next:".

## 13. The question comment carries the tier

**Setup:** as scenario 4, normal tier confirmed. Two questions only the issue author can answer remain.
**Prompt:** "Frag den Autor." (draft the comment).
**Expected:** the drafted comment holds the line `Tier: normal, confirmed`.

## 14. A fix of stated behaviour makes the proposal small, whatever its length

**Setup:** issue #650 "Search ignores the selected category: results show all categories. Expected, as the README says: only the selected one." The fix is about 60 lines across two files. No comments.
**Prompt:** "/cj-grill-with-docs #650" (write the first question).
**Expected:** the first question proposes the small tier, names the sign (a fix that restores behaviour the issue states), and offers normal and big as the other options.

## 15. A refactor without behaviour change makes the proposal small

**Setup:** issue #651 "Refactor: move the date formatting duplicated in four screens into one helper. Behaviour unchanged." No comments.
**Prompt:** "/cj-grill-with-docs #651" (write the first question).
**Expected:** the first question proposes the small tier, names the sign (a refactor without behaviour change), and offers normal and big as the other options.

## 16. A confirmed small fix or refactor goes straight to implement

**Setup:** a) issue #650 as in scenario 14; b) issue #651 as in scenario 15. The user typed `/cj-grill-with-docs` with the issue and answered the tier question with the small option.
**Prompt:** the small option's number (write the reply).
**Expected:** no grilling question; the reply ends with `/cj-implement #650` (a) or `/cj-implement #651` (b) under "Next:". Both meet expected on the old text too, where the user's tier already holds; they guard the handoff.

## 17. A risk sign still wins over a small fix

**Setup:** issue #652 "Refunds use last year's VAT rate. Expected, as the pricing doc says: the current rate." A three-line fix. No comments.
**Prompt:** "/cj-grill-with-docs #652" (write the first question).
**Expected:** the first question proposes the big tier, naming money, and offers normal and small as the other options.

## 18. A small pick against a stake is posted on the issue

**Setup:** a) issue #652 as in scenario 17; b) issue #650 as in scenario 14; c) issue #640 as in scenario 2. The user typed `/cj-grill-with-docs` with the issue and answered the tier question with the small option.
**Prompt:** the small option's number (write the messages until the run ends).
**Expected:** a) before naming `/cj-implement #652`, a comment holding the line `Tier: small, confirmed` is drafted and posted through `cj-posting`. b) and c) post nothing: #650 has no stake, #640 is a feature; the reply names `/cj-implement` under "Next:".

## 19. A Tier line from any comment settles the tier

**Setup:** issue #652 as in scenario 17, plus one comment posted by `/cj-implement`: "Going on as small: a three-line fix of a stated rate." and the line `Tier: small, confirmed`. No `## Challenge` comment, no grill question comment.
**Prompt:** "/cj-grill-with-docs #652" (write the first message).
**Expected:** no tier question and nothing posted; the run treats the tier as small and names `/cj-implement #652` under "Next:".

## Runs, scenarios 2 to 13

- 2026-10-16, before the change (#96): one fresh grader. 2 to 13 missed expected; only a variant of 9 met ("two tickets" proposes no tier), because the old text has no tier at all, so the variant was dropped. The first versions of 7 and 10 met expected on the old text, so both were rewritten.
- 2026-10-16, after the change: one fresh grader. 2 to 13 met expected. A second pass on 6, 7, 9 and 10 after the entry wording in `CHALLENGE.md` changed: met expected. Gaps it named in the unchanged challenge text (which open gate gives which verdict, no owner field in the report) stay out of scope.
- 2026-10-16, 9b (added after review) and the reordered tier step: one grader per version. Before, 9b missed expected: no challenge runs inside the grill, and grilling goes on. After, 2, 9b, 10 and 11 met expected.
- 2026-10-16, final text after the second review round: one fresh grader. 2 to 13 and 9b met expected.

## Runs, scenarios 14 to 17

- 2026-10-16, before the change (#100): one blind grader. 14 and 15 missed expected (normal proposed); 16 and 17 met expected, kept as regression guards.
- 2026-10-16, after the change: one blind grader. 14 to 17 met expected; 17 cites the line that a big sign wins.
- 2026-10-16, text after the first review round: one blind grader. 14, 15, 16a, 16b and 17 met expected.
- 2026-10-16, final text after the second review round (normal now covers anything else): one blind grader. 14, 15, 16a, 16b and 17 met expected.

## Runs, scenarios 18 and 19

- 2026-10-16, one blind run per version (#133), before on c763457, after on 900900d. Before: 18a missed expected (nothing posted), 19 missed expected (tier question, the line was not in the grill's own comment); 18b and 18c met expected, kept as regression guards. After: 18a, 18b, 18c and 19 met expected.
