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
