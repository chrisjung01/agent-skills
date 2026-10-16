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

## 8. Kill criterion once every row is met

**Setup:** as scenario 4, the rerun ends with every row met. #412 has a `## Challenge` comment whose kill criterion reads "Under 5 % of chat users open a push within 7 days; 2026-12-15". `issue-tracker` names a project board with a date field "Due".
**Prompt:** "/cj-accept #412"
**Expected:** one issue filed through `cj-filing-issues` after a yes, with the signal, the 5 % threshold and the due date 2026-12-15 in the body, #412 linked in its body, no parent, the board's "Due" field set to 2026-12-15; the Acceptance comment names it. Scenario 3's run, which has a fix-now row left, files no kill issue.

## 9. No challenge comment, no deviations

**Setup:** as scenario 8, but #412 has no `## Challenge` comment and no `Deviation:` comment.
**Prompt:** "/cj-accept #412"
**Expected:** no kill issue filed; the Acceptance comment says there is no challenge comment and nothing was filed, and its deviation line reads "none"; nothing is written to any branch.

## 10. Deviations sorted

**Setup:** as scenario 8. #412 holds three comments: "Deviation: push payloads go through the backend queue instead of direct FCM calls | the app has no server key | moving back means a new backend endpoint and a migration", "Deviation: the setting is called Quiet chat, not muted chat | the OS already uses Mute | rename later is one string", "Deviation: tap handler in `NotificationRouter`, not `ChatScreen` | one place for every push type | a move".
**Prompt:** "/cj-accept #412"
**Expected:** the queue becomes an ADR, Quiet chat a glossary term, the handler move is only listed. ADR and glossary text are shown before anything is written; after a yes they are committed on the last PR's branch and pushed to `#421`, without the local merge of `main`. The Acceptance comment lists all three with where each went.

## 11. Deviation after every PR is merged

**Setup:** as scenario 5, every row met. #412 holds the queue deviation from scenario 10 only.
**Prompt:** "/cj-accept #412"
**Expected:** the ADR text is shown first; after a yes it goes on a new branch from `origin/main` and into its own small draft PR that names #412; nothing is pushed to `main` or to a merged branch.

## 12. Declined follow-up turns into fix now

**Setup:** as scenario 8, but the rerun's muted row is asked again, still not met, and the user picks follow-up. On the filing question the user files the kill issue and declines the follow-up, then picks fix now for the muted row.
**Prompt:** "/cj-accept #412"
**Expected:** the muted row's choice is asked again before anything is filed; once it is fix now, no kill issue is filed, no ADR or glossary text is written, the changed comment without kill and deviation lines is shown before the posting question; next step `/cj-implement #412`.

## 13. User asks the agent to check, app change

**Setup:** as scenario 3, PR #421 open at head `abc1234`, no `## Acceptance` comment. The PR adds unit tests for AC-1, AC-2 and AC-4 and a layout script for the foreground row; nothing in the repo shows whether a muted chat's phone buzzes.
**Prompt:** "/cj-accept #412 prüf alles automatisch"
**Expected:** no row asked one per turn; each row checked on `abc1234` with the cheapest evidence (the tests, the script, a short probe); no sessions played by subagents. One reply lists every row with its evidence, then one yes/no question whether to record them as met; the muted row, without evidence, is then asked by hand as in step 3. The comment opens `Checked by the agent at the user's request on <#421 link> at abc1234.`, never "Checked by hand", and each observation names its evidence.

## 14. User asks the agent to check, skill text change

**Setup:** spec #96 in a skills repo changes only a `SKILL.md` and adds `docs/tests/<skill>-scenarios.md`; PR #123 open at `8e7dc22`. Its Results section records that `cj-implement` ran every scenario against the new text, all met expected. The spec has 13 ACs and 7 edge rows, all about what an agent following the text does.
**Prompt:** "/cj-accept #96 einmal bitte automatisch prüfen"
**Expected:** evidence per row is the changed line of the skill text, a command for file rows, or the recorded scenario result that covers it; no scenario re-run, no role-play sessions with subagents. One reply with every row and its evidence, one yes/no question, then the comment as in scenario 13.

## Results

2026-10-13, fresh subagents reading the texts literally.

- Before (flow skills without `cj-accept`, free-text prompt): scenarios 1 to 6 missed expected. Nothing gates on open PRs or open tickets, asks row by row, records a result or names a rerun.
- After, first version: scenarios 1, 2, 3, 5, 6 met expected; 4 missed expected (the rerun asked the follow-up row again). Also found: the checked sha could be the local merge commit, "merge the last PR" with no PR open, the context call before the gate.
- Fixed: the checked sha is the PR head, the rerun asks only fix-now rows and rows the diff touches (AC-7 changed after the user's yes), merge named only while a PR is open, the gate first.
- After the fix: scenarios 4 and 5 met expected.
- Review rounds 1 and 2 (fresh reviewer): gate on ticket state and open PRs, follow-up Find before File with one carried yes, carried rows keep their observation, declined follow-ups, merge conflict named as next step. Scenario 7 added for the follow-up cases.
- After the review fixes, all scenarios on the final text: 1 to 7 met expected.
- 2026-10-13, kill issue and deviations (#55). Before: scenarios 8 to 11 missed expected; the text never read the challenge or `Deviation:` comments. After, first version: 1 to 11 met expected. Also found: a declined follow-up turned fix now after the records were written, the records PR had no yes, the kill body asked for a fact the criterion lacks. Review round 1: the kill issue could be filed before that row turned fix now, an interrupted run filed the kill issue twice, the records PR was not shown. Scenario 12 added; all fixed. After the fixes: 1 to 12 met expected, graded before two wording fixes in step 4 (Find loads `cj-filing-issues`, a new glossary is shown). Review round 2: a rerun after every PR merged would draft the records branch again; it is now reused.
- 2026-10-16, checked by the agent on request (whetstone, sessions b5b34fc9, 756b318c, 8a09d650). Before: 13 and 14 missed expected; the text has no such path, so the request is ignored and every row asked by hand (6 and 20 turns), comment "Checked by hand". After: 13 and 14 met expected, evidence from tests, script and recorded scenario results, no subagents, one yes/no. Also found: a mixed run's opening line claimed the agent checked rows the user checked; rows checked by hand now end with "(by hand)".
