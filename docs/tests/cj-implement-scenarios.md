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

**Setup:** ticket #21, branch `feat/21-bare-question-hook`, never pushed except the empty branch `gh issue develop` created. Review findings F5 and F9 are fixed in the working tree, checks are green, the acceptance table is done. Tool output:

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

## Results

- 2026-10-08, scenario 1, two runs each. Without the change: both found no branch to pass to `gh pr list --head`, called #26 ready and named a second `gh issue develop 26`. With it: both found PR #28 through `closingIssuesReferences`, marked #26 done and took #22 as the unit, stacked on `feat/26-skill-metrics-skeleton`.
- 2026-10-11, scenarios 2 and 3, two runs each against the skill before the change: all four passed on paper (separate diff command, fixup on 9f7f845; commit or copy before the mutation). The failures came from real runs chaining commands under momentum, which a paper run does not reproduce. Change applied anyway as approved; verify in next real run: no `&&` between the fold check and `git push`, no `checkout HEAD --` over uncommitted work.
- 2026-10-12, scenario 4 with both variants, one run each. Without the change: variant A missed expected (opened, "no such field"), variant B missed expected (opened, "no length limit"); the main case in its first form stopped through step 7, because it named a test that did not exist, so it could not tell the versions apart and was rewritten to the manual-check form above, then rerun: met expected, the old "Deliberately unchanged, backed by the diff" line made the agent read the body against the diff too. With the change: main case met expected both times (both unbacked claims and the step shape, no PR); variant A added numbered steps; variant B named the unit too big and proposed a split without cutting Verified. So the main case does not tell the versions apart; the variants do.
