---
name: cj-lint-from-findings
description: Use when review findings, review threads or a retro name a mechanical mistake (banned API, import shape, file location, fixed code pattern) that a linter, hook or CI check could catch next time.
user-invocable: false
---

# Lint from findings

A mistake a linter could have caught comes back in the next PR unless a check stops it. This skill turns each mechanical finding into the cheapest check that holds, proves the check fires, and lets the user decide where it goes. The caller passes the findings (`id, location, title, problem`) and per finding the commit that fixed it, or `uncommitted`.

Call the Skill tool with "cj-project-context" for: commands, standards, conventions.

## Steps

1. **Filter.** Keep a finding only when a check can spot it in the source text without knowing intent: a banned API or import, an import shape, a file in the wrong place, a fixed code pattern. Smells and judgement calls drop out; they belong in the standards file. So does a rule that would catch only this one instance (one banned name), not the class of mistake. Nothing left: say "nothing to mechanize" in one line and stop, without a question.
2. **Inventory.** From `commands` and `standards`: the linters, their config files, the lint command, whether it fails on warnings, and the hook or CI step that runs it. Run the lint command once on one file: it does not run (missing dependencies, unknown command), so name the error, offer every finding as an issue, and never install dependencies to make it run. No linter: every finding starts at rung 3.
3. **Draft** on the first rung that holds:
   1. a rule the installed linter already has, enabled or configured
   2. a generic pattern rule of that linter (restricted imports, restricted syntax, banned API) with the finding's pattern
   3. a grep script in the repo's script folder, run by the hook or CI step that runs the lint command
   4. a custom plugin rule: draft no code; it becomes an issue draft in step 5
4. **Prove** in the working tree and leave nothing behind:
   1. Note `git status --short`. Copy every file you will touch (lint config, hook, CI file) to a scratch location outside the repo.
   2. Add the rule.
   3. Write a probe file with the faulty code next to the original file, so path-scoped config applies. Source, first that exists: the file before the fix (`git show <fix>^:<path>`, or `git show HEAD:<path>` when the fix is `uncommitted`), the code quoted in the finding, code written from the finding text. Give it the original's extension and a `lint-probe-` prefix, so the linter's globs include it.
   4. Run the lint command on the probe: it must report this rule. Run it on the fixed file: it must not report this rule. Run it on the whole repo and count this rule's hits, the probe left out.
   5. Save the change as a diff against the saved copies (not against `HEAD`, which would carry earlier uncommitted edits), delete the probe, copy the saved files back. `git status --short` matches the note from 4.1 and each restored file is byte-identical to its copy (`cmp`); a difference is fixed before anything else.

   A rule that misses the probe or fires on the fix drops one rung. No probe source at all: the finding is "not proven" and offered as an issue.
5. **Show.** Write it scannable: the first line is the result or the decision needed. One sentence per bullet, at most 25 words. A list has 3 to 5 items, never more than 7; a longer one splits under two headings or goes onto a page. Three or more items go one per line, never inside a sentence. No nested lists, and no tables unless this skill names one; code, log and command blocks and the `Next:` block keep their own shape. Bold only as a label at the start of a line, code font for paths and commands. Headings name the topic in the reader's terms, not generic labels like Scope or Process. No made-up abbreviations: write glossary terms out unless the glossary defines the abbreviation as the term; code identifiers, paths and issue or PR numbers stay. Before a question, the reply text holds the full list with details, numbered; with nothing to list, one line of context is enough. A question has 3 options, a fourth only as a real alternative, the recommended one first; with only one real option, the proposal and something else. Each option names its item and says in full words what it means and its one consequence, so the user decides without scrolling up. Per finding, numbered: its title, the rung and rule, the diff, and the line `fires on probe, fix clean, <N> hits in repo`. With N > 0, list the hits as `file:line`, at most 10, then the count of the rest. Rung 4 and unproven findings show the issue draft instead.
6. **Ask** one question per finding, as a choice, naming the finding by its number and a few words:
   - **own PR** (recommended): a new branch from the default branch with the rule. With N > 0 the hits go into the PR body, and the option says how new cases get blocked: at `warn` level when the lint command fails on warnings; otherwise the PR fixes the hits and the rule lands at `error`. A rung-3 script with N > 0 checks only files changed against the default branch.
   - **current branch**: the rule goes into the caller's working tree, same `warn` handling.
   - **issue**: call the Skill tool with "cj-filing-issues", the diff and the proof line in the body.
   - **drop**.

   Rung 4 and unproven findings get two options, issue (recommended) and drop; this overrides the three-option rule above.
7. **Do what got a yes.** Branch and commit by `conventions`. Call the Skill tool with "cj-posting" before any push, PR or issue.

Done when every mechanical finding has its rule applied, an issue, or a no, and the working tree holds only what got a yes.
