---
name: cj-accept
description: Check a whole spec by hand on its last open PR before it merges, row by row, and post the result as an "## Acceptance" comment; only the records when the spec says verified is enough. Step 5 of the spec-driven flow.
disable-model-invocation: true
---

# Accept

Step 5 of the spec-driven flow, used when the spec's Acceptance line says by hand; with verified is enough it only writes the records (step 1). Green tests can hide a broken feature, so the user checks every acceptance criterion and edge-case row of the spec by hand, once, on the spec's last open PR, before it merges. That is **Acceptance**; the agent's evidence per PR is **Verification** and belongs to `cj-implement`. This skill builds nothing: a fix goes through `cj-implement`.

Call the Skill tool with "cj-project-context" for: repos, commands, conventions, issue-tracker, after the gate in step 1 passes. Call the Skill tool with "cj-posting" before any outbound action.

## 1. Read and gate

- Input: the spec's issue. Read it, its latest `## Spec` comment and every comment after it, its sub-issues (the tickets) and their state, the latest `## Acceptance` and `## Challenge` comments if there are any, and every `Deviation:` comment.
- **The tickets** are the spec's sub-issues, or the spec's issue itself when it has none. A closed ticket is done.
- **The open PRs of the spec** are the open PRs that close one of its tickets. Look them up in the spec's repo and every repo that holds a ticket: `gh pr list --repo <repo> --state open --json number,url,headRefName,headRefOid,closingIssuesReferences`.
- Gate, first match wins:
  - No `## Spec` comment and no issue body starting with `## Spec`: stop and name `/cj-grill-with-docs <issue>` and `/cj-to-spec <issue>`.
  - An open ticket without an open PR: stop and name `/cj-implement <issue>`.
  - Two or more open PRs: stop. Name the open PRs with links, in blocked-by order, and say to merge all but the last before acceptance. Nothing is asked.
  - Exactly one open PR: it is the **last PR**. A one-ticket spec and `one-pr` mode land here with their single PR.
  - Every ticket closed, no open PR: every PR is merged; acceptance runs on the default branch, and a failed row can only become a follow-up.
- **The Acceptance line**: the last `Acceptance:` line of the latest `## Spec` comment, or of the issue body when it starts with `## Spec`. Only `verified is enough` counts as verified is enough; any other value, or no line, counts as by hand. By hand: go on with step 2.
- **Records-only run.** Verified is enough, and records that no earlier `## Acceptance` comment holds yet: a `**Kill criterion:**` line in a `## Challenge` comment, or a `Deviation:` comment. It runs step 2 without any merge of the default branch and without the line on how to start the feature, skips step 3, and runs steps 4 to 6 without rows, follow-ups or fix-now; its next step names `/cj-retro` only after a session with friction. Verified is enough and nothing to record: say nothing is to be checked or recorded, post nothing, stop.

## 2. Set up

- The worktree is in the repo of the last PR. When that repo is not the one this checkout belongs to, ask the user for its local path.
- `git fetch`, then a detached worktree, so nothing from this step is pushed: `git worktree add --detach <path> <remote>/<last PR branch>`, then `git merge --no-edit <remote>/<default branch>` inside it. With every PR merged: `git worktree add --detach <path> <remote>/<default branch>`. `<path>` from `conventions`, else a sibling directory of the checkout, `../<repo>-accept-<issue>`. A rerun reuses the existing one from `git worktree list`: `git checkout --detach <remote>/<last PR branch>` and the same merge again. A merge conflict: `git merge --abort` and stop; the next step is to merge the default branch into the last PR's branch, then `/cj-accept <issue>` again.
- **The checked sha** is the last PR's `headRefOid`, or the default branch head when every PR is merged; never the local merge commit.
- Tell the user once: `Worktree: <absolute path> (<last PR link, or default branch> at <checked sha, short>)`, and how to start the feature: the run or start command from `commands`, plus the "How to test" steps of the spec's PRs. No way to start it found: ask the user once and use the answer.

## 3. Ask every row

- **Rows**: every AC of the spec in order, then every row of its edge-case table. No edge-case table: the ACs only.
- **Checked by the agent.** The user asks for it, in the argument or a reply ("prüf automatisch"): check each row yourself with the cheapest evidence that shows its Then, on the checked sha: a command, test or probe run, or the changed line for a text-only change. Scenario results `cj-implement` already recorded count; re-run nothing they cover, and play no sessions with subagents unless the user asks. Show every row with its evidence in one reply, then ask once, yes/no, whether to record them as met. A row without evidence is asked by hand as below.
- **Rerun.** The latest `## Acceptance` comment with rows names the sha it checked; a records-only comment counts as no earlier check, so every row is asked. Read `git diff <that sha> <checked sha>`. Ask only the rows marked not met with fix now, and the rows whose behaviour that diff changes; carry every other row over unchanged, a follow-up row with its link. Before the first row, list the rows to ask, each with one line on why (failed, or which change touches it); name the default branch's commits since the earlier comment's date (`git log <remote>/<default branch> --since=<date>`), which that diff does not cover; and ask once, as a yes/no choice question with that list directly above it, whether to ask the rows as listed: no lets the user name rows to add, or every row, in their own words.
- **One row per turn.** Show the row verbatim and what to try: the Given as setup, the When as the action, the Then as what to look for. Ask it as a choice question: "Met" or "Not met"; the user's own words are the observation. This question and the next have two options and no recommended one: the agent has no evidence to recommend from, so a bare "yes" gets one follow-up asking which. "Met" without words records "as described". "Not met" without words: ask once what they saw.
- **A row not met** gets a second question, a choice with two options: "Fix now" (built in the last PR through `cj-implement`) or "Follow-up" (a new issue, the last PR merges without it). With every PR merged, only "Follow-up" exists: say so and ask nothing.
- Keep the results in the harness's task list, one entry per row, so a long run loses none.

## 4. Records

Only in a run with no fix-now row left: a fix through `cj-implement` can add deviations, and every run would otherwise record them again. A kill issue or deviation an earlier `## Acceptance` comment already records is carried over with its link, never filed or written twice.

- **Kill issue.** The latest `## Challenge` comment on the spec's issue and its `**Kill criterion:**` line: signal, threshold, date. A date relative to launch counts from today; the body says so. Call the Skill tool with "cj-filing-issues" and run Find for it first; an open kill check for this spec it finds is reused, nothing filed. Else draft one issue for File with these presets: the spec's repo, kind task, title `Kill check: <spec title>`, a body with signal, threshold, due date and links to the spec's issue and the challenge comment, no parent, no blockers, and the board's date field set to the due date (`issue-tracker`; no such field: skip it). No challenge comment, or no kill criterion in it: draft nothing.
- **Deviations.** Call the Skill tool with "cj-grilling" for the ADR test and the ADR and glossary formats; write nothing before the yes in step 5.3. Sort every `Deviation:` comment on the spec's issue, first match wins: hard to reverse, surprising without context and a real trade-off goes to an ADR; a new domain term goes to the glossary; everything else is only listed. Draft the ADR and glossary text in the repo's existing format; an ADR or term already on the target branch for that deviation is not written again. No glossary yet: the new file and the agent-doc pointer `cj-grilling` adds are part of the text shown.
- **Writing them**, after the yes in step 5.3. The last PR open: in the worktree, `git status` clean (else ask before anything), `git checkout --detach <remote>/<last PR branch>`, which leaves the local merge of the default branch behind, write the files, commit as `conventions` says, `git push <remote> HEAD:<last PR branch>`. Every PR merged: the branch `<prefix>/<issue>-acceptance-records` (prefix from `conventions`, else `agent/`); it already exists from an earlier run: build on it and its open PR, and check there what is already written; else a new one from `<remote>/<default branch>`; commit, push, and a draft PR titled `Record decisions from acceptance of #<issue>`, its body naming the deviations and `Part of #<issue>`. A rejected push: stop and report, never force.

## 5. Post

1. **Follow-up drafts.** Call the Skill tool with "cj-filing-issues" and run Find for each follow-up; a duplicate it finds becomes that row's follow-up, and nothing is filed for it. Draft the rest for File with these presets: the spec's repo, kind bug, the spec's issue as related (a link in the body), no parent, no blockers. The follow-up is not a sub-issue of the spec.
2. **The `## Acceptance` comment** on the spec's issue, in the language from `conventions`:

   ```
   ## Acceptance

   Checked by hand on <last PR link, or default branch> at <checked sha, short>.

   | Row | Result | Observation | Choice |
   |---|---|---|---|
   | AC-1 | met | <the user's words> | |
   | AC-3 | not met | <the user's words> | fix now |
   | Edge: <input or state> | not met | <the user's words> | follow-up #<n> |

   **Kill issue:** #<n>, due <date>
   **Deviations:**
   - <decision>: ADR `<path>`
   - <decision>: glossary, <term>
   - <decision>: listed
   ```

   A records-only run replaces the line under the heading with `Verified is enough, per the spec's Acceptance line; no row checked by hand.` and has no table. A run checked by the agent reads `Checked by the agent at the user's request on <last PR link, or default branch> at <checked sha, short>.`, and each observation names its evidence; a row the user checked by hand ends its observation with "(by hand)".

   A row asked again and now met has an empty choice. A carried row keeps its result, observation and choice, followed by "(carried from <link to the comment where it was last asked>)", which replaces an earlier such marker. Every row of the spec appears, none twice; a records-only run has no rows. The kill line otherwise reads "no challenge comment, nothing filed", "no kill criterion, nothing filed" or "declined, nothing filed"; the deviations line reads "none" without `Deviation:` comments. A run with fix-now rows left has neither line.
3. Show the follow-up drafts, the kill issue draft, the ADR and glossary text with its commit message and, when every PR is merged, the records PR's title and body, and the comment (issue numbers as placeholders), all in one reply. Then ask through `cj-posting`, one question at a time; nothing to file or write skips its question:
   1. Filing the follow-ups and the kill issue. A follow-up the user declines: ask that row's choice again while the last PR is open; with every PR merged, the row stays not met with "no follow-up" as its choice. A row that turns fix now: drop the kill issue draft, the records and both lines from the comment, and show the changed comment. Then run File for what got a yes; this skill carries that yes, so File asks nothing.
   2. Writing and pushing the ADR and glossary text, with the records PR when every PR is merged. Write them as step 4 says; declined text leaves its deviation listed only.
   3. Posting the comment. Fill in the numbers, then post. The comment never carries a placeholder. A rerun posts a new comment and never edits the earlier one.

## 6. Report and next step

Report the comment's link, one line per row not met with its choice, the follow-ups and the kill issue with links, and where the ADR and glossary text went. Write it scannable: the first line is the result or the decision needed. Be extremely concise. Sacrifice grammar for the sake of concision. Text others read keeps full sentences. One sentence per bullet, at most 25 words. A list has 3 to 5 items, never more than 7; a longer one splits under two headings or goes onto a page. Three or more items go one per line, never inside a sentence. No nested lists, and no tables unless this skill names one; code, log and command blocks and the `Next:` block keep their own shape. Bold only as a label at the start of a line, code font for paths and commands. Headings name the topic in the reader's terms, not generic labels like Scope or Process. No made-up abbreviations: write glossary terms out unless the glossary defines the abbreviation as the term; code identifiers, paths and issue or PR numbers stay. Before the first question to the user, call the Skill tool with `cj-asking` and ask as it says.

The next step:

- Fix-now rows left: `/clear`, then `/cj-implement <issue>` to build them on the last PR, then `/cj-accept <issue>` again.
- No fix-now row left: merge the last PR (when one is still open) and the records PR (when one was opened), close the spec's issue, then `/cj-retro`, after a records-only run only after a session with friction. Merging and closing are the user's; this skill does neither.

Write the next step under a bold "Next:" (in the reply's language) as a numbered list, one command or action per line, each followed by ` · ` and its reason in a few words. A step that names a PR, issue or comment carries its full link, even when the link appears earlier in the reply. No table. Name the worktree with `git worktree remove <path>` as the last line once no fix-now row is left.

Done when every row has a result (a records-only run has none), every failed row has its choice, the kill issue, the records and the comment are done after a yes or their decline reported, and the reply ends with the next step.
