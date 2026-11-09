# Brief

## Window

Each scope keeps its own time: `state/last-brief` for the default scope, `state/last-brief-<scope>` for any other.

- The scope's file exists: from the time it holds.
- It does not: from 00:00 of the last workday before today (Monday → Friday).

## What to read

Only what `state/context.md` lists for the run's scope, and only the healthy sources. Each item keeps its link.

**Chat**
- Direct and group messages from people.
- Mentions of the user anywhere.
- Threads the user wrote in that have new replies.
- Every message in the channels listed under `Chat channels`.
- Bot messages are skipped; the Git host is read directly.

**Git host** (organisations under `Git host`)
- Review requests to the user outside the radar repos, oldest first.
- Issues assigned to the user with activity in the window.
- Board items assigned to the user whose status is ahead of their pull request: review or done on the board while the pull request is a draft, or in progress with no pull request at all. The board API gives no date of the last status change, so compare states instead.

**Pull request radar** (repos under `PR radar` in `context.md`)
- Run `python3 scripts/pr-radar.py <owner/repo> <login>` per repo, with the Git host login from `Me`. Idle means days since the last commit, review or comment by a person; bots never count.
- Listed: every ready pull request of the user; the user's drafts idle longer than `Stale after`; pull requests the user is requested on, reviewed or commented on, idle longer than `Stale after`; every other ready pull request idle longer than `Stale after` without an approval.
- Each line ends with the first next move that fits:
  - CI red: "fix CI" on the user's own, "ask <author> about CI" on others'.
  - Changes requested, last action by the reviewer: "you are up" on the user's own, "<author> is up" on others'.
  - The user's own, no reviewer requested and none reviewed: "request a reviewer".
  - The user's own, open threads with the last word from someone else: "answer threads".
  - The user's own, last action by the user, idle longer than `Stale after`: "nudge <reviewers>".
  - Approved by the user, not merged: "ask <author> what is missing".
  - The user's draft: "finish or park".
  - Someone else's, no review: "review or find a reviewer".
- A stack of pull requests with the same move is one line.
- When ready pull requests have no reviewer requested, one line: `<n> of <m> ready pull requests have no reviewer requested`.

**External tracker** (what the user waits for from another organisation)
- Tickets where the user is assignee, reporter or watcher.
- Moved in the window (status, comment, question): top of its section, the user is up again.
- Open with no change for more than 14 days: add "nudge?".

**Error tracker** (projects under `Error tracker`)
- Issues that are new, regressed or spiking in the window.
- Open issues assigned to the user, any project.

**Docs**
- Personal to-dos: the statuses and date rule in `context.md`.
- Team tasks assigned to the user and open; overdue or due within 3 days first.
- Project papers of the user's team changed in the window; owner, responsible or writer is the user → higher.
- Decision log entries changed in the window; open or in clarification with the user as participant → "You block others".
- Goals page: when `Goals checked` in `context.md` is more than 7 days old, compare the page with `context.md`, list differences under "Rest", and update `Goals checked` and the copied goals.

**Notes vault** (under `Notes vault`)
- Overview notes whose status is the active value: each one's next step.
- Active, and no file in its folder changed for more than 14 days: add "stalled?".
- Active with an empty next step, or with a next step naming a pull request or issue merged or closed in the window: "next step?".
- Active with a repo, and the next step names no open pull request or issue: read the repo's open pull requests and issues and suggest one to do next, with a reason of a few words. Take the first that fits: the user's own open pull request, ready before draft; an issue marked ready to implement or in the earliest milestone; a defect by label or title; the smallest; the most recently updated. Epics, parent issues and issues labelled as deferred count only when nothing else is left. Nothing open → no suggestion.
- A project whose repo field names a repo: its Git host activity in the window, read as for side projects.
- Open tasks in the `Tasks` format, outside the skipped folders: due today or earlier, or within 3 days; overdue first. Tasks without a date are not listed.
- Notes in the inbox folder: one count line when there are any.
- The latest `## Brief` in the daily notes: a line ticked `- [x]` is done; leave its item out unless its source shows activity after that brief.

**Side projects** (repos under `Side projects`): pull requests and issues with activity, CI on the default branch. Nothing → one line "quiet".

## Rank

One item from two sources (a bot message and the Git host, say) is one line, linked to the source of truth `context.md` names.

1. **Focus today**, at most 3: waits on the user AND (a driver is involved OR it moves a goal). Each line ends with its reason: the driver's name or the goal's short name. An item with neither reason is never here, however urgent it sounds. In a scope without `Goals` and `People`: overdue tasks, then tasks due today, then the next step or suggestion of the most recently active project; the reason is the due date or `active <date>`. Each line names the concrete action for today.
2. **Today**: personal to-dos, team tasks and vault tasks not in Focus today; overdue first, then by due date, then undated ones in progress or planned this week.
3. **You block others**: review requests, open decisions with the user, questions to the user in chat.
4. **Pull requests**: the radar, longest idle first; the user's own first, then the ones the user is involved in, then the team's.
5. **Waiting on others**: the external tracker.
6. **Errors**: the error tracker.
7. **Rest**: one line each.
8. **Side projects.**
9. **Projects**: the active projects, stalled first, then the inbox count.

An item that fits several sections stands only in the highest one. An empty section is left out. Chat messages that match nothing above are not mentioned at all, not even as "skipped".

## Reply

Write it scannable: the first line is the result or the decision needed. Be extremely concise. Sacrifice grammar for the sake of concision. Text others read keeps full sentences. One sentence per bullet, at most 25 words. A list has 3 to 5 items, never more than 7; a longer one splits under two headings or goes onto a page. Three or more items go one per line, never inside a sentence. No nested lists, and no tables unless this skill names one; code, log and command blocks and the `Next:` block keep their own shape. Bold only as a label at the start of a line, code font for paths and commands. Headings name the topic in the reader's terms, not generic labels like Scope or Process. No made-up abbreviations: write glossary terms out unless the glossary defines the abbreviation as the term; code identifiers, paths and issue or PR numbers stay. Before the first question to the user, call the Skill tool with `cj-asking` and ask as it says.

```
**Brief <scope> <weekday> <date>**, since <window start>
Missing: <source> (<reason>)
**Focus today**
1. <item> <link> (<driver or goal>)
**Today**
- <due date or status>: <task> <link>
**You block others**
- <item> <link>
**Pull requests**
- [#<number>](<url>) <title>, idle <days> days: <next move>
**Waiting on others**
- <item> <link>
**Errors**
- <item> <link>
**Rest**
- <item> <link>
**Side projects**
- <item> <link>
**Projects**
- <project>: <next step> (<stalled? or next step?>); suggested [#<n>](<url>) <title>: <reason>
- Inbox: <n> notes
```

Every pull request, issue, ticket and error number is itself a markdown link to its page, such as `[#812](<url>)` or `[EXT-9](<url>)`; never a bare number, never a separate link word.

A section with more than 7 items keeps the 7 highest and ends with `+<n> more`.

## Close

Write the time the reads started to the scope's file (see Window), as `YYYY-MM-DDTHH:MM`. Do this after the reply, also when nothing changed.

When the run's scope lists the notes vault and its health check is `ok`:
- Append the brief to today's daily note under `## Brief HH:MM`; create the note when missing.
- In that copy, every line with a next move for the user (Focus today, Today, You block others, Pull requests, Rest, Projects) starts with `- [ ] `; other lines stay as in the reply. No due date on these checkboxes.
- Decisions the user makes on this brief's items later in the same session: append them as bullets under `**Entschieden**` below that brief, then commit as below.
- For each project whose repo had a pull request merged or an issue closed in the window: append `- YYYY-MM-DD: <merged or closed> [#<n>](<url>) <title>` under `## Log` in its overview note, creating the heading at the end when missing.
- When that pull request or issue is the one the next step names (its number or title), empty the next step, so the next brief asks "next step?". Never write a new next step; the user picks it.
- Change nothing else in the vault: no other field, note, move or deletion, unless the user asks for it.
- `Git` is yes: commit only the files written, message `chief-of-staff: brief YYYY-MM-DD`; never push.
