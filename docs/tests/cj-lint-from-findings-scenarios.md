# cj-lint-from-findings scenarios

Dry runs by fresh subagents (role-play, nothing posted or pushed). RED: the subagent gets the caller's current text and no `cj-lint-from-findings`. GREEN: it also gets the new skill and the caller's new call line.

## A. Banned internal import

Setup: `cj-review-loop` in loop mode, a TypeScript repo with ESLint. Finding F2 (STANDARDS, medium) at `src/orders/list.ts:3`: "imports `src/billing/internal/tax.ts` directly; other modules must use `src/billing/index.ts`". Fixed in the working tree, uncommitted.
Expected: rung 2 (restricted imports with the pattern `**/billing/internal/*`), the probe is today's `src/orders/list.ts` with the uncommitted line-3 hunk reversed, the line "fires on probe, fix clean, 0 hits in repo", one choice question with own PR first.

## B. Smell only

Setup: `cj-review-loop` final report with F1 "feature envy in `OrderMapper`" and F3 (medium) "mysterious name `tmp2`", both fixed.
Expected: one line "nothing to mechanize", no question, no file touched.

## C. Needs a custom AST rule

Setup: `cj-address-pr-feedback`, fixed thread: "every `useQuery` call must pass `staleTime` when the key starts with `user`".
Expected: rung 4, no plugin code, an issue draft with the pattern and the fix commit; the choice offers only issue and drop.

## D. Many hits elsewhere

Setup: as A, but the whole-repo run finds 12 other files with the same import.
Expected: the hits listed (`file:line`, at most 10, then "and 2 more"); with 12 hits the own-PR option names the count and makes the cleanup an issue, and `warn` is offered only when the lint command ignores warnings; nothing decided without the question.

## E. Config with uncommitted changes

Setup: as A, and `eslint.config.js` already has an uncommitted edit from the review fixes.
Expected: the agent copies the config before the proof and restores it after; `git status --short --ignored` before and after are equal and the earlier edit is still there.

## F. Retro with a mechanical standards finding

Setup: `/cj-retro`; the session shows a reviewer missing a `console.log` left in production code.
Expected: the finding is listed with the change "lint rule via cj-lint-from-findings"; after the user's yes in retro step 5, the agent calls the skill instead of filing a plain issue.

## G. Linter not runnable

Setup: as A, but `node_modules` is missing and the lint command fails with "Cannot find module".
Expected: the proof stops, the reply names the error, the finding is offered as an issue; config unchanged.

## Results

- RED (2026-10-13, one rep per scenario, callers before #63): 7 of 7 reps drafted no rule.
  - A, B, C, E, G: the run ends at the final report or output block, with no question and no rule.
  - D: the 12 other imports go unmentioned. "Change only what the findings name" keeps the loop off them, and nothing reports them.
  - F: `cj-retro` files a plain issue "enable `no-console`" without proving the rule fires.
  - G: the rep notes that "Red: repair first" pushes toward installing dependencies.
- GREEN (2026-10-13, one rep per scenario, with the skill and the call lines): 7 of 7 reps met Expected.
  - A and E: rung 2 `no-restricted-imports`, probe from `git show HEAD:`, restore checked. B: "nothing to mechanize", no question. C: rung 4, issue draft, no plugin code.
  - The reps found six gaps, all fixed in the skill: a one-name rule counted as mechanical (B), a diff against `HEAD` carried earlier edits and `git status` missed a bad restore (E), the linter ran only after the config edit (G), the probe counted as a repo hit (A), two options on rung 4 clashed with the three-option rule (C), and `warn` blocks nothing when CI ignores warnings (F).
  - Rerun after the fixes: D recommends fixing the 12 hits at `error` because CI ignores warnings. G stops at the first lint run, touches no file and offers issue or drop.
- Verify in next real run: the next whetstone run checks cj-lint-from-findings calls for a probe left behind, a config not restored, or a rule proposed without its proof line.
- Real run (2026-10-13, ESLint 9.39.5, throwaway repo with an uncommitted config edit): rung 2 `no-restricted-imports`, "fires on probe, fix clean, 0 hits in repo". Afterwards `git status --short` matched, `cmp` showed the config byte-identical, and no probe was left. The first attempt hit a config that did not load (CommonJS package, ESM config), stopped at the step 2 dry run and offered issue or drop.

## H. Lint fails on warnings, hits exist

Setup: as A, the lint command runs with `--max-warnings 0`, the whole-repo run finds 12 hits.
Expected: 12 hits exceed the cap of 10, so own PR names the count and makes the cleanup an issue; no `warn` option; current branch not offered.

## I. Lint script fixes files

Setup: as A, `commands` gives the lint command `npm run lint` = `eslint --fix .`.
Expected: the proof runs the check-only form (`eslint .`), the uncommitted F2 fix stays untouched, both 4.1 notes match.

## J. Path ignored

Setup: as A, the linter answers the swapped `src/orders/list.ts` with "File ignored because of a matching ignore pattern".
Expected: no second file and no rename; the agent asks the linter whether it lints that path, and since it does not, the finding is "not proven" and offered as an issue; no rung drop.

## K. Retro without a fix

Setup: as F, `console.log("debug", order)` is still in `src/orders/submit.ts`, fix `none`.
Expected: the first run lints `src/orders/submit.ts` unchanged; the second run puts a corrected version at that path, different only at the `console.log` line, and runs over the whole repo; no rung drop because today's file still has the bug, and that `console.log` counts as 1 hit in repo; afterwards the file is byte-identical to its saved copy.

## L. Own PR with uncommitted review fixes

Setup: as A, the user picks own PR.
Expected: a new worktree from the default branch; the caller's checkout and its uncommitted fixes stay untouched.

## M. Rule breaks the config

Setup: as A, the first draft has a typo in the options and the lint run errors "Configuration for rule ... is invalid".
Expected: restore, fix the draft, run again; the edit never stays in place.

## Results after the final review

- The final review (fresh reviewer, Opus) found one critical and five important gaps: `warn` handling was inverted, a `--fix` lint command could rewrite user work, an ignored probe counted as a miss, the probe source could be wrong, the own PR had no place to work, and a broken draft could stay in the config. Scenarios H to M are those cases.
- GREEN (2026-10-13, one rep each): 6 of 6 met Expected. H drops `warn` and the current branch when lint fails on warnings. I never runs the `--fix` script. J renames the ignored probe without dropping a rung. K writes a corrected second probe. L works in a new worktree. M restores before fixing the draft.
- The reps found three more gaps, now fixed: the step 2 dry run had no check-only wording (I), there was no probe source for fix `none` (K), and the own PR did not re-check in the new worktree (L).

## N. Few hits

Setup: as H, but the whole-repo run finds 4 hits.
Expected: own PR fixes the 4 hits and lands the rule at `error`.

## O. Current branch from PR feedback

Setup: `cj-address-pr-feedback` after phase 3 (pushed, replies posted), F2 proven with 0 hits, the user picks current branch.
Expected: a commit on the PR branch, then a push question through `cj-posting`; nothing pushed without the yes.

## P. Retro offers no current branch

Setup: as F, proven with 0 hits.
Expected: the question offers own PR, issue and drop; no current branch.

## Q. Second instance in the fixed file

Setup: as A, but `src/orders/list.ts` also imports `billing/internal/rates.ts` on line 9, which the finding did not name.
Expected: fix clean at line 3, the line-9 import counts as 1 repo hit; no rung drop.

## Results after the review loop

- GREEN (2026-10-13, one rep per group): D, H, N, O, P and Q met Expected.
  - D and H: 12 hits make the cleanup an issue, and `warn` is offered only when lint ignores warnings. N: 4 hits are fixed in the own PR at `error`.
  - O: on the PR branch the rule lands as a commit, then a push question through `cj-posting`. P: a retro offers no current branch.
  - Q: the line-9 import counts as 1 repo hit, and there is no rung drop.

## R. No linter, no CI

Setup: a shell-script repo with no linter, no CI workflow and no hook; the finding: "scripts call `curl` without `--fail`".
Expected: rung 3, a grep script, and the diff also adds a CI step or a pre-commit entry for it; the reply says so; proof on probe and fix as usual.
  - R (after round 3): rung 3 grep script plus a new CI step in the diff, the reply says so, current branch not offered.

## S. File-location finding

Setup: finding "a barrel `index.ts` under `src/components/button/`, components export from their own file"; the fix renamed it to `src/components/button/exports.ts` in the same folder, uncommitted; rung 2 restricts the file name `index.ts`.
Expected: for the first run the file is moved back to `src/components/button/index.ts`, not copied, so the folder never holds both names; the rule fires on it; for the second run and afterwards the file sits at `exports.ts` again and nowhere else.

## T. Sibling file shares a namespace

Setup: a repo where every source file in a folder belongs to one namespace, and a top-level name defined twice breaks the build. Finding: "`report/export` calls the banned `os.exit` directly", fixed in commit `abc123`; the linter has a banned-call rule.
Expected: no second source file anywhere in the repo; the faulty version is today's `report/export` with the fix's hunks reversed, linted at its own path; rung 2.

## U. Unregistered file is never linted

Setup: a repo where a source file is compiled and linted only when a parent file registers it by name. Finding: "`metrics/store` calls the banned `unwrap_all`", fixed in commit `def456`; the linter has a banned-call rule.
Expected: no second source file and no edit to the registering file; the faulty version is linted at `metrics/store`'s own path; rung 2.

## V. Parallel uncommitted edits

Setup: `cj-review-loop` in loop mode, fix `uncommitted`. `git diff HEAD` on `api/client` holds two hunks: the fix at line 12 (finding's line) and an unrelated rename at line 40.
Expected: only the line-12 hunk is reversed; the faulty version keeps the line-40 rename; afterwards the file is byte-identical to its saved copy.

## W. Faulty version does not build

Setup: as T, but undoing the fix in today's file fails (the three-way merge conflicts), and the whole file before the fix makes the linter error on a name today's code no longer has.
Expected: the agent tries the sources in order (fix undone in today's file, whole old file, the code quoted in the finding in place of the finding's lines, code written from the finding); a source the linter errors on without reporting this rule counts as failed; no rung drop for it; every source failed means "not proven" and an issue offer.

## X. Path silently skipped

Setup: as T; the first run on the swapped file prints no error, no ignore notice and no hit of this rule.
Expected: the agent asks the linter whether it lints that path (its file list or debug output) before deciding; not linted or unclear means "not proven" and an issue offer; only a path shown to be linted drops the rule a rung.

## Y. File changed during the proof

Setup: as V; between the first and the second lint run the user saves `api/client` in the editor.
Expected: before writing the corrected version the agent compares the file's hash with what it last wrote, writes nothing more, stops, and names `api/client` and every saved copy's path.

## Z. Saved copies named first

Setup: a repo with one linter, a lint config with an uncommitted edit, and an uncommitted fix in the finding's source file; the draft proves on the first try.
Expected: before the first write, the chat names the saved copy of the source file and of the lint config.

## Z2. Two runs per draft attempt

Setup: as Z.
Expected: exactly two lint runs inside the proof: the swapped source file alone, then the whole repo with the corrected version.

## AA. Rung drop after a silent first run

Setup: as T; the first run reports nothing for this rule, and the linter's file list shows it lints `report/export`.
Expected: the rule drops a rung, but first the agent restores (4.7) and both notes match; the next attempt keeps the saved copies from the first one, so the user's file ends byte-identical to its state before the proof.

## AB. Fix deleted the file

Setup: finding "a barrel `index` file under `components/button/`", the fix commit `fed789` deleted it; rung 2 restricts that file name.
Expected: step 2 makes no baseline run for the deleted file and does not treat its absence as a linter that does not run; the file comes back at its old path from reversing the fix's deletion for the first run, and is gone again for the second run and afterwards.

## AD. Deletion-only fix with a parallel edit

Setup: fix `uncommitted`; `git diff HEAD` on `orders/submit` deletes the debug-print line 12 the finding names and, as a second change, renames a variable on line 30.
Expected: the deletion hunk's old side holds the debug print, so only it is undone; the faulty version keeps the line-30 rename; no fallback to the whole `HEAD` file.

## AE. Later commit next to the fix

Setup: as T, but a commit after `abc123` changed the line two lines below the fix.
Expected: the three-way merge undoes the fix without a conflict; the faulty version keeps the later change; no fallback to the whole old file.

## AF. Deletion-only fix, later lines above it

Setup: as T, but the fix commit `abc123` only deleted the banned call on line 2, and a later commit added a five-line header comment at the top of `report/export`.
Expected: the banned call comes back below the header, where it stood before the fix, not at line 2 inside the header; the later header stays; no fallback and no rung drop.

## AG. Parallel edit above a deletion-only fix

Setup: fix `uncommitted`; `git diff HEAD` on `orders/submit` replaces line 2 with three lines (an unrelated edit) and deletes the debug-print line 4 the finding names.
Expected: the debug print comes back after the line that stood above it, below the three new lines; the unrelated edit stays; no fallback to the whole `HEAD` file.

## AH. Later edit right above the fix

Setup: as T, but a commit after `abc123` changed the line directly above the fixed line.
Expected: the three-way merge reports a conflict, so the first source does not apply; the next source is used and the rule does not drop a rung.

## AI. Committed rename with the fix

Setup: a file-location finding; commit `c0ffee` renamed `report/f` to `report/g` and also removed the banned call from it; no later change.
Expected: the agent reads the old path from `git show -M --name-status c0ffee`, moves the file back and merges with the file at both paths; the probe holds the banned call, never an empty file; no rung drop.

## AJ. Finding's code never committed

Setup: fix `uncommitted`; `HEAD` imports `./lib`, the session changed it to `../internal/lib` without committing (the finding), and the fix changed it to `@pkg/lib`.
Expected: no hunk's old side holds `../internal/lib`, so the first source fails; the whole `HEAD` file lacks it too and fails; the import quoted in the finding goes in at the finding's line and the rule fires; no rung drop.

## AK. Fix that only inserts a line

Setup: as T, but the finding is "empty catch block at `report/export:12`" and commit `abc123` only inserted `log(e)` after line 12; line 12 itself is unchanged.
Expected: the fix still counts as committed, not `none`; the three-way merge takes `log(e)` out again and the rule fires at the empty catch; no rung drop.

## AL. Fix reverted later

Setup: as T, but a commit after `abc123` put the banned call back at the line the fix changed.
Expected: the fix counts as `none`; the first run lints today's file unchanged and fires; the corrected version is today's file corrected at that line; the call counts as 1 hit in repo, never as firing on the fix; no rung drop.

## Results for the swap (#71)

- RED (2026-10-14, text before #71, one rep per scenario): 24 of 24 missed expected.
  - J, K, T, U, V, W, X, AD to AL: a `lint-probe-` file next to the original, from the whole old file; T, W, X, AE, AF, AH, AK and AL end in a wrong rung drop or a rename loop.
  - S (rename within one folder): the probe is a copy, so the folder holds `index.ts` and `exports.ts` in every run. AB: the probe stays on disk through the whole-repo run.
  - Y: no hash check, the user's save is lost at the restore. Z: no saved copies named. Z2: three runs inside the proof. AA: no restore before a rung drop.
  - A and E (Expected rewritten for #71): A builds the probe from the whole `HEAD` file, E notes `git status` without `--ignored`.
  - The first S (deletion) met expected under the old text too, so it was rewritten as a rename.
- GREEN (2026-10-14, text after #71, one rep per scenario): 25 of 25 met expected, Q and M included as regressions.
  - All 23 earlier ones ran on commit 1a4c270. The commits after it change when a fix counts as `none` and how config errors are handled, so A, K, M, T, W, AK and AL ran again on 3cf172e and met.
  - Eleven review rounds and the scenario passes found the gaps the scenarios D to AL now hold. The mechanism changed twice on the way: a probe patch with context lines, then without context lines, then the three-way merge and hunk placement that ship.
- Real runs (2026-10-14, text of commit 1a4c270, before the `none` switch was narrowed and config errors were split from source failures; neither run reaches those parts):
  - Clippy 0.1.99, throwaway copy of `skill-metrics` with a fix commit and a later commit to the same file: rung 2 `disallowed-methods`, "fires on probe, fix clean, 1 hits in repo" (`src/main.rs:95`). `git merge-file -p <today> <fix> <fix^>` built the probe and kept the later commit's comment.
  - ESLint 9.39.5, throwaway repo with an uncommitted fix and config edit: rung 2 `no-restricted-imports`, "fires on probe, fix clean, 0 hits in repo"; the probe put the old side of the `git diff -U0 HEAD` hunk back at its new-side position.
  - Both: status with ignored files, diff hash and the hash of every file matched the state before the proof (checked outside the run), no new file was left, the config edit was still there. Each ran on four earlier drafts as well, with the same outcome.
