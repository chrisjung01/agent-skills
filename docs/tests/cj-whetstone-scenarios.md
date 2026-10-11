# Scenarios: cj-whetstone

Paper exercises built from real whetstone data (October 2026). The agent under test describes what it would do; it does not edit files or run `status`. Each scenario runs once without the skill (baseline) and once with it. Setups name what `skill-metrics open` and `show` print; until #44 the same data came from markdown notes.

## 1. Calls without corrections

**Setup:** `open cj-issue-to-draft-pr` lists 7 calls. In `show`, six have user texts without a `correction` flag, one has tool errors (Bash ×2, EnterWorktree ×1).

**Prompt:** "Verbessere bitte cj-issue-to-draft-pr anhand der gesammelten Daten."

**Baseline:** Opened only the call with tool errors, set the six others to `verworfen` unread ("absence of a signal").

**Expected with skill:** Full scan first; `show` read for all 7 calls; ledger with one row per call; change proposed only for a pattern in 2+ sessions; no edit before approval.

## 2. Approval flagged as correction

**Setup:** The `unslop` call in session `7d81b90c` has the user text `prompt [correction]: nein setz das gerne um`. In the transcript the agent had asked for spec approval; the next user prompt "die umsetzung braucht sehr lange" is about the implementation, not about unslop.

**Prompt:** "Schau mal was die Notizen zu unslop sagen und bau das ein."

**Expected with skill:** "nein …" classified as noise (approval); "dauert lange" moved to the `cj-issue-to-draft-pr` ledger; unslop unchanged; correction regex fix suggested (ROUTING.md).

## 3. One-off experiment, user in a hurry

**Setup:** The `cj-stacked-prs` call in session `b3a2eeba` has no flagged user text, only tool errors. Its user texts are "lass mal um den Spec zu schreiben bitte fabel nehmen …" and then "ne nicht als präferens ablegen, ich teste das nur einmal".

**Prompt:** "Mach cj-stacked-prs besser, die Daten liegen in whetstone. Schnell bitte."

**Expected with skill:** Verdict `one-off`; no skill change, no memory; tool errors classified (declined prompt, failed `cat` of a skill path = noise); `status -- <start uuid> verworfen "one-off"` proposed.

## Results (2026-10-05)

- Baseline (scenarios 1–3, no skill): sensible on 2 and 3 when the transcript content was handed over, but in 1 it closed six entries unread and would never have opened the transcripts that hold the real criticism.
- With skill, run 1 on the real notes of `cj-issue-to-draft-pr`: all 7 entries read, no change invented. Found that the script ended the window at the nested `unslop` call, so implementation and review were never visible. Fixed: the window now runs across nested skill calls.
- With skill, run 2: found the "too slow, checks only at the end" pattern (2 sessions plus an explicit rule), saw it was already fixed in `8b84a8a`, proposed no change and status lines for every entry. Leftover ambiguities (duplicate sessions, single observations, repo root for `git log`) fixed in the text.

## By session, removed skills, status vocabulary (2026-10-07)

Evidence from the whetstone run of 2026-10-06 on `cj-issue-to-draft-pr` and `unslop`:
- the same 13 sessions sat in both notes files and every transcript was read twice
- 26 windows did not fit one context; two read-only subagents were used without the skill saying so
- a subagent proposed a status word outside the vocabulary ("erledigt")
- the worktree double switch could not be reproduced by a subagent, so the "scenario must fail first" step could not run
- two notes of the removed `cj-pr-review` had no owner

Change: the open list groups entries by session and marks removed and outside-repo skills; a check enforces the status vocabulary (both replaced in #44: `open` grouped by its session column, `git log --diff-filter=D` for removed skills, `status` refusing other words); the skill reads by session, fans out above five sessions, routes removed skills to their successor, and allows "verify in next real run" for failures a subagent cannot reproduce; the window printer gained untruncated agent text (`show` prints it in full).

Run on the real notes, read-only, by a fresh subagent: 8 in-scope sessions (16 open entries over 11 sessions; three outside-repo-only sessions skipped as asked), each read once, including the one that touched two skills; both `cj-pr-review` notes routed to `cj-review-loop` via the removal commit; every proposed status line passed the vocabulary check. No pattern found, so no skill change was proposed. Its unclear points (status for "no signal", outside-repo entries inside a read session, successor when a commit split a skill, truncated agent text, window size) are now answered in the skill.

## Binary as the only collector (#44)

**Setup:** A session was killed (no SessionEnd hook ran), another was resumed after the hook had stored it.

**Prompt:** "/cj-whetstone cj-implement"

**Expected with skill:** `bash scripts/skill-metrics.sh` runs before `open`; both sessions appear with their latest window; statuses are set with `status`, never by editing files. The binary side is covered by `full_scan_stores_killed_and_resumed_sessions_with_their_latest_window` in `skill-metrics/tests/cli.rs`.

## Approved retro finding (2026-10-11)

**Setup:** `open cj-implement` lists 13 calls; none has a user text about the size rule. In session `4ffad4ea` a `cj-retro` call reported "cj-implement: the 400-line size rule counts moved lines", quoted the moment and the skill line, and the user answered "Alle (Recommended)" to "Welche Retro-Punkte soll ich umsetzen?".

**Prompt:** "/cj-whetstone cj-implement"

**Baseline:** Did not read the `cj-retro` call ("no rule makes me read it"), counted the finding as agent text, proposed no change; noted that the skill gives no path for retro findings. In the real run the user asked why the retro recommended a whetstone run that could never act on it.

**Expected with skill:** Reads the `cj-retro` window of that session, counts the approved finding as one piece of evidence for `cj-implement`, and proposes the smallest change for it (or says why it is already fixed).

**With skill (2026-10-11):** Read the `cj-retro` window from `closed cj-retro`, gave the finding its own ledger row (defect, §3 Size), counted it under the approved-retro rule, and checked `git log` for an earlier fix before proposing a diff.
