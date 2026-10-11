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
