# cj-filing-issues scenarios

Target: Fortress Architect (no issue templates, no release tags, no impact labels). Dry runs on 2026-10-07 by fresh subagents; nothing was created on GitHub.

## 1. Bug with the old cj-bug-report-to-issue (RED)

Input: "Nach einer Niederlage und 'Erneut versuchen' springt die freigeschaltete Welle auf Welle 1 zurück, wenn das Spiel zwischendurch neu gestartet wurde."

Found #37 (closed, same area) and #199 (open refactor of the same code) and argued sensibly against edges, but on its own: no fixed classes, no direction rule. The body listed four internal functions under "Code", carried a "since when: unknown" filler block, and had no rule for marking where the steps came from.

## 2. Same bug through cj-bug-report-to-issue + cj-filing-issues (GREEN)

- Find table with a class and reason per match: #199 related (order unclear, direction rule), #198 not a parent (epic excludes behaviour changes), #37 related (different trigger; release state unknown without tags).
- Body 258 words, BODY.md checks run, `unslop` applied; steps marked "as reported"; the code section says plainly that the read code does not yet explain the symptom.
- Unclear points fixed afterwards: no release tags, no template, origin of steps, tier without `core-flows`.

## 3. Task straight through cj-filing-issues (GREEN)

Input: "Spätere Karten sollen pro Welle mehrere Eintrittsrichtungen gleichzeitig nutzen können."

- Problem in three sentences without the solution; non-goals with reasons; six acceptance criteria, each checkable by someone else ("an enemy breaking through from the east: the diagnosis names east, not north").
- Find kept eight matches, all related; no edge set, because nothing has to land first.
- Checked the user's premise against the spec and found it softer ("dürfen", later maps after release).
- Unclear points fixed afterwards: unrelated matches, headings without a template, judgement labels, open questions in a task, premise check, links to docs vs code paths.
- Note: it used the old term "Eintrittsbereich" because the local Fortress checkout was behind `origin/main`, not because of the skill.
