---
name: cj-lint-from-findings
description: Use when review findings, review threads or a retro name a mechanical mistake (banned API, import shape, file location, fixed code pattern) that a linter, hook or CI check could catch next time.
user-invocable: false
---

# Lint from findings

A mistake a linter could have caught comes back in the next PR unless a check stops it. This skill turns each mechanical finding into the cheapest check that holds, proves the check fires, and lets the user decide where it goes. The caller passes the findings (`id, location, title, problem`) and per finding the commit that fixed it, `uncommitted`, or `none`.

Call the Skill tool with "cj-project-context" for: commands, standards, conventions.

## Steps

1. **Filter.** Keep a finding only when a check can spot it in the source text without knowing intent: a banned API or import, an import shape, a file in the wrong place, a fixed code pattern. Smells and judgement calls drop out; they belong in the standards file. So does a rule that would catch only this one instance (one banned name), not the class of mistake. Nothing left: say "nothing to mechanize" in one line and stop, without a question.
2. **Inventory.** From `commands` and `standards`: the linters, their config files, the lint command, whether it fails on warnings, and the hook or CI step that runs it. Run the check-only form of the lint command (never a fix or format variant) once on one file: it does not run (missing dependencies, unknown command, a config that does not load), so name the error, offer every finding as an issue, and never install dependencies in the caller's checkout to make it run. No linter: every finding starts at rung 3.
3. **Draft** on the first rung that holds:
   1. a rule the installed linter already has, enabled or configured
   2. a generic pattern rule of that linter (restricted imports, restricted syntax, banned API) with the finding's pattern
   3. a grep script in the repo's script folder, run by the hook or CI step that runs the repo's checks (`commands`); with none, the diff adds a CI step or a pre-commit entry for it, and step 5 says so
   4. a custom plugin rule: draft no code; it becomes an issue draft in step 5
4. **Prove** in the working tree and leave nothing behind:
   1. Note `git status --short` and `git diff HEAD | git hash-object --stdin`. Copy every file you will touch (lint config, hook, CI file) to a scratch location outside the repo.
   2. Add the rule, or write the rung-3 script.
   3. Write a probe file with the faulty code next to the original file, so path-scoped config applies; for a file-location finding, at the path before the fix. Source, the first that holds the code the finding names: the file before the fix (`git show <fix>^:<path>`, or `git show HEAD:<path>` when the fix is `uncommitted`, or the current file when the fix is `none`), the code quoted in the finding, code written from the finding text. Give it a `lint-probe-` prefix and the original's full suffix (`.test.ts`, not `.ts`); a file-location probe keeps the exact old path and name instead, because a rule may match on the file name.
   4. Run the check-only form of the lint command (never a fix or format variant), or the rung-3 script, on the probe: it must report this rule. Run it on the fixed file: it must not report this rule at the lines the finding names (another instance elsewhere in the file counts as a repo hit); with fix `none`, write a corrected copy from the finding as the second probe. Run it on the whole repo and count this rule's hits, the probe left out.
   5. Save the change as a diff against the saved copies (not against `HEAD`, which would carry earlier uncommitted edits), with new files in full. Delete the probes and new files, copy the saved files back. Both notes from 4.1 match again and each restored file is byte-identical to its copy (`cmp`); a difference is fixed before anything else.

   A probe the linter did not lint (an ignore notice, no other rule ran on it) is a probe problem, not a miss: move or rename it and run again. A rule that misses a linted probe or fires on the fix drops one rung. The lint run errors after the edit (unknown rule, bad options): the draft is at fault; restore (4.5), then fix the draft or drop a rung, and never stop with the edit in place. No probe source at all: the finding is "not proven" and offered as an issue.
5. **Show.** Everything the user reads goes through the `unslop` skill. Write it scannable: the first line is the result or the decision needed. One sentence per bullet, at most 25 words. A list has 3 to 5 items, never more than 7; a longer one splits under two headings or goes onto a page. Three or more items go one per line, never inside a sentence. No nested lists, and no tables unless this skill names one; code, log and command blocks and the `Next:` block keep their own shape. Bold only as a label at the start of a line, code font for paths and commands. Headings name the topic in the reader's terms, not generic labels like Scope or Process. No made-up abbreviations: write glossary terms out unless the glossary defines the abbreviation as the term; code identifiers, paths and issue or PR numbers stay. Before a question, the reply text holds the full list with details, numbered; with nothing to list, one line of context is enough. A question has 3 options, a fourth only as a real alternative, the recommended one first; with only one real option, the proposal and something else. Each option names its item and says in full words what it means and its one consequence, so the user decides without scrolling up. Per finding, numbered: its title, the rung and rule, the diff, and the line `fires on probe, fix clean, <N> hits in repo`. With N > 0, list the hits as `file:line`, at most 10, then the count of the rest. Rung 4 and unproven findings show the issue draft instead.
6. **Ask** one question per finding, as a choice, naming the finding by its number and a few words:
   - **own PR** (recommended): the rule at `error` in a new worktree from the default branch, at the path from `conventions`; the caller's checkout stays as it is. With N from 1 to 10 the PR also fixes the hits; with more, the option names the count and the cleanup becomes an issue, so the rule waits for it. When the lint command ignores warnings, the option may land the rule at `warn` instead: it shows the hits and blocks nothing new. A rung-3 script with N > 0 checks only the files a change touches.
   - **current branch**: the rule lands on the caller's branch the way the caller lands its own fixes: uncommitted when they stay uncommitted; a commit when the caller commits them, then a push question through `cj-posting`, because the caller's own push may be past. Not offered when the caller has no branch of its own (a retro). With N > 0 only at `warn` and only when the lint command ignores warnings; otherwise this option is not offered, because the hits would make the branch fail its own check.
   - **issue**: call the Skill tool with "cj-filing-issues", the diff and the proof line in the body.
   - **drop**.

   Rung 4 and unproven findings get two options, issue (recommended) and drop; this overrides the three-option rule above.
7. **Do what got a yes.** Own PR: set up the new worktree as `commands` set up a fresh checkout, re-apply the rule there (by hand when the diff does not apply), and run the lint command again; a hit count other than the proof's goes to the user before the commit. Then branch, commit and open a draft PR by `conventions` (a normal PR where the repo has no drafts). Call the Skill tool with "cj-posting" before any push, PR or issue.

Done when every mechanical finding has its rule applied, an issue, or a no, and the working tree holds only what got a yes.
