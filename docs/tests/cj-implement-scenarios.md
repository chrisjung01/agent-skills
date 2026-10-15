# Scenarios: cj-implement

Paper exercises built from real sessions. The agent under test gets the skill text and the tool output below and describes its next commands; it runs nothing. The flow-wide scenarios are in [spec-driven-flow-scenarios.md](spec-driven-flow-scenarios.md).

## 1. Resume: the ticket's branch link is gone, its PR is open

**Setup:** spec #18, tickets #26 → #22 → #23 (blocked-by chain), mode `per-ticket`. An earlier run created `feat/26-skill-metrics-skeleton` with `gh issue develop 26` and opened draft PR #28 (`Closes #26`, CI green). Since then GitHub dropped the branch link under "Development". Tool output in this run:

```
$ for n in 26 22 23; do echo "#$n: $(gh issue develop --list $n)"; done
#26:
#22:
#23:
```

**Prompt:** "/cj-implement #18 per-ticket". The agent names only the commands step 2 asks for and says what each returns in this setup.

**Expected:** #26 counts as done through PR #28, found by the skill's own lookup and not by luck; no second `gh issue develop 26`. The next unit is #22, stacked on PR #28 (base `feat/26-skill-metrics-skeleton`).

**Source:** sessions `765a26e3`, `27597af8`, `557480b2`, `40075e28` (2026-10-08): `gh issue develop --list 26` was empty in all four while PR #28 was open; each agent found the PR only through an extra `gh pr list` the skill did not ask for.

## 2. Fold review fixes, then push

**Setup:** ticket #21, branch `feat/21-bare-question-hook`, never pushed except the empty branch `gh issue develop` created. Review findings F5 and F9 are fixed in the working tree, checks are green, the verification table is done. Tool output:

```
$ git log --oneline origin/main..HEAD
f341d48 fixup! skill-metrics: count bare questions per skill call and show their rate (#21)
9f7f845 cj-whetstone: PreToolUse hook blocks bare questions, install.sh prints its line (#21)
57c4b68 skill-metrics: count bare questions per skill call and show their rate (#21)
01770d8 glossary: bare question, and as a kind of rate (#21)
$ git status --short
 M install.sh                                   # F5: line 25 `sed -n '2,12p'` -> `sed -n '2,11p'`
 M skills/cj-whetstone/skill-metrics/src/report.rs   # F9: rewrap one paragraph
$ git log -L25,25:install.sh --oneline | head -1
9f7f845 cj-whetstone: PreToolUse hook blocks bare questions, install.sh prints its line (#21)
```

**Prompt:** "Commit the fixes, fold them and push." The agent writes the exact commands up to and including `git push`, in order, and says what makes it stop before the push.

**Expected:** the `install.sh` fix is a fixup of 9f7f845 (the commit that last changed the line), not of 57c4b68. The old HEAD is saved before the rebase. `git diff <old-HEAD> HEAD` runs as its own command, not chained with `&&` to the push; a non-empty diff stops the push and leads to reset, retarget and a new fold.

**Source:** sessions `254a75c0`, `85f8e9bc`, `ebc90e1c`, `e3c882d1` (2026-10-11). In 254a75c0 fixup, autosquash, `git diff --stat` and push ran in one `&&` chain; the fixup of 57c4b68 undid the F5 fix and the regression was pushed. The others skipped the diff or did not let it gate the push.

## 3. Mutation check during the build

**Setup:** ticket adds a rule to `scripts/validate.sh` that rejects a skill table row without a purpose. The agent has edited `AGENTS.md` (new table row) and `scripts/validate.sh`; neither is committed. It wants to see the new rule fail on a broken `AGENTS.md`, then restore.

**Prompt:** "Show that the new check catches a missing purpose." The agent writes the exact commands for the mutation, the check run and the restore, and the `git status` it expects afterwards.

**Expected:** the edits are committed (or the tree is otherwise clean) before the mutation; the restore leaves `git status` clean with the agent's edits intact. Never `git checkout HEAD -- AGENTS.md` over uncommitted work.

**Source:** session `ebc90e1c` (2026-10-11), approved cj-retro finding: `git checkout HEAD -- AGENTS.md` during a mutation run reverted the agent's own edit; only `git diff --cached` caught it.

## 4. PR body that does not match the diff

**Setup:** ticket #282 "Run summary shows the best wave", branch `feat/282-run-end-bestwelle`, checks green, review done, no stack. Diff: `scripts/run_summary.gd` (+31 −4), `tests/test_run_summary.gd` (+22, test `test_best_wave_shown`), `scenes/run_end.tscn` (+6 −1). The agent has drafted this body:

```
Why: #282 closes the gap from spec comment <link>: the run summary shows the best wave.
What:
- Run summary shows "Best wave: N" under the score
- Best wave persists in the save file (`save_game.gd`)
- Summary scene has one more label
How to test: run the project, lose a run.
Verified:
- AC-1 test_best_wave_shown: met
- AC-2 best wave survives a restart: met (manual, save file shows `best_wave`)
- Edge: first run ever, label reads "Best wave: 1": met (manual)
- Checks: `godot --headless -s tests/run.gd` green
- Review: 2 findings fixed, 0 open
Risk: none
Closes #282, Part of #280
```

**Prompt:** "Open the draft PR." The agent names what it does before any push, and whether the PR opens.

**Expected:** no PR yet. The agent reads the body against the diff and finds two claims without backing: `save_game.gd` is not in the diff, and nothing in the diff writes a save file, so the manual check under AC-2 cannot have seen what it says. Step 7 does not catch either, since a manual check with an observation counts as evidence there. How to test is one line without an expected result, not numbered steps. The agent fixes the body (drops or corrects the two claims, writes numbered steps with results) or names the missing commit, then reads again. A run that opens the PR as drafted fails.

**Variants**, same setup, one at a time:
- The body has no How to test block at all. Expected: no PR; the agent adds numbered steps with expected results, or the one command that shows the change.
- The body is true: How to test has five steps, Verified has eight AC and edge rows plus checks and review, 23 lines in all counted as PR-BODY.md counts. Expected: no PR; the agent says the unit is too big and proposes a split, as it would at 400 changed lines. Trimming Verified below one line per piece of evidence is wrong.

**Source:** spec #46 (2026-10-12): a PR body that names behaviour outside the diff is the common failure of agent PRs; nothing in the old shape checked the body against the diff.

## 5. Mutation evidence in the Verified block

**Setup:** ticket #90 "Reject a skill table row without a purpose", per-ticket, no stack. AC-1: given an `AGENTS.md` row with an empty purpose cell, `scripts/validate.sh` exits 1 and names the row. AC-2: the README's Checks section names the new rule. Diff: `scripts/validate.sh` adds `if [ -z "$purpose" ]; then fail "$row: no purpose"; fi`, `tests/validate_test.sh` adds `empty_purpose_fails` (fixture row with an empty purpose cell, asserts exit 1 and the row name in the output), `README.md` one line. Everything is committed, the tree is clean, checks are green, the review is done.

**Prompt:** "Write the Verified block." The agent names the commands it runs for it, then writes the block.

**Expected:** before writing, the agent inverts `-z` to `-n` on the clean tree, runs `empty_purpose_fails`, sees it red and restores; `git status` is clean. The AC-1 line names the test, the inverted condition and "red", in the form `empty_purpose_fails: AC-1 met, inverted -z → -n → red`. The AC-2 line names its check (the README line) and no mutation. A block whose AC-1 line has no inverted condition missed expected.

**Variant**, same setup: the mutation leaves `empty_purpose_fails` green, because its fixture row also breaks another rule that exits 1 with the same row name. Expected: no PR yet; the agent writes a test whose fixture breaks only the purpose rule, reruns the mutation until that test is red, and the AC-1 line names the new test. Calling AC-1 met on `empty_purpose_fails` missed expected.

**Source:** spec #61 (2026-10-13), from addyosmani/agent-skills `code-review-and-quality`: a test never seen failing is no evidence; the old step 2 allowed a mutation check but never asked for one.

## 6. Reproduction test for a bug

**Setup:** ticket #91, a bug: "`./install.sh --dry-run` lists `example-skill`, which must never be installed. Steps: run `./install.sh --dry-run`. Expected: no `example-skill` line." Spec comment on #91 with AC-1 "the dry run lists no `example-skill`". The harness offers subagents. Reading `install.sh`, the agent has already seen that the loop over `skills/*/` lacks the `example-skill` filter.

**Prompt:** "/cj-implement #91". The agent names its steps from the branch up to the first commit, including the full prompt of any subagent it dispatches.

**Expected:** before any fix, a fresh subagent gets the issue text and the repo, and no diagnosis, no line number and no fix; it writes the reproduction test. The agent runs that test, sees it red, then writes the fix. A subagent prompt that names the missing filter, or the agent writing the test itself while subagents are available, missed expected.

**Variants**, same setup, one at a time:
- The subagent's test is green before the fix. Expected: the bug is not reproduced; the agent stops and reports, and writes no fix.
- The same bug arrives as a fix-now row of the latest `## Acceptance` comment on spec #89, whose last open PR is #95. Expected: the same rule; the subagent's prompt holds the row and the user's observation, no fix.
- The issue body was filed by `cj-bug-triage` and also holds "Suspected: `install.sh:47`, the loop over `skills/*/` has no filter" and "Regression test: dry run output has no `example-skill`". Expected: the subagent's prompt holds the symptom, steps, expected and actual result, and neither of those two lines.

**Source:** spec #61 (2026-10-13), from addyosmani/agent-skills `test-driven-development`: an agent that already knows its fix writes a test that passes for the fix, not one that shows the bug.

## 7. Bug without a subagent

**Setup:** as scenario 6, but the harness offers no subagent. The agent wrote the reproduction test itself, saw it red, fixed the bug; checks are green. The user ran the `cj-reviewing` brief in a fresh session and pasted its findings; all are fixed.

**Prompt:** "Write the PR body's Risk line."

**Expected:** the Risk line says the implementing agent wrote the reproduction test itself, because no subagent was available. A Risk line of "none", or one that leaves out who wrote the test, missed expected.

**Source:** spec #61 (2026-10-13): the reader of the PR needs to know the reproduction test was not independent.

## 8. Auto-merge for a small PR

**Setup:** repo whose `conventions` say "small agent PRs (under 100 changed lines) may auto-merge"; `main` lists required status checks. Ticket #110 is done and checked; the spec's Acceptance line reads verified is enough; review is clean. `git diff origin/main --shortstat` reads `3 files changed, 40 insertions(+), 12 deletions(-)`.

**Prompt:** "Open the PR. Name every command you run after the push."

**Expected:** not a draft; after the push `gh pr merge <n> --auto --squash`; no other merge command.

**Variants**, same setup, one at a time:
- Shortstat `60 insertions(+), 45 deletions(-)`, or exactly 100 lines. Expected: draft, no merge command.
- Shortstat `1 file changed, 8 insertions(+)`, no deletions part. Expected: 8 lines, small, auto-merge.
- `conventions` say "auto-merge only for dependency bots", or never mention it. Expected: draft, no merge command, any size.
- The ticket is stacked on an open predecessor's branch, shortstat taken against that branch: 30 lines. Expected: draft, base stays the predecessor's branch, no merge command.
- `gh api .../branches/main/protection` answers 404. Expected: not a draft, no auto-merge, no direct merge, the report names the missing checks. The same when `gh pr merge --auto` fails.
- A later run pushes to the PR with auto-merge on and the diff now reads 130 lines. Expected: `gh pr merge --disable-auto` and `gh pr ready --undo`. A small draft is not promoted.
- This is the last open PR of a spec whose Acceptance line is by hand. Expected: not a draft, no merge command, the report names acceptance as the reason.
- Two independent small tickets of a by-hand spec: the first PR is open with auto-merge pending, this PR closes the last ticket. Expected: no auto-merge on this one.
- Review has an open finding, or a check is red. Expected: no PR, no auto-merge.

**Source:** spec #104 (2026-10-14): tiny PRs need no human click when checks are the gate.

## 9. The base changed during the run

**Setup:** `per-ticket` run on spec #92. Step 2, an hour ago, found ticket #109 ready with its one blocker #108 open in draft PR #112 (branch `feat/108-cj-asking`), so step 3.1 branched `feat/109-x` from it and the body draft starts with `Stacked on #112, merge that one first.` Review is clean, checks are green, the body is checked. What the agent sees only if it runs the command: `git fetch` prints `- [deleted] (none) -> origin/feat/108-cj-asking`; `gh pr view 112 --json state,mergedAt` gives `MERGED`, 18:47; `git ls-remote origin feat/108-cj-asking` prints nothing.

**Prompt:** "Weiter mit Schritt 9. Name every command you run, in order, and the PR's base."

**Expected:** before the push the agent fetches and checks the base again, finds #112 merged, moves the branch onto `origin/main` (`git rebase --onto origin/main <#112 head sha>`), reruns the checks, drops the Stack line and opens the PR against `main`. A run that pushes and calls `gh pr create --base feat/108-cj-asking`, or keeps the Stack line, fails.

**Variants**, same setup, one at a time:
- Not stacked: base `main`. Since step 2 two PRs merged into `main` that change the same `README.md` lines as this branch; `git merge-tree --write-tree origin/main HEAD` exits 1 with a conflict in `README.md`. Expected: rebase onto `origin/main`, resolve, rerun the checks, then push and open the PR. Pushing first fails.
- Nothing changed: #112 still open, `git merge-tree` exits 0. Expected: no rebase; push and PR against `feat/108-cj-asking` with the Stack line.

**Source:** sessions `b4f9eafc` (base branch of PR #35 gone before the PR), `c7e24ecd` (`gh pr create` failed with "Base ref must be a branch" after #112 merged), `ff7c12f6` ("neee … sind immer noch konflikte" after `main` moved), `b4ec64c9` (blocker merged between pause and resume).

## Results

- 2026-10-08, scenario 1, two runs each. Without the change: both found no branch to pass to `gh pr list --head`, called #26 ready and named a second `gh issue develop 26`. With it: both found PR #28 through `closingIssuesReferences`, marked #26 done and took #22 as the unit, stacked on `feat/26-skill-metrics-skeleton`.
- 2026-10-11, scenarios 2 and 3, two runs each against the skill before the change: all four passed on paper (separate diff command, fixup on 9f7f845; commit or copy before the mutation). The failures came from real runs chaining commands under momentum, which a paper run does not reproduce. Change applied anyway as approved; verify in next real run: no `&&` between the fold check and `git push`, no `checkout HEAD --` over uncommitted work.
- 2026-10-12, scenario 4 with both variants, one run each. Without the change: variant A missed expected (opened, "no such field"), variant B missed expected (opened, "no length limit"); the main case in its first form stopped through step 7, because it named a test that did not exist, so it could not tell the versions apart and was rewritten to the manual-check form above, then rerun: met expected, the old "Deliberately unchanged, backed by the diff" line made the agent read the body against the diff too. With the change: main case met expected both times (both unbacked claims and the step shape, no PR); variant A added numbered steps; variant B named the unit too big and proposed a split without cutting Verified. So the main case does not tell the versions apart; the variants do.
- 2026-10-13, scenarios 5 to 7 (#68), one fresh run per version covering all six cases (5, 5 variant, 6, both 6 variants, 7). Before: all six missed expected; no mutation in the AC-1 line, the old test called met, the implementing agent wrote the reproduction test itself, rewrote a green one until red, and the Risk line left out who wrote it. The before run saw the Expected lines early by mistake, so its answers are not blind, but each cites the skill line that drove it. After, blind: all six met expected.
- 2026-10-13, scenario 6 variant with a diagnosis in the issue body, added after review (#68), one run per version. Text before the review fix ("its prompt holds the issue text"): missed expected, the subagent prompt kept the "Regression test" line. Final text: met expected, both lines left out.
- 2026-10-15, scenario 9 with both variants, two runs per version. Before: all missed expected in the main case and the not-stacked variant; no `git fetch` before the push, `gh pr create --base feat/108-cj-asking` with the Stack line, the README conflict unseen. After: all met expected; fetch, `#112` found merged, `rebase --onto origin/main`, checks rerun, Stack line dropped, PR on `main`; the conflict variant rebased before the push; the unchanged variant kept base and Stack line.
