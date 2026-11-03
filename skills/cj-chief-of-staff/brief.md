# Brief

## Window

- `state/last-brief` exists: from the time it holds.
- It does not: from 00:00 of the last workday before today (Monday → Friday).

## What to read

Only what `state/context.md` lists, and only the healthy sources. Each item keeps its link.

**Chat**
- Direct and group messages from people.
- Mentions of the user anywhere.
- Threads the user wrote in that have new replies.
- Every message in the channels listed under `Chat channels`.
- Bot messages are skipped; the Git host is read directly.

**Git host** (organisations under `Git host`)
- Review requests to the user, oldest first.
- The user's open pull requests with red CI, a merge conflict, changes requested or new comments. A stack of pull requests is one line.
- Issues assigned to the user with activity in the window.
- Board items assigned to the user whose status has not changed for more than 5 workdays.

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
- New comments and mentions of the user.
- Project papers of the user's team changed in the window; owner, responsible or writer is the user → higher.
- Decision log entries changed in the window; open or in clarification with the user as participant → "You block others".
- Goals page: when `Goals checked` in `context.md` is more than 7 days old, compare the page with `context.md`, list differences under "Rest", and update `Goals checked` and the copied goals.

**Side projects** (repos under `Side projects`): pull requests and issues with activity, CI on the default branch. Nothing → one line "quiet".

## Rank

One item from two sources (a bot message and the Git host, say) is one line, linked to the source of truth `context.md` names.

1. **Focus today**, at most 3: waits on the user AND (a driver is involved OR it moves a goal). Each line ends with its reason: the driver's name or the goal's short name. An item with neither reason is never here, however urgent it sounds.
2. **You block others**: review requests, open decisions with the user, questions to the user in chat.
3. **Waiting on others**: the external tracker.
4. **Errors**: the error tracker.
5. **Rest**: one line each.
6. **Side projects.**

An item that fits several sections stands only in the highest one. An empty section is left out. Chat messages that match nothing above are not mentioned at all, not even as "skipped".

## Reply

Write it scannable: the first line is the result or the decision needed. Be extremely concise. Sacrifice grammar for the sake of concision. Text others read keeps full sentences. One sentence per bullet, at most 25 words. A list has 3 to 5 items, never more than 7; a longer one splits under two headings or goes onto a page. Three or more items go one per line, never inside a sentence. No nested lists, and no tables unless this skill names one; code, log and command blocks and the `Next:` block keep their own shape. Bold only as a label at the start of a line, code font for paths and commands. Headings name the topic in the reader's terms, not generic labels like Scope or Process. No made-up abbreviations: write glossary terms out unless the glossary defines the abbreviation as the term; code identifiers, paths and issue or PR numbers stay. Before the first question to the user, call the Skill tool with `cj-asking` and ask as it says.

```
**Brief <weekday> <date>**, since <window start>
Missing: <source> (<reason>)
**Focus today**
1. <item> <link> (<driver or goal>)
**You block others**
- <item> <link>
**Waiting on others**
- <item> <link>
**Errors**
- <item> <link>
**Rest**
- <item> <link>
**Side projects**
- <item> <link>
```

A section with more than 7 items keeps the 7 highest and ends with `+<n> more`.

## Close

Write the time the reads started to `state/last-brief`, as `YYYY-MM-DDTHH:MM`. Do this after the reply, also when nothing changed.
