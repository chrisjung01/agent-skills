---
name: cj-implement
description: Implement a spec's tickets as small reviewed draft PRs, one per ticket and stacked where a ticket builds on unmerged work; "one-pr" puts the whole spec on one branch and PR instead. Step 4 of the spec-driven flow.
disable-model-invocation: true
---

# Implement

Step 4 of the flow `cj-challenge` → `cj-grill-with-docs` → `cj-to-spec` → `cj-to-tickets` → `cj-implement` → `cj-retro`. Small PRs reach main faster and get better reviews, so the default is **one ticket, one branch, one draft PR**. Never merge.

## Mode

The argument after the issue picks the mode. No mode given and the spec has more than one ticket in this repo: ask once, before any branch, as a choice with two options where the harness offers one: "One PR per ticket (recommended)" and "One PR for the whole spec". One ticket: no question, both modes give one PR. State the mode in the first line of the reply.

| | `per-ticket` (default) | `one-pr` (argument `one-pr`, "ein PR", "one PR") |
|---|---|---|
| PR unit | one ticket | all tickets of the spec in this repo |
| Branch | `gh issue develop <ticket>`, name `<prefix>/<ticket>-<slug>` | `gh issue develop <spec issue>`, name `<prefix>/<spec issue>-<slug>` |
| Base | the default branch; the open PR branch of its one unmerged blocker in this repo (stacked) | the default branch |
| Done state (resume) | per ticket: closed, or an open PR that closes it (its "Development" link can be missing) | the PR's ticket checklist |
| Links | `Closes #<ticket>`, `Part of #<spec issue>` | `Closes` for each done ticket of this repo, `Part of #<spec issue>` |
| When | the recommended answer | trying it out, or a wide refactor whose steps are not green on their own |

Call the Skill tool with "cj-project-context" for: commands, conventions, issue-tracker. Call the Skill tool with "cj-posting" before any outbound action. The user started this skill for draft PRs, so it carries the yes for the linked remote branches (`gh issue develop`), pushes to them, the draft PRs and their descriptions, deviation comments on the spec's issue, and the restack in "After a merge"; every other outbound action needs its own yes. When a situation tempts you to bend a step, check [WRONG-TURNS.md](WRONG-TURNS.md).

Keep a TODO list with the harness's task tool: one entry per PR unit, each with its steps. Before the final report nothing is open, or the report names it with the reason.

## 1. Read and gate

- Input: the spec's issue, or one of its tickets (then only that ticket).
- Read the issue, its latest `## Spec` comment and every comment after it (deviations, answers), the sub-issues with their blocked-by edges and state, the applicable `AGENTS.md`, the glossary and the ADRs the spec names, and the affected code.
- **No spec comment.** A trivial change (typo, copy, a one-line fix whose expected behaviour the issue states) may use the issue as its spec: say so in one line and go on with its criteria as AC-1… Anything else: stop and name `/cj-grill-with-docs <issue>` and `/cj-to-spec <issue>`.
- The spec has an open question, or a criterion you cannot verify: stop and ask. Code or docs answer facts; only product decisions go to the user.
- No sub-issues (the spec fits one ticket): the spec's issue is the one ticket; both modes give one PR. Leave its title unchanged, including any existing `[Epic] ` prefix. The PR only `Closes #<issue>`, without `Part of`.

## 2. Pick the work

First restack what "After a merge" describes. Then, per ticket of this repo, and for the spec issue in `one-pr`: `gh issue develop --list <n> --repo <its repo>`, and once `gh pr list --repo <its repo> --state open --json number,headRefName,closingIssuesReferences`; skip what the mode's done state marks as done. A linked branch without a PR is an interrupted run: `git fetch`, then continue in the worktree `git worktree list` shows for it, else `git worktree add <path> <branch>` as in step 3.1; never run `gh issue develop` for it again.

A ticket is **ready** when every blocker is closed, or (per-ticket) exactly one blocker is open, is in this repo and has an open draft PR, including one opened earlier in this run: that PR's branch becomes the base. In `one-pr`, a blocker inside the same unit counts as done once its commits are on the branch. A blocker in another repo that is still open, or two open blockers: not ready; the report says why and names `/cj-implement` in the other repo.

Done when you can state the PR units of this run, in blocked-by order, with the base of each.

## 3. Per PR unit

Finish one unit completely, PR included, before the next.

1. **Branch** through the issue the mode names, so it is linked under "Development": `gh issue develop <n> --repo <its repo> --branch-repo <this repo> --base <base> --name <name>` (prefix from `conventions`, else `agent/`). Check it out in its own worktree: `git fetch` and `git worktree add <path> --track -b <branch> <remote>/<branch>`, `<path>` from `conventions`, else the harness's, else a sibling directory of the checkout; the user's checkout only when they ask for it. Then tell the user once: `Worktree: <absolute path> (branch <branch>)`. `HEAD` equals `<remote>/<base>` before the first change. Never overwrite, discard or stash the user's uncommitted changes. The first unit of a spec also commits the glossary or ADR files the spec lists under "Recorded in repo"; the report tells the user to delete uncommitted originals in their checkout.
2. **Build** each ticket of the unit in blocked-by order. Tests first at the spec's seams where the project allows it; for a bug, a test that reproduces it fails before the fix. Targeted tests and typechecks while working. Commit per finished step, self-contained, tests with the logic they cover, the repository's message convention, the ticket number in the message. A mutation check (break the code to see a test fail) runs only on a clean, committed tree; afterwards `git status` is clean again.
3. **Size.** Over about 400 changed lines without tests and generated files: the ticket was cut too big. Say so and propose a split before going on; a preparatory refactoring becomes its own ticket.
4. **Deviations.** A technical decision that differs from the spec (interface, order, a module): a comment on the spec's issue, `Deviation: <decision> | <reason> | <cost if wrong>`, and continue. A change to behaviour, scope or an AC: stop and ask. On a yes, edit the spec comment as `cj-to-spec` describes under "Changing a posted spec" (that yes covers the edit).
5. **Checks.** The full required checks once on the unit's final state, independent commands in parallel; every PR is green on its own. Fix with `git commit --fixup=<sha>` on an unpushed commit, `<sha>` the commit that last changed those lines, else an ordinary commit.
6. **Review.** Mandatory. Call the Skill tool with "cj-reviewing" and dispatch a fresh reviewer with the spec, the unit's AC and edge cases verbatim, the deviation comments, base, diff and check results. Fix legitimate findings the same way, rerun the affected checks, reject only with evidence. Production-code fixes get a second round. No reviewer available: report and stop before pushing.
7. **Acceptance.** Per AC and edge-case row of the unit: evidence from the final state (test name, check with result, or a manual check with its observation) and met or not met. "All tests green" covers nothing without its own test; a skipped test is missing; never change an expectation to match the code. Checks red, findings open or an AC not met: report, no push.
8. **Draft PR.** Fold fixups of unpushed commits (`GIT_SEQUENCE_EDITOR=: git rebase -i --autosquash <upstream>`, `<upstream>` = `<remote>/<branch>` when pushed before, else `$(git merge-base HEAD <remote>/<base>)`). Save `<old-HEAD>` before the rebase, then run `git diff <old-HEAD> HEAD` as its own command, never chained to the push. Not empty: a fixup hit the wrong commit; `git reset --hard <old-HEAD>`, retarget, fold again. Plain push only after an empty diff. Open the draft PR against the base, or update its description, from [PR-BODY.md](PR-BODY.md).

After each PR: more ready units and the window still light, take the next; otherwise end with `/clear`, then `/cj-implement <issue>` (plus the chosen mode as argument, so the next run does not ask again).

## 4. Report

Per PR: link, base (stacked on which PR), acceptance table, checks, review outcome. Then deviations, units left and why. Write it scannable: one item per line, its core in a few words first; reason, source or path on an indented line below it, or left out when the user does not need it to decide; group under headings that name the topic in the reader's terms, not generic labels like Scope or Process; no item longer than two lines. The acceptance table keeps its shape. Then the next step: `/clear` + `/cj-implement <issue>` while work is left; once every ticket is merged, ask the user to close the spec's issue (or close it after their explicit yes) and run `/cj-retro`. Write the next step under a bold "Next:" (in the reply's language) as a numbered list, one command per line, each followed by ` · ` and its reason in a few words. A step that names a PR, issue or comment carries its full link, even when the link appears earlier in the reply. No table.

## After a merge (per-ticket stacks)

After a squash merge git no longer recognises the merged PR's commits in the PR stacked on it, and GitHub may already have retargeted that PR to the default branch. Find each open PR of the spec whose description says `Stacked on #<P>` where #P is merged, then: `git fetch`, `git rebase --onto <remote>/<default branch> $(gh pr view <P> --json headRefOid -q .headRefOid) <ticket branch>`, `git push --force-with-lease`, `gh pr edit <n> --base <default branch>`, and drop the Stack line from its description. Only the ticket's own branch, never anyone else's. Step 2 runs this first.
