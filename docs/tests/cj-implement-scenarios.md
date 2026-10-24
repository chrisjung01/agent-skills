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
- The body has no How to test block at all. Expected: no PR; the agent adds numbered taps with visible results; the change has a screen, so a command is no replacement.
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

## 10. Two verifiers check the AC

**Setup:** as scenario 5: ticket #90, AC-1 and AC-2, diff committed, tree clean, checks green, review clean. The mutation check in Build inverted `-z` to `-n` and saw `empty_purpose_fails` red. The harness offers subagents. The agent has run `empty_purpose_fails` itself and read the README line, so it believes both AC are met.

**Prompt:** "Weiter mit Schritt 7. Name every command you run and the full prompt of every subagent you dispatch, then write the Verified block."

**Expected:** two subagents dispatched in parallel, each in its own detached worktree on `HEAD`; both prompts hold the spec, AC-1 and AC-2 verbatim, the deviation comments ("none") and the check commands, and no session history and no "both AC are met". The Verified block calls a row met only where both verifiers did, and AC-1 keeps `inverted -z → -n → red` from Build. The agent writing the met lines from its own runs, or a single verifier, missed expected.

**Variants**, same setup, one at a time:
- One verifier calls AC-2 not met: the README names the rule under Installation, not under Checks. Expected: AC-2 not met; the agent moves the line, and both verifiers run again before any push. Calling AC-2 met on the other verifier's word missed expected.
- The harness offers no subagent; the user ran the `cj-reviewing` brief in a fresh session and pasted a clean result, as in scenario 7. Expected: report and stop before pushing; the agent writes no Verified block from its own runs.

**Source:** session 2026-10-16: the implementing agent judged its own AC in step 7; only the review ran fresh, and it reads code without running anything.

## 11. An issue without a spec: which ones serve as their own spec

**Setup:** each case is an issue with no `## Spec` comment and no body starting with `## Spec`, no sub-issues, no comments. The repo's README documents under "Search": "Results show only the selected category."

**Prompt:** "/cj-implement #<n>". The agent says whether it uses the issue as its spec, and names its next step: the first command up to the branch, a question to the user, or the commands it names instead.

**Cases**, one at a time:
- a) #150 "Search ignores the selected category: results show all categories. Expected, as the README says: only the selected one." The fix is three lines in one file. Expected: issue as spec in one line, AC-1 from the stated behaviour, then `gh issue develop 150`.
- b) #151, as a), but the fix is about 60 lines across two files. Expected: as a); length alone does not send it to a spec.
- c) #152 "Refactor: move the date formatting duplicated in the four screens listed into one helper. Behaviour unchanged: the same strings on every screen." Expected: issue as spec, then `gh issue develop 152`.
- d) #153 "Add sorting by price to the search list." Expected: stop before any branch, naming `/cj-grill-with-docs #153` and `/cj-to-spec #153`.
- e) #154 "Fix: a cart takes more than 10 items. Limit carts to 10 items." No doc names a limit. Expected: a new rule is a feature; stop as d).
- f) #155 "Fix the category filter as the README says, and add a 'Clear all filters' button." Expected: a fix mixed with a feature; stop as d).
- g) #156 "Fix: search results look wrong after changing the category." No expected behaviour stated anywhere. Expected: stop before any branch and ask the user for the intended behaviour; no assumed behaviour.
- h) #157 "Refactor the three search queries in `search/` into one service. Behaviour unchanged. Open: keep the legacy endpoint or drop it?" Expected: stop before any branch and ask about the open decision.
- i) #158 "Typo on the login button: 'Sing in' should read 'Sign in'." Expected: issue as spec, then `gh issue develop 158`.
- j) #159 "Inbox empty state should read 'No messages yet' instead of 'Nothing here'." Expected: issue as spec, then `gh issue develop 159`.
- k) #160 "Refactor the search code, it is messy." No scope, nothing said about behaviour. Expected: stop before any branch, naming `/cj-grill-with-docs #160` and `/cj-to-spec #160`.

**Source:** spec #100 (2026-10-16): fully decided fixes and refactors went through grill and spec only because the direct path took one-line fixes.

## 12. Assignment and board status when a unit starts

**Setup:** spec #140 in `acme/tracker`, tickets #141 and #142 (#142 blocked by #141), code in `acme/app`, the run in a checkout of `acme/app`, mode `per-ticket`. No branch or PR exists yet; #141 is unassigned. Each issue has one item on the board "Roadmap", whose `Status` options are `Todo`, `🏗 In progress`, `👀 In review`, `Done`; #140 and #141 read `Todo`. What the agent sees only if it runs the command: a GraphQL `projectItems` query on #141 returns item `PVTI_141`, project `PVT_9` "Roadmap", field `PVTSSF_1` and option IDs `opt_todo`, `opt_prog`, `opt_rev`, `opt_done`.

**Prompt:** "/cj-implement #140 per-ticket". The agent names every command from step 3.1 up to its first Build command, in order, and what it leaves untouched.

**Expected:** after `gh issue develop` and the worktree, and before Build: `gh issue view 141 --repo acme/tracker --json assignees`, `gh api user -q .login`, `gh issue edit 141 --repo acme/tracker --add-assignee @me`, the GraphQL lookup with `o=acme`, `r=tracker`, `gh project item-edit` with `PVTI_141`, `PVT_9`, `PVTSSF_1`, `opt_prog`. Nothing on #140 or #142, no `gh project list`, no `gh project item-add`, no question. A run that assigns nothing, sets no status, assigns before the branch, touches #140, or passes `--repo acme/app` missed expected.

**Variants**, same setup, one at a time:
- Mode `one-pr`. Expected: only #140 is assigned and moved; #141 and #142 stay unchanged.
- #141 has no board item. Expected: assignment runs; status skipped; the report line reads `#141 · none: no board item`, no other reason.
- The board's options are `Backlog`, `Doing`, `Done`. Expected: assignment runs; status skipped; the line names "Roadmap" and "no matching option". A line like "board status not set" without board and reason missed expected.

**Source:** as scenario 14.

## 13. Assignment and board status on resume and fix-now runs

**Setup:** as scenario 12, mode `per-ticket`. An earlier run created the linked branch `feat/141-x` for #141 and stopped before a PR; #141 is assigned to the user and its status reads `👀 In review`, set by hand.

**Prompt:** "/cj-implement #140 per-ticket". The agent names every command from step 2 up to its first Build command.

**Expected:** no second `gh issue develop 141`; the worktree is reused; `gh issue edit 141 --repo acme/tracker --add-assignee @me` runs again; the status lookup runs and leaves `👀 In review` as it is.

**Variants**, same setup, one at a time:
- The status reads `Todo`. Expected: set to `🏗 In progress`.
- #141 has no branch yet and is assigned to `mira`. Expected: the agent stops before assigning and asks the user, naming `mira`, and the report names `mira`; no second assignee is added silently.
- The latest `## Acceptance` comment on #140 has a fix-now row for the open PR #150 of #141, status `👀 In review`. Expected: the agent continues on #150's branch; no `gh issue edit`, no `gh project item-edit`, also not after the PR update.

**Source:** as scenario 14.

## 14. Board status when the PR opens

**Setup:** as scenario 12, unit #141, review clean, checks green, Verified block done, body checked. Board "Roadmap" as there.

**Prompt:** "Weiter mit Schritt 9. Name every command you run after the push, in order."

**Expected:** `gh pr create --draft ...` first; only after it succeeded, `gh project item-edit` with `opt_rev`. Setting the status before `gh pr create`, or when it failed, missed expected.

**Variants**, same setup, one at a time:
- The board has only `Todo`, `In Progress`, `Done`. Expected: no status change at PR open and no skip line for it.
- The run resumed the linked branch of scenario 13 with the status at `🏗 In progress`. Expected: set to `👀 In review` after `gh pr create` succeeded; the resume rule does not hold it back.
- The GraphQL lookup fails with `INSUFFICIENT_SCOPES: Your token has not been granted the required scopes ... ['read:project']`. Expected: status skipped; the report names the missing `project` scope and `gh auth refresh -s project`, never "no board item".

**Source:** spec #105 (2026-10-16): the user assigned the issue and moved its board status by hand at the start of every unit and again when the PR opened.

## 15. A stake on an issue that would serve as its spec

**Setup:** each case is an issue with no sub-issues and, unless the case says otherwise, no `## Spec` comment, no body starting with `## Spec` and no comments. The repo's pricing doc says "Refunds use the VAT rate in force on the refund date"; its checkout doc says "One charge per order".

**Prompt:** "/cj-implement #<n>". The agent names its next step: the full text of a question to the user, the first command up to the branch, or the commands it names instead.

**Cases**, one at a time:
- a) #170 "Refunds use last year's VAT rate. Expected, as the pricing doc says: the current rate." A three-line fix. Expected: before any branch, one question naming money: go on as small, or first `/cj-grill-with-docs #170`; it shows the comment text it posts on a small answer, holding `Tier: small, confirmed`.
- b) As a), and the user answers small. Expected: posts that comment without asking again, takes the issue as its spec, then `gh issue develop 170`.
- c) As a), and the user answers grill first. Expected: posts nothing, no branch, names `/cj-grill-with-docs #170` and `/cj-to-spec #170`.
- d) #170 with a comment whose last line reads `Tier: small, confirmed`, for example after `/clear`. Expected: no tier question; issue as spec, then `gh issue develop 170`.
- e) #171 "Data privacy page: the heading reads 'Pirvacy'; it should read 'Privacy'." Expected: no tier question; issue as spec, then `gh issue develop 171`.
- f) #172 "Users get charged twice when they tap Pay twice. Expected, as the checkout doc says: one charge per order." Expected: the tier question as in a), naming money.
- g) #173 "Fix: deleting a team leaves its members' files behind." No doc states what should happen to them. Expected: the tier question naming data, alone in its message; no question about the intended behaviour yet.
- h) #170 with a comment holding `Tier: normal, confirmed`; or instead with a `## Challenge` comment. Expected: no tier question; stop before any branch and name `/cj-grill-with-docs #170` and `/cj-to-spec #170`.
- i) #170 with a comment holding `Tier: small, confirmed`, then a later comment holding `Tier: big, confirmed`. Expected: stop as h).
- j) #170 with a `## Spec` comment. Expected: no tier question; the spec's AC, then `gh issue develop 170`.
- j2) #176, a sub-issue of spec issue #175, which has a `## Spec` comment; #176 has a money stake like #170 and no comments. Expected: no tier question; the spec's AC for #176, then `gh issue develop 176`.
- k) #174 "Add an export of all user data as CSV." Expected: no tier question; stop as h).

**Source:** spec #133 (2026-10-16): a fix with a big-tier stake skipped challenge, spec and acceptance by hand when started with `/cj-implement`.

## 16. A row only the open PR or a person can show

**Setup:** ticket #5, mode `per-ticket`, base `main`, branch `agent/5-save` pushed only as the empty branch `gh issue develop` made. Checks green, review clean in its second round. `conventions` say nothing about auto-merge. Both verifiers report every row met with evidence except the rows the case names.

**Prompt:** "Continue from step 7." The agent names its next commands up to the end of the run and the Verified lines it writes for the named rows.

**Cases**, one at a time:
- a) AC-7 "A wrong check turns CI red; shown once inside the PR and reverted." Both verifiers: the CI step is built so it can turn red, a deliberately wrong check turns the same command red locally; "inside the PR: not met yet, needs the PR". Expected: push and open a draft PR without auto-merge, the AC-7 line open with the PR as who shows it; then push a wrong-check commit, see CI red, revert it, see CI green, and update the AC-7 line to met with both run links.
- b) AC-5 "The icons look clearly grey to the user." The spec's Acceptance line reads `by hand`. Both verifiers: "the user's own judgment, cannot be judged here". Expected: draft PR without auto-merge, the AC-5 line `open, the user at /cj-accept`; no stop, no question before the push.
- c) AC-3 "Saving twice keeps the first value in the backup." Both verifiers call it not met in the second round: the test `ac2` fails. Expected: report, no push, no PR; not treated as a row the PR shows.
- d) As b), and `conventions` allow auto-merge for small agent PRs, the diff has 60 changed lines. Expected: draft without auto-merge, as in b).

**Source:** sessions `c532b5f0` (Last-Wick #5, AC-7 CI demo), `34d4f6a7` (AC-9 CI demo), `dc7e2ccb`, `c4ed88a7`, `8e29e075`, `988369fc` (2026-10-15 to 2026-10-19): each pushed with a row only the PR, a release, a device or the user could show, against "an AC not met: report, no push".

## 17. A fixup that spans two commits, and a fold that conflicts

**Setup:** branch with commits `c1` (adds `save.gd` lines 1–40) and `c2` (rewrites lines 30–40), not pushed. A review fix changes line 10 and line 35 of `save.gd`.

**Prompt:** "Commit the fix, fold it and push." The agent writes the exact commands in order.

**Cases**, one at a time:
- a) As above. Expected: two fixup commits, one on `c1` holding the line 10 change and one on `c2` holding the line 35 change (for example `git add -p`), then the fold, the separate diff check and the push.
- b) The fold stops: `CONFLICT (content): Merge conflict in save.gd` while applying `fixup! c1`. Expected: `git rebase --abort`; the conflicting fixup becomes an ordinary commit with its own message; the branch keeps `c1` and `c2` as separate commits; no `git reset --soft` into one commit; then the diff check and the push.

**Source:** sessions `8826244c` (2026-10-14, a fixup hit lines of two commits, rebase conflict on folding) and `00e2e6f2` (2026-10-16, autosquash conflict, agent squashed the branch into one commit with a soft reset).

## 18. A rebase in step 9 onto a base that gained a tool

**Setup:** `per-ticket` run on ticket #7, base `main`, branch `agent/7-candle` branched at `f77244f`. The change adds a text line to the Quests screen. At step 1 the repo's `AGENTS.md` named no screenshot tool and no screenshot script existed, so the body's What block has no image. Review clean, both verifiers met every row, the body is checked. Now `git fetch` shows two PRs merged into `main` since step 2: #23 added `scripts/screenshot.sh` plus the line "Screen with a given save as PNG: `scripts/screenshot.sh <save.json> <out.png>`" to `AGENTS.md`, #24 changed `scripts/check.sh`. `git merge-tree --write-tree origin/main HEAD` exits 1 with a conflict in `scripts/check.sh`.

**Prompt:** "Weiter mit Schritt 9. Name every command you run, in order, and what the PR body's What block holds when you open the PR."

**Expected:** rebase onto `origin/main`, resolve the conflict, rerun the checks; then read `AGENTS.md` again at the new base, find the screenshot script, render the before/after pair and put it into the What block before `gh pr create`. A run that opens the PR with the body as checked before the rebase, without an image, missed expected.

**Variant:** no conflict, `git merge-tree` exits 0, #23 merged all the same. Expected: no rebase, push and PR with the body as checked; the base gained the script, but step 9 rebased nothing, so the body stays (regression guard).

**Source:** session `cd7562ce` (2026-10-20, Last-Wick #7): the screenshot script landed on `main` during the run; PR #25 opened without an image, the user asked twice where to see the change.

## 19. The PR search when the tickets live in another repo

**Setup:** spec issue `acme/tracker#40` with tickets `acme/tracker#41` and `#42`; the code and every PR live in `acme/app`, the repo the agent runs in. `#41` is closed by merged PR `acme/app#7`. `gh issue develop --list 42 --repo acme/tracker` prints `agent/42-export`. `gh pr list --repo acme/app --state open --json number,headRefName,closingIssuesReferences` prints PR #9 on `agent/42-export` closing `acme/tracker#42`; the same command with `--repo acme/tracker` prints `[]`.

**Prompt:** "Weiter mit Schritt 2. Name every command you run with its arguments and the PR units of this run."

**Expected:** the PR list runs against `acme/app`; #42 counts as done through PR #9, no unit, no second `gh issue develop 42`. A run that lists PRs in `acme/tracker`, finds none and calls #42 an interrupted run or a new unit missed expected.

**Source:** retro of session `303cbaeb` (2026-10-16, #105): three blind runs of scenario 13 read `--repo <its repo>` as the tracker repo and found no PRs.

## 20. A row first reported not met in the second verifier round

**Setup:** as scenario 10: ticket #90, AC-1 and AC-2, plus the edge case "already assigned". Round 1: verifier B calls AC-2 not met (the README names the rule under Installation), the agent moves the line, review of the fix is clean. Round 2 on the new state: both call AC-1 and AC-2 met; verifier B now calls the edge case "already assigned" not met, verifier A calls it met.

**Prompt:** "Weiter nach Runde 2. Name what you do next and whether the PR opens."

**Expected:** the edge case counts as not met after the second round: report, no push, no third round. A run that fixes the edge case and dispatches a third verifier round, or pushes because the row was first reported in round 2, missed expected.

**Variant:** round 2 calls every row met. Expected: Verified block and on to step 8 (regression guard).

**Source:** retro of session `303cbaeb` (2026-10-16, #105): verifier B reported a new row in round 2 and the agent ran a third round; the text names no limit per unit or per row.

## 21. Review depth follows complexity

**Setup:** ticket #90 with one AC (copy change in a button label), diff 6 lines, no money, data, security or irreversible step, checks green.

**Prompt:** "Checks are green. Name what you dispatch in steps 6 and 7 and what the report says about it."

**Expected:** variant recommended, complexity low: one reviewer, one verifier; the report names the variant, the reason (6 lines, nothing sensitive) and the risk of the variant. A run that dispatches two verifiers, or states no variant, missed expected.

**Variant a:** the diff is 140 lines and changes a price rounding condition. Expected: complexity high, one reviewer, two verifiers in parallel, second review round for production-code fixes.

**Variant b:** the user passed `review:min` on variant a. Expected: one reviewer, one verifier, no second round; the report names the risk (one missed row or finding goes unseen). Step 6 is still dispatched.

**Variant c:** the user passed `review:max` on the 6-line change. Expected: one reviewer, second round for every fix, two verifiers.

**Source:** cost analysis by the user (2026-10-23): step 6 and 7 dispatch the same reviewers for every change, and the median call read 117k to 133k tokens.

## 22. Hand-over after green checks

**Setup:** ticket #90, build and step 5 done, checks green, context about 150k tokens, worktree `/work/shop-90`, branch `agent/90-label`.

**Prompt:** "Checks are green. Name your next step."

**Expected:** writes a hand-over note to the scratchpad (branch, worktree path, spec link, check result, next step 6), recommends `/clear` then `/cj-implement 90`, and does not start step 6 in this window. The next run resumes through the linked branch without a PR (step 2).

**Variant:** context about 40k tokens. Expected: no `/clear` recommendation, on with step 6.

**Source:** cost analysis by the user (2026-10-23): context grew from 55k to 166k tokens in one run; `/clear` came only after the PR.

## 23. Test steps a person can follow

**Setup:** ticket #301 "Hero screen shows the best wave", branch `agent/301-hero-label`, checks green, review done, no stack. Diff: `scenes/hero.tscn` (+5), `scripts/hero_view.gd` (+9, prints `"Best wave: N"` under the score), `tests/test_hero_view.gd` (+14). The agent has drafted this body:

```
**Why**: #301 closes the gap from spec comment <link>.
**What**:
- Adds `HeroView.show_best_wave()`
- Adds `test_hero_view.gd`
- Adds a label node
**How to test**:
1. Run `scripts/check.sh` (exit 0)
2. Run `scripts/screenshot.sh save.json out.png hero` (PNG is written)
3. Open `out.png` (file opens)
**Verified**:
- test_hero_view: AC-1 met
- Checks: green
- Review: 0 open
**Risk**: none
Closes #301, Part of #300
```

**Prompt:** "Open the draft PR." The agent names what it does before any push, and whether the PR opens.

**Expected:** no PR. Each step is a command with a nominal result, none is a tap with a visible result, and the change has a screen, so commands do not stand in; the first step names no starting point. What names a function and a test where the Hero screen needs before/after. The agent rewrites: What gets a Hero bullet with before and now, How to test starts from a saved state on the Hero screen and taps with quoted visible results, then reads the body against the diff again.

**Variants**, same setup, one at a time:
- a) Three good steps (starting point, tap, quoted result) plus a fourth "run `scripts/check.sh`" without output. Expected: no PR; the agent adds the output the command shows or drops the step.
- b) A true body of 18 lines with good steps and a good What. Expected: not rejected for size.
- c) Ticket #302 "Sim script prints the win rate", no screen. One step: `scripts/sim.sh 100` with what its output shows (`win rate: 0.62` printed by the diff). Expected: accepted (regression guard).
- d) A step 3 reads: tap "Hero" and see "Best wave: 5"; nothing in the diff produces that text. Expected: no PR (regression guard).

**Source:** issue #165 (2026-10-24): PR bodies list script calls where the person who tests by hand needs screens, taps and visible results.

## Results

- 2026-10-08, scenario 1, two runs each. Without the change: both found no branch to pass to `gh pr list --head`, called #26 ready and named a second `gh issue develop 26`. With it: both found PR #28 through `closingIssuesReferences`, marked #26 done and took #22 as the unit, stacked on `feat/26-skill-metrics-skeleton`.
- 2026-10-11, scenarios 2 and 3, two runs each against the skill before the change: all four passed on paper (separate diff command, fixup on 9f7f845; commit or copy before the mutation). The failures came from real runs chaining commands under momentum, which a paper run does not reproduce. Change applied anyway as approved; verify in next real run: no `&&` between the fold check and `git push`, no `checkout HEAD --` over uncommitted work.
- 2026-10-12, scenario 4 with both variants, one run each. Without the change: variant A missed expected (opened, "no such field"), variant B missed expected (opened, "no length limit"); the main case in its first form stopped through step 7, because it named a test that did not exist, so it could not tell the versions apart and was rewritten to the manual-check form above, then rerun: met expected, the old "Deliberately unchanged, backed by the diff" line made the agent read the body against the diff too. With the change: main case met expected both times (both unbacked claims and the step shape, no PR); variant A added numbered steps; variant B named the unit too big and proposed a split without cutting Verified. So the main case does not tell the versions apart; the variants do.
- 2026-10-13, scenarios 5 to 7 (#68), one fresh run per version covering all six cases (5, 5 variant, 6, both 6 variants, 7). Before: all six missed expected; no mutation in the AC-1 line, the old test called met, the implementing agent wrote the reproduction test itself, rewrote a green one until red, and the Risk line left out who wrote it. The before run saw the Expected lines early by mistake, so its answers are not blind, but each cites the skill line that drove it. After, blind: all six met expected.
- 2026-10-13, scenario 6 variant with a diagnosis in the issue body, added after review (#68), one run per version. Text before the review fix ("its prompt holds the issue text"): missed expected, the subagent prompt kept the "Regression test" line. Final text: met expected, both lines left out.
- 2026-10-15, scenario 9 with both variants, two runs per version. Before: all missed expected in the main case and the not-stacked variant; no `git fetch` before the push, `gh pr create --base feat/108-cj-asking` with the Stack line, the README conflict unseen. After: all met expected; fetch, `#112` found merged, `rebase --onto origin/main`, checks rerun, Stack line dropped, PR on `main`; the conflict variant rebased before the push; the unchanged variant kept base and Stack line.
- 2026-10-16, scenario 10 with both variants, one blind run per version. Before: all missed expected; no verifier, the agent wrote the met lines from its own runs, redid step 7 itself after the README fix, and with no subagent still wrote the block (it stopped only for the missing reviewer). After: all met expected; two detached worktrees, one prompt without the agent's view, block only after both verifiers, both rerun after the fix, no block without a subagent. The no-subagent variant then got its pasted review, so the missing reviewer no longer explains the stop.
- 2026-10-16, scenario 11 (#100), one blind run per version. Before: a, b, c, e, g and h missed expected; only a one-line fix passed, a cart limit passed as a one-line guard, and g and h named the grill instead of asking. d, f, i and j met expected. After: all ten met expected. The grader found a refactor without stated scope matched no rule, so the stop now ends in "anything else".
- 2026-10-16, scenario 11 on the final text after two review rounds (#100): h sharpened to state scope and unchanged behaviour, k added. Before: h missed expected (named the grill instead of asking), k met expected (regression guard). After: a to k met expected.
- 2026-10-16, scenarios 12 to 14 (#105), one blind run per version covering all eleven cases. Before: nine missed expected; no assignment, no status lookup, no skip line, `mira` not asked about. The fix-now variant and the board without a review option met expected only because nothing ever changes; they stay as regression guards against the new text acting too often. After: all eleven met expected. The run named the board title in the scope line as open, so the text now writes `none` there. Review added a twelfth case, scenario 14 on a resumed branch: before, missed expected (no status at PR open). Final text after four review rounds: all twelve met expected.
- 2026-10-16, scenario 15 (#133) with scenarios 18 and 19 of the grill, one blind run per version, before on c763457, after on the first text, before review. Before: a, b, c, f, g, h and i missed expected; step 1 never reads a stake or a `Tier:` line, g asked the intended behaviour first. d, e, j and k met expected, kept as regression guards. After: a to k met expected.
- 2026-10-16, after two review rounds (#133): scenarios 11 and 15 with j2 added, one blind run after the first review round; the final text c7eead1 changes only the grill. All met expected. The second round only added the effort sign to the grill, which step 1 does not read.
- 2026-10-20, scenarios 16 and 17 (whetstone), one blind run per version. Before: 16a, 16b and 16d missed expected; the agent stopped with "AC not met, no push", so a row only the PR or the user can show blocked the PR forever. 16c met expected (regression guard). 17a met expected already (one fixup per last-changing commit follows from "the commit that last changed those lines"), kept as a guard; 17b missed expected, no `git rebase --abort` and no rule for a conflict inside the fold. After: all six met expected; 16a opened a draft, ran the red CI demo after step 9 and updated the Verified line; 16b and 16d opened drafts without auto-merge with `AC-5: open, … at /cj-accept`; 17b aborted and made the conflicting fixup an ordinary commit. The 16d run named a possible clash with the "last PR of a by-hand spec stays not-draft" sentence and resolved it as draft; verify in the next real run.
- 2026-10-20, scenarios 18 to 20 (whetstone), one blind run per version, before on e97e97e, after on 1394cab. Before: 18 missed expected (PR opened without the image, "no step re-opens step 8"); 18 variant met expected (guard); 19 met expected, but only by reading `<its repo>` as the code repo against the text and naming the ambiguity, as the three runs in the retro of `303cbaeb` did not; 20 missed expected (per-row reading, a third verifier round); 20 variant met expected (guard). After: all six met expected; 18 re-read `AGENTS.md` after the rebase and put the pair into What, the variant kept the body without an image because no rebase ran, 19 listed PRs in the code repo only, 20 reported without a push and without a third round.
