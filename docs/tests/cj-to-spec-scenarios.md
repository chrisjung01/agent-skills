# Scenarios: cj-to-spec

Skill under test: `cj-to-spec` (with `SPEC.md`).

## 1. Render criteria name the sizes they must hold at

**Setup:** repo `acme/clock-tui`, a terminal UI. Its main view splits the terminal width: from 100 columns on it has three columns (40 % list, 30 % form panel, 30 % calendar), below 100 two columns of 50 %. Existing render tests draw a single panel into a test frame of any size and check cells. Issue #52 "Absence form: type as pills" is grilled; the decision list says: the four types `Sick` `Vacation` `Holiday` `Other` as pills, the selected one black on yellow, the others grey; digits 1–4 select a type while the type field has focus; the hint reads `←/→ or 1–4 to select, Enter to confirm`. The form panel is the 30 % column.
**Prompt:** "/cj-to-spec #52" (draft only, do not post).
**Expected:** every AC about what the panel shows names the terminal widths it must hold at (at least one width per layout, for example 99 and 140 columns), and the render seam draws through the app's real layout at those widths, not a panel in a frame of the test's choosing. A draft whose render ACs give no width, or whose seam renders the panel alone in a frame of free size, fails.
**Origin:** a real run passed AC-1 ("all four pills visible") in a 100-wide test frame, while the app gave the panel 40 columns at a 140-column terminal and cut `Other` to `Oth`; only a manual terminal run found it.

## 2. Same rule for a web layout

**Setup:** repo `acme/shop-web`, a web app. Its CSS breakpoints: below 640 px one column, from 640 px a sidebar of 240 px plus content, from 1024 px a third column for the cart. Component tests mount one component in a container of any width. Issue #77 "Product card: price and stock badge in one row" is grilled; the decision list says: price left, stock badge right, in one row; the badge reads `In stock`, `Few left` or `Sold out`.
**Prompt:** "/cj-to-spec #77" (draft only, do not post).
**Expected:** the ACs about the card's row name the viewport widths they must hold at, taken from the app's breakpoints (one per layout, for example 375, 800 and 1280 px), and the seam renders the page or layout at those widths, not the card alone in a container of free width. No terminal-specific wording carried over.

## Runs

- 2026-10-07, before the change: scenario 1 three runs, scenario 2 one run. All four rendered the panel or card alone in a frame or container of free size; one of the TUI runs named a single layout only. Fail.
- 2026-10-07, after the change: scenarios 1 and 2 two runs each. All four named one size per layout and rendered through the real layout, and called the free-size frame insufficient. Pass.

## 3. The closing reply tells the user what to type next

**Setup:** the spec for #5091 is posted; it needs three tickets. The conversation is in German.
**Prompt:** "ja, posten" (write the reply after posting).
**Expected:** the reply ends with an instruction: run `/cj-to-tickets <issue>` now, in this session, without `/clear` first. A line that only names the command and "in this same session" fails.
**Origin:** session 8d055489 (same wording in `cj-grill-with-docs`).
- 2026-10-09, scenario 3 before the change: one run. "Nächster Schritt: `/cj-to-tickets …` hier in dieser Session." No word on `/clear`. Fail.
- 2026-10-09, scenario 3 after the change: one run. "Nächster Schritt: Führ jetzt `/cj-to-tickets …` aus, in dieser Session. Vorher kein `/clear`." Pass.

## 4. A one-ticket spec ends with `/clear`, then `cj-implement`

**Setup:** issue #6031 exists; the grilled spec fits one ticket in one repo. The conversation is in German.
**Prompt:** "ja, posten" (write the reply after posting).
**Expected:** the spec goes out as a comment on #6031, no new issue and no `cj-filing-issues`. The reply ends with an instruction: `/clear`, then `/cj-implement <issue>`. A reply that names `/cj-implement` without `/clear`, or files an issue, fails.
**Origin:** 2026-10-09, a user asked why the flow skipped `cj-filing-issues` and got no word on `/clear`.

## 5. A number in an AC comes with its counting definition

**Setup:** repo `acme/notes-cli`. Issue #88 "Warn before a choice prompt that has no context line" is grilled. Early in the conversation the agent ran a quick scan, `scripts/scan_prompts.py logs/`, which marks a choice prompt as "with context" when any text line appears earlier in the same turn → "120 of 300 choice prompts had no context line". The decision list later adds an edge-case row: "context line, then a tool call, then the prompt → counts as no context". The quick scan counted that case as having context.
**Prompt:** "/cj-to-spec #88" (draft only, do not post).
**Expected:** an AC that states a baseline names the command or query that produced it, and the number is recomputed with the spec's own definition (or the draft says it must be recomputed before posting and does not post 120 as the baseline). A draft that writes "baseline 120 of 300" with no definition or command fails.
**Origin:** sessions 5752094e and 254a75c0: AC-8 said 170 of 311 from an ad-hoc scan; under the spec's definition it was 206; it cost a question to the user and a spec change.

## 6. A check mechanism is tried against the case it must catch

**Setup:** repo `acme/tower-game`, a Godot game. Issue #219 "Tests leave save files behind" is grilled. The decision list says: "The runner contract counts the files in the user directory before and after the full test run and fails on a difference, listing the new files." The conversation also notes that some tests write files with fixed names (`settings.cfg`, `profile.onboarding`) and that about 200,000 leftovers from earlier runs already sit in that directory.
**Prompt:** "/cj-to-spec #219" (draft only, do not post).
**Expected:** before the draft fixes the counting mechanism, it checks it against the hard case: a file with a fixed name that an earlier run already left there is rewritten, so the count does not change and the leak goes unseen. The draft either picks a mechanism that catches it (for example a timestamp marker before the run and every file written since), or names the gap and sends the decision back to grilling. A draft that fixes "count before and after" without a word on fixed-name files fails.
**Origin:** sessions b21b5e0f (spec) and 0648b81d (implement): the review found the gap (F1), the implementation switched to a timestamp marker, retro 3b92833d.
- 2026-10-11, scenario 5 before the change: two runs (the first with an obviously wrong `grep`, the second with a plausible scan whose definition differs). Both noticed the mismatch and did not post the baseline. Pass already; the real run had the definition only implicit in an earlier tool call. Verify in next real run.
- 2026-10-11, scenario 6 before the change: one run. Kept "count before and after" and claimed in an edge-case row that a fixed-name file written by a test is caught. Fail.
- 2026-10-11, scenario 6 after the change: one run. Held the draft back: an overwritten fixed-name file leaves the count unchanged; proposed name-plus-mtime snapshot or an empty user directory and asked for one try on the hard case. Pass.

## 7. Quantities are checked against limits the repo already states

**Setup:** a game repo with 16 plan folders under `docs/plans/`. One of them, `issue-100-30-wellen-kampagne/decisions.md`, says "P8: A wave spawns at most 100 enemies in total", backed by `docs/performance-load-test.md` (100 active enemies, more not measured). Enemy rewards: runner 3, raider 5. The grilled decision list for #284 "Wellen-Generator mit Seed": budget per wave 25 + 11·(n−1), drawn until nothing fits, no last wave, wave 50 must be playable. The user says "Das Grilling ist durch und alle Entscheidungen sind abgenickt, bitte jetzt nur noch die Spec entwerfen, kurz halten."
**Prompt:** "/cj-to-spec #284" (draft only, do not post).
**Expected:** the draft names the 100-enemy limit, either kept as a decision or AC, or as a conflict that sends the budget back to grilling. A draft that posts an unbounded budget with "wave 50 playable" and no word on the limit fails.
**Origin:** session fdb7349a (spec for #284) posted the unbounded budget; the review of the implementation found the conflict with P8 and the spec gained AC-34 afterwards. Retro finding approved in session fad331f6.
- 2026-10-12, scenario 7 before the change: three runs (one in a 5-file repo, two in the 25-file repo with the time-pressure line). All three found P8, computed when the budget passes 100 enemies and held the draft back. Pass already; the real run had grilled in an earlier context and read only SPEC.md, issue-tracker.md and checks.md before drafting. Verify in next real run.

## 8. Test seams list each catch and miss on its own line

**Setup:** issue #84 "Wellen-Generator mit Seed" is grilled. Seam A (the generator in the test runner) catches seven things, from "same seed, same waves" to "remaining budget smaller than the cheapest group ends the wave", and misses balance and game feel. Seam B (the game scene via GameSceneHelper) catches three things and misses layout at 720×1280. The conversation is in German.
**Prompt:** step 2 of `/cj-to-spec #84`: show the seams and ask for confirmation.
**Expected:** each seam heading is bold with its location in parentheses on the same line, and catches and misses are sub-lists with one case per line. A seam whose catches sit on one line separated by commas or semicolons, or several cases on one line, fails.
**Origin:** session fdb7349a: the user called the seam list hard to read twice and said "so sollte das immer aussehen" for the nested layout; agent-skills#44.
- 2026-10-12, scenario 8 before the change: one run. Catches as three `fängt:` lines with up to three cases joined by semicolons. Fail.
- 2026-10-12, scenario 8 after the change: one run. Bold heading with location in parentheses, catches and misses as sub-lists, one case per line (seven catches for seam A). Pass.

## 9. A fresh reader tries to break the draft before it is shown

**Setup:** repo `acme/booking-api`. Issue #140 "Cancel a booking up to 24 hours before start" is grilled. The decision list says: a customer can cancel until 24 hours before the booking starts; later the cancel button is gone and the API answers 409; a cancelled booking frees its slot; staff can cancel at any time. The draft so far has AC-1 "Given a booking that starts in 48 hours, when the customer cancels, then the booking is cancelled" and AC-2 "Given a booking that starts in 2 hours, when the customer cancels, then the API answers 409". Nothing in the conversation says what happens at exactly 24 hours or whether the slot is freed for staff cancellations too.
**Prompt:** "/cj-to-spec #140" (draft only, do not post).
**Expected:** before the draft is shown, a subagent got only the issue and the draft and looked for ACs a wrong implementation still passes (for example a 12-hour cutoff passes AC-1 and AC-2; a cancel that never frees the slot passes both). Below the draft, every finding has one line, worked in or rejected with its reason. The open boundary at exactly 24 hours goes to the user as a question, not into the draft as a guess. A draft shown without a reader's findings, or with the boundary decided silently, fails.
**Origin:** agent-skills#61, adapted from addyosmani/agent-skills `doubt-driven-development`.

## 10. Merge danger says whether old clients stay valid

**Setup:** repo `acme/fit-backend`, the API behind a mobile app. Issue #310 "Workout type: replace `cardio` with `running` and `cycling`" is grilled. The decision list says: the `type` field of `GET /workouts` returns `running` or `cycling` instead of `cardio`; existing rows are migrated; the current app release already knows the new values. Nothing in the conversation mentions older app versions.
**Prompt:** "/cj-to-spec #310" (draft only, do not post).
**Expected:** the Merge danger line says whether old clients and old code still running during the rollout stay valid, here that app versions without `running` and `cycling` get a value they do not know. A Merge danger line that names door and blast radius but says nothing about old clients fails.
**Origin:** agent-skills#61, adapted from addyosmani/agent-skills `deprecation-and-migration`.
- 2026-10-13, scenarios 9 and 10 before the change: one run each. Scenario 9 dispatched no reader and found the 24-hour boundary through its own checks, with no finding lines. Scenario 10 wrote Merge danger as door and blast radius only, without old app versions. Fail.
- 2026-10-13, scenarios 9 and 10 after the change: one run each. Scenario 9 sent only issue and draft to a reader, worked in ACs at 25 and 23 hours, and asked the 24-hour boundary and staff-slot findings as questions. Scenario 10's Merge danger named older app releases and old backend instances. Pass.
- 2026-10-13, scenario 9 again on the text after the review fixes, three turns: the reader's prompt held task, issue and draft; two findings worked in, two asked through `cj-grilling` one per turn; checks rerun after the answers; four `<finding> → worked in` lines below the draft. Pass.

## 11. A feature a user sees ends with acceptance by hand

**Setup:** repo `acme/shop-web`. Issue #120 "Merkliste: Artikel per Herz speichern" is written in German and grilled in German; the decision list says: a heart on every product card saves the item, the saved items show on a new page "Merkliste", one ticket.
**Prompt:** "/cj-to-spec #120" (draft only, do not post).
**Expected:** the draft's last line reads `Acceptance: by hand · behaviour a user sees`, in English although the headings are German. A draft without the line, or with `verified is enough`, fails.

## 12. A data migration no user sees ends with acceptance by hand

**Setup:** repo `acme/fit-backend`. Issue #318 "Store workout durations in seconds instead of minutes" is grilled: a migration rewrites every stored duration, the API keeps answering in minutes, no screen changes, one ticket.
**Prompt:** "/cj-to-spec #318" (draft only, do not post).
**Expected:** the last line reads `Acceptance: by hand · data`. A draft that proposes `verified is enough` because no user sees a change fails.

## 13. A big refactor without a trigger ends with verified is enough

**Setup:** repo `acme/notes-cli`. Issue #92 "Split the storage module into reader and writer" is grilled; the decision list says: no behaviour changes, no stored format changes, five tickets, about two weeks of work, big tier.
**Prompt:** "/cj-to-spec #92" (draft only, do not post).
**Expected:** the last line reads `Acceptance: verified is enough · no trigger applies`. A draft that sets by hand because of the tier, the effort or the ticket count fails.

## 14. The Acceptance line names its trigger

**Setup:** as scenario 11. An earlier draft in the conversation ends with `Acceptance: by hand`, without a trigger.
**Prompt:** "Prüf den Entwurf gegen die Checks und zeig ihn mir" (draft only, do not post).
**Expected:** the checks catch the missing trigger; the draft shown ends with `Acceptance: by hand · behaviour a user sees`. A draft shown with the bare `Acceptance: by hand` fails.

## 15. The user overrides the proposed value

**Setup:** as scenario 11; the draft ends with `Acceptance: by hand · behaviour a user sees`.
**Prompt:** "Mach verified is enough daraus, das teste ich nicht von Hand" (draft only, do not post).
**Expected:** the draft shown ends with `Acceptance: verified is enough · behaviour a user sees · user chose`. A line that drops the trigger or omits `· user chose` fails.
- 2026-10-14, scenarios 11 to 15 before the change (#95): one fresh grader. No line of `SPEC.md` or `cj-to-spec` adds an Acceptance line, so all five missed expected. Fail.
- 2026-10-14, scenarios 11 to 15 after the change: one fresh grader. All five met expected, citing the Acceptance line section and its check in `SPEC.md`. Pass.

## 16. A rule that applies in many places lists them by what they do

**Setup:** repo `acme/agent-skills`, twelve skills. Issue #120 "Approvals are yes/no" is grilled; the decision list says: every approval or confirm question to the user is a yes/no question with the whole list above it, never a picker and never options like "all, pick per item, none". The conversation notes these lines from the skills:
- `cj-posting`: "Ask one question per kind of action, each in the picker: all, pick per item, none."
- `cj-retro`: "Use the picker for each route question."
- `cj-grill-close`: "Ask to confirm the list in the picker: confirm all, change one, drop one."
- `cj-review-pr`: "First which findings to post (all, pick per finding, none), then the form."
- `cj-spec`: "Ask the user to confirm the seams as one question with a choice: confirm all, change one, drop one."
- `cj-whetstone`: "Ask for approval per change: apply, change, reject."
**Prompt:** "/cj-to-spec #120" (draft only, do not post).
**Expected:** the draft lists every approval or confirm step, including `cj-review-pr`, `cj-spec` and `cj-whetstone`, which never say "picker", and an AC checks each listed place for the yes/no shape. A draft whose only coverage AC is "a search for picker terms finds none" fails.
**Origin:** spec #92 AC-3 searched for picker terms; the review of #109 then found such approvals in round 1 (`cj-posting`, `cj-review-teammate-pr`), round 2 (`cj-whetstone`, `cj-to-spec`) and round 3 (`cj-accept`). Same shape in #104 (AC-8 named three files; `AGENTS.md`, `README.md` and `WRONG-TURNS.md` followed in later commits) and #96 (the fresh reader found a `grep` that missed places without the name).

## 17. The ACs hold together

**Setup:** repo `acme/agent-skills`. Issue #146 "PR body template" is grilled; the decision list says: the body has five blocks (Why, What, How to test, Verified, Risk); the whole body is at most 15 lines; Verified holds one line per AC and per edge-case row of the spec. The spec being written has 7 ACs and an edge-case table of 4 rows. Why, What, How to test and Risk take at least 7 lines together.
**Prompt:** "/cj-to-spec #146" (draft only, do not post).
**Expected:** the draft works out that 11 Verified lines plus 7 other lines exceed the 15-line cap for this very spec, and either changes an AC with a new decision sent back to grilling (rows with the same evidence share a line, or the cap counts differently) or names the conflict as an open question. A draft that keeps both ACs unchanged without a word on the conflict fails.
**Origin:** spec #46, AC-2 and AC-4; review round 2 of the implementation found it (F9), the spec had to be edited (retro `b412d0b3`).

**Variants used for the runs:** 16b gives a fixture repo of eight skills and only the decision list, so the agent finds the places itself; three approval steps there never say "picker". 17b gives the eleven decisions of the PR body template without counting the ACs or the other blocks.

- 2026-10-14, scenarios 16 and 17 before the change: two runs each. All four met expected: the setup listed the places and the numbers. Rewritten as 16b and 17b.
- 2026-10-14, 16b and 17b before the change: two runs each. 17b met expected in both: each worked out the cap from the decisions and asked. 16b met expected in one run (an AC per place) and nearly in the other (one behaviour AC for all approvals, the places under Decisions). A paper run does not reproduce the failure; the real ones came late in long sessions (#92, #104, #47, #46).
- Status: verify in next real run. The next whetstone run for `cj-to-spec` checks later specs for an AC that only greps, and for ACs that contradict each other.
