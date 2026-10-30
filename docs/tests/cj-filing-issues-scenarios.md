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

## 4. A caller's "CI is green" goes into a bug unchecked (verify in next real run)

Source: a `cj-retro` session on 2026-10-29 filed chrisjung01/Last-Wick#153 with "CI ist auf demselben Stand grün" and "Ursache offen". It never ran `gh run view`; the log of run 40918091393 held the same 10 `ERROR:` lines, hidden by a test loop that only failed on its last test (Last-Wick#30).

Input (from the calling skill): "Issue in chrisjung01/Last-Wick: tests/camp_scene_test.gd lokal rot (10 ERROR-Zeilen) auf origin/main e155f1f, CI grün auf demselben Stand; lokal Godot 4.7.2, CI 4.7.1; Ursache offen. Retro-Befund, Kind bug."

Expected: File step 1 checks the CI claim in the run's log, finds the errors, and reports the mismatch to the user before drafting; the draft links the run.

Baseline, 2026-10-30, fresh subagent with the unchanged skill: passed. It read the log, found the 10 lines and the masking loop, linked the run and related #30. The real failure came at the end of a long retro session; no change to `SKILL.md`. Verify in the next real run: a whetstone run looks for issues that state a CI result without a run link.

## 5. Issue language for another repo (baseline passes)

Source: sessions on 2026-10-16 and 2026-10-24 where the user had to ask for an English issue.

Input: a session in Last-Wick (German issues) asks for an issue in chrisjung01/agent-skills (English issues).

Expected: English, from the target repo's `AGENTS.md`.

Baseline, 2026-10-30, fresh subagent with the unchanged skill: passed, citing `AGENTS.md` of agent-skills. No change to `SKILL.md`.
