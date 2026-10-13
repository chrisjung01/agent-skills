# Scenarios: cj-accept

Paper exercises. The agent under test gets the skill text and the tool output below and describes its next steps and commands; it runs nothing. "Before" is a run with the flow skills as they were before `cj-accept` existed and the prompt "check the whole spec by hand before the last merge".

Shared setup: repo `acme/shop-client`, default branch `main`. Spec #412 "Push notification on new message" has a `## Spec` comment with AC-1 to AC-4 and an edge-case table of two rows (muted chat, app in foreground). Tickets #413 and #414, #414 blocked by #413.

## 1. Two PRs still open

**Setup:** draft PR #420 (`Closes #413`) and draft PR #421 (`Closes #414`, `Stacked on #420`) are both open.
**Prompt:** "/cj-accept #412"
**Expected:** stops before any worktree or question; names #420 and #421 with links and says to merge #420 first and keep #421 for acceptance. Nothing posted.

## 2. A ticket without any PR

**Setup:** #413 merged through PR #420; #414 has no branch and no PR.
**Prompt:** "/cj-accept #412"
**Expected:** stops and names `/cj-implement #412`; does not take #420's state as "every PR merged" and does not start acceptance on `main`.

## 3. First run on the last PR

**Setup:** PR #420 merged, PR #421 open, head `abc1234`. No `## Acceptance` comment yet. The user answers AC-1 met, AC-2 met, AC-3 "not met: the tap opens the inbox, not the chat" and picks fix now, AC-4 met, muted chat "not met, it still buzzes" and picks follow-up, foreground met.
**Expected:** a detached worktree of `origin/agent/414-…` with `origin/main` merged in, its path and how to start the app; six questions, one row per turn, ACs first, then the two edge-case rows; a second question for each failed row with exactly "Fix now" and "Follow-up"; the follow-up filed through `cj-filing-issues` with #412 linked in its body and no parent; after a yes, one `## Acceptance` comment with six rows, the sha `abc1234`, AC-3 "fix now" and the muted row with the follow-up's number. Next step `/clear`, `/cj-implement #412`, then `/cj-accept #412`; no merge, no close.

## 4. Rerun after a fix

**Setup:** scenario 3's comment is posted. A fix commit on #421 changes only the tap handler; head is now `def5678`.
**Prompt:** "/cj-accept #412"
**Expected:** reads `git diff abc1234 def5678`; asks AC-3 (failed) and the row whose behaviour the tap handler changes, each named with its reason, and offers once to add rows; does not ask AC-1, AC-4 or foreground again; the muted row keeps its follow-up; posts a new comment with all six rows, carried rows linking the first comment. All met: next step is merge #421, close #412, `/cj-retro`.

## 5. Every PR already merged

**Setup:** #420 and #421 merged, no `## Acceptance` comment.
**Prompt:** "/cj-accept #412"
**Expected:** runs on a detached worktree of `origin/main`; a failed row gets no "Fix now" question and becomes a follow-up; next step closes #412 and names `/cj-retro`.

## 6. Spec without an edge-case table, one ticket

**Setup:** spec #430 has AC-1 and AC-2, no edge-case table, no sub-issues; one open PR #431 `Closes #430`.
**Prompt:** "/cj-accept #430"
**Expected:** #431 is the last PR; exactly two row questions; the comment has two rows.

## 7. Follow-up declined or already filed

**Setup:** as scenario 3, but `cj-filing-issues` Find returns open issue #440 "Muted chats still buzz" as a duplicate for the muted row; on the posting question the user says yes to the comment.
**Prompt:** "/cj-accept #412"
**Expected:** nothing filed for the muted row; its choice is "follow-up #440". Had Find found nothing and the user declined filing, the row's choice is asked again; the comment never holds `#<n>`.

## Results

2026-10-13, fresh subagents reading the texts literally.

- Before (flow skills without `cj-accept`, free-text prompt): scenarios 1 to 6 missed expected. Nothing gates on open PRs or open tickets, asks row by row, records a result or names a rerun.
- After, first version: scenarios 1, 2, 3, 5, 6 met expected; 4 missed expected (the rerun asked the follow-up row again). Also found: the checked sha could be the local merge commit, "merge the last PR" with no PR open, the context call before the gate.
- Fixed: the checked sha is the PR head, the rerun asks only fix-now rows and rows the diff touches (AC-7 changed after the user's yes), merge named only while a PR is open, the gate first.
- After the fix: scenarios 4 and 5 met expected.
- Review rounds 1 and 2 (fresh reviewer): gate on ticket state and open PRs, follow-up Find before File with one carried yes, carried rows keep their observation, declined follow-ups, merge conflict named as next step. Scenario 7 added for the follow-up cases.
- After the review fixes, all scenarios on the final text: 1 to 7 met expected.
