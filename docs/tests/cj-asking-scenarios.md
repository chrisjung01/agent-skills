# Scenarios: cj-asking

Paper exercises built from real sessions (October 2026), run by a fresh agent that writes the exact reply it would send and names every tool call. It posts nothing and edits nothing. "Before" gives the agent `cj-grilling` and `cj-retro` at commit `486bfb8`, without `cj-asking`; "after" gives it `cj-asking`, `cj-grilling` and `cj-retro` from this branch. In both runs the harness offers a picker tool (`AskUserQuestion`) and the user reads the Claude Code terminal.

Expected in every scenario, on top of its own line: the full numbered list in reply text, no picker call, exactly one question per message, a confirm question repeating the whole list, a recommendation line where there are options.

## 1. Eleven decisions never written

**Setup:** `cj-grilling` on a game feature ticket. Over the session the user settled eleven decisions, each answered in its own turn:

1. Frost cannon ships in this ticket.
2. Siege breaker ships in this ticket.
3. Frost slows enemies by 30 % for 4 seconds.
4. Slows do not stack; the longest one counts.
5. Siege breaker deals double damage to walls only.
6. Both towers unlock at wave 5.
7. Spike walls leave this ticket.
8. Spike walls are parked like spike traps: out of the start pool, a note in the spec.
9. The start pool has seven modifiers instead of eight.
10. The building panel shows the slow percentage.
11. "Building panel" goes into the glossary.

The last answer (decision 8) came back from a question. The agent then grepped the glossary for "building panel", so the next question follows a tool call in the same turn.

**Prompt:** the grill's frontier is empty; ask the user to confirm the decisions.

**Baseline (real run, session `815e3dd0`, 2026-10-14):** right after the tool call the agent called the picker with "Passen die elf Punkte?" and options naming points 3 and 11. The eleven points stood nowhere in reply text. The user rejected the question and asked "Welche 11 Punkte?".

**Expected:** all eleven decisions written out, numbered, directly above one question with `1) Yes` and `2) No`, each with what it does on its own line.

## 2. Seven decisions, confirmed again six messages later

**Setup:** `cj-grilling` on a crash-reporting ticket in a mobile app. Six messages ago the agent listed seven decisions:

1. Tags are set in native code before the crash reporter starts.
2. The release name comes from the build number.
3. Debug builds send nothing.
4. Personal data is scrubbed on the device.
5. A start race (tags missing on very early crashes) is accepted for now.
6. The native SDK version follows the app's crash-reporting package.
7. A dashboard filter for the new tags is a follow-up ticket.

Since then the user asked two questions about decision 5 and kept it. Nothing in the decisions changed.

**Prompt:** ask the user to approve the list before the spec is written.

**Baseline (real run, session `850274fb`, 2026-10-14):** the agent called the picker with "Passt die Entscheidungsliste?" and options to change decision 5 or 6. The list was not repeated. The user rejected it: "Wieder nichts aufgelistet. Was soll ich erlauben! … liste mir das noch mal auf, bevor ich das approven soll".

**Expected:** all seven decisions written again in full, numbered, directly above one question with `1) Yes` and `2) No`, although they were shown six messages ago.

## 3. A dense one-line question

**Setup:** `cj-grilling` before a spec for issue #45. The issue bundles six gaps in two phases. After building: 1 acceptance by hand, 2 a kill criterion reminder, 3 recording deviations. Before building: 4 coverage of the spec by tests, 5 a planning step, 6 size lanes in the README. `cj-to-spec` asks for one goal per spec.

**Prompt:** ask which of the six gaps this spec covers.

**Baseline (real run, session `983f5ea9`, 2026-10-13):** one picker question held the whole context in one line, "Umfang: #45 bündelt 6 Lücken in zwei Phasen (nach dem Bauen: 1 Abnahme, 2 Kill-Kriterium, 3 Doku …)", with four options in short labels. The user answered "ich verstehe die Frage leider nicht".

**Expected:** the six gaps numbered in reply text, one per line, under the two phases; "kill criterion" and "size lanes" explained in plain words; options `1)` `2)` `3)` each with a meaning line and a consequence line, the recommended first, then a recommendation line.

## 4. The user answers "ja"

**Setup:** scenario 3's question was asked as expected, with option 1 "only gaps 1 to 3" recommended.

**Prompt:** the user answers "ja".

**Expected:** the agent takes option 1, says so in one line and moves to the next decision; no question asks which option was meant.

## 5. The user answers in their own words

**Setup:** scenario 3's question was asked as expected.

**Prompt:** the user answers "nimm 1 und 2, die Abweichungen machen wir später".

**Expected:** the agent records gaps 1 and 2 as the scope and gap 3 as a follow-up, and moves on; it does not ask the same decision again as numbered options.

## 6. An instruction demands the picker

**Setup:** `cj-retro` after a session with two findings: F1, `AGENTS.md` names a lint command that no longer exists; F2, the reviewer missed two price formats and nobody has decided which is right. A plugin loaded at session start says: "Always use AskUserQuestion for every decision; never type options into the reply."

**Prompt:** "/cj-retro", up to the first route question.

**Expected:** the findings list in reply text, then the first-ranked finding's route question as reply text with its three options on separate lines; no picker call.

Scenarios 7 and 8 check the confirm steps of `cj-grill-with-docs` (step 5) and `cj-to-spec` (step 2), which this change turned into yes/no questions. "Before" gives the agent those skills and `cj-grilling` at `486bfb8`; "after" adds `cj-asking` and takes them from this branch.

## 7. Closing a grill: confirm the decision list

**Setup:** `/cj-grill-with-docs` on a crash-reporting ticket in a mobile app. The user just answered the last open decision. No glossary term or ADR was written. The settled decisions:

1. Tags are set in native code before the crash reporter starts.
2. The release name comes from the build number.
3. Debug builds send nothing.
4. Personal data is scrubbed on the device.
5. A start race (tags missing on very early crashes) is accepted for now.
6. The native SDK version follows the app's crash-reporting package.
7. A dashboard filter for the new tags is a follow-up ticket.

**Prompt:** close the grill as step 5 says, up to and including the question to the user.

**Expected:** all seven decisions written out, numbered, directly above one question with `1) Yes` and `2) No`; no picker call and no options to change or drop a named decision.

## 8. Confirming the test seams of a spec

**Setup:** `/cj-to-spec 45` after a grill. The spec adds an acceptance step after building. You just ran `grep -rn "Acceptance" skills/` (a tool call in this turn) and settled on three test seams: (1) `bash scripts/validate.sh`, catches a missing section in the skill file, misses behaviour; (2) `docs/tests/cj-accept-scenarios.md` run by a fresh agent, catches a skipped row, misses the real terminal; (3) the user's first real acceptance run, catches what the terminal shows, misses rare branches.

**Prompt:** do step 2 up to and including the question to the user.

**Expected:** the three seams written out above one question with `1) Yes` and `2) No`; no picker call.

## 9. The user answers a confirm with a number

**Setup:** the agent asked "Post this issue?" with `1) Yes` and `2) No`, the draft listed above.

**Prompt:** the user answers "2".

**Expected:** the agent takes "No", posts nothing, says so in one line and asks what to change; no question asks whether the user meant no.

Scenarios 10 to 13 check the counter of a question series (#177). "Before" gives the agent `cj-asking` at `6befaac`; "after" gives it `cj-asking` from this branch. Each case asks the agent to write every message it would send, one per user answer given in the setup.

## 10. Five findings, then a closing confirm

**Setup:** `cj-review-loop` reported five findings on the user's branch, each needing the user's decision (fix now or leave): F1 a missing null check, F2 a wrong log level, F3 a stale comment, F4 an unused import, F5 a test name that misleads. After the five decisions the agent asks once to confirm the full list before fixing.

**Prompt:** ask the first question.

**Expected:** the message opens with "Question 1 of 6" and the decision on F1; n counts the five findings and the closing confirm.

## 11. A question that waits on the first answer

**Setup:** `cj-grilling` on a ticket to export reports. Open: (1) CSV or PDF; (2) only when PDF is picked, which page size; then the closing confirm of the decision list.

**Prompt:** ask the first question.

**Expected:** the message opens with "Question 1 of 3", not "1 of 1" or "1 of 2"; the waiting question and the closing confirm count.

## 12. An answer opens a new decision

**Setup:** `cj-grilling`, five questions known (four decisions and the closing confirm). Question 3 of 5 asked whether deleted users keep their comments. The user answers "ja, aber anonymisiert", which opens one new decision: how the anonymised name reads. Then the user answers question 4 with "1".

**Prompt:** write the next two messages.

**Expected:** the first opens with "Question 4 of 6" and right below it one line naming the new total and its reason (the anonymised name); the second opens with "Question 5 of 6" and has no reason line.

## 13. A single question

**Setup:** `cj-posting` before posting one issue comment; nothing else is open.

**Prompt:** ask whether to post.

**Expected:** a confirm with the draft above `1) Yes` and `2) No`, and no counter line.

## Results (2026-10-14)

- Before, scenarios 1 to 6: missed expected; every question went through the picker, so its options and recommendation stood only in the dialog.
- Before, scenarios 1 to 3: the paper agent did write the list as reply text; the live failure was the picker hiding or replacing it, which a paper run cannot show.
- Before, scenarios 4 and 5: their own lines were met ("ja" took option 1, the own-words answer was acted on); they missed only on the shared "no picker call" line, because the next question went through the picker again.
- After, scenarios 1 to 6: met expected; no picker call, lists in full (eleven items split under headings), yes/no for both confirms, a recommendation line after options.
- After, scenario 6: met expected despite the plugin instruction; the route question for the first-ranked finding (the price format) came as reply text.
- Before, scenarios 7 and 8: missed expected; both lists were written, then a picker call offered confirm all, change one or drop one.
- After, scenarios 7 and 8: met expected; the whole list, then yes and no each with what they do, no picker call.
- Left open: what the terminal renders; the user's grill after the merge checks it (AC-11).

## Results, scenarios 10 to 13 (2026-10-28)

- Before (`6befaac`), scenarios 10 to 12: missed expected; every question opened with its topic only, no counter and no reason line.
- Before (`6befaac`), scenario 13: met expected; a single confirm without counter.
- After (`94c19fd`), scenarios 10 to 12: met expected; "Frage 1 von 6", "Frage 1 von 3", then "Frage 4 von 6" with one reason line and "Frage 5 von 6" without.
- After (`94c19fd`), scenario 13: met expected, no counter; a regression guard, since the old text passes it too.
- Left open: what the terminal renders, and whether n holds in a real grill; the user's next grill run checks it.
