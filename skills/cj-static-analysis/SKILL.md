---
name: cj-static-analysis
description: Use when a skill or review needs to know who calls or imports a symbol, which symbols a diff leaves without callers, or which findings the project's linter already blocks in CI.
user-invocable: false
---

# Static analysis

Answers two kinds of questions for the calling skill: who calls or imports a symbol, and so which symbols a diff leaves orphaned; and which findings the project's linter already blocks. Every answer names its source, because a `grep` hit and a resolved reference carry different weight.

## Sources

Call the Skill tool with "cj-project-context" for: commands (lint and analyzer), ci, code-index, conventions. Per question, take the first checked source that answers; with none, an unchecked code index, else `grep`:

1. **Code index**: the service the `code-index` fact names, with the commit it indexed.
2. **Analyzer**: a tool that resolves references the way the compiler does: a language server the harness offers, or an analyzer command from `commands`, such as one that reports unused code.
3. **`grep`**: a word-bounded search over the tracked files. Always there, never certain: it misses calls through dependency injection, codegen or a string name, and it hits comments and namesakes.

The analyzer and a current code index are **checked** sources. A code index is current when its indexed commit is the diff's base commit, the merge base (`git merge-base <base> <head>`), not the tip of the base branch; one that does not report its commit is not. An analyzer is checked only for the question it answers: a report of unused code that leaves tests out answers orphaned symbols, not caller lists, which then come from the next source. Such a report often leaves out exported or public symbols; for the ones it leaves out, orphans come from the next source. `grep` and any other code index are **unchecked**, and `grep` at the diff's head fills in what that index has not seen.

Every source looks at the diff's head. When the head is not checked out (someone else's pull request, a stacked base), fetch it (`git fetch <remote> pull/<n>/head` for a pull request) and check it out in its own worktree (`git worktree add --detach <path> <head>`, `<path>` from `conventions`, else a sibling directory of the checkout), tell the user once `Worktree: <absolute path>`, hand the path to the caller as the repo to review, and remove it when the caller is done.

The source line for the caller: `code index <name> @ <indexed commit>` (add `, not the base <base>, unchecked` when it is not current; `@ unknown, unchecked` when it reports no commit), `analyzer <tool>`, or `grep, unchecked`. When the questions have different sources, one per question: `callers: <source>; orphans: <source>`. When the analyzer leaves out exported symbols: `orphans: analyzer <tool>, exported symbols: <next source>`.

## Callers of a symbol

Answer as `<symbol>: <callers as file:line> | <source>`. Calls and imports both count; line numbers are those at the diff's head.

- Code index: its callers, plus the callers the diff adds, minus those it removes; the index has not seen the diff.
- Callers in tests are listed apart. They do not make a symbol used.
- "No other callers" from an unchecked source says so in the same sentence: "no other callers (grep, unchecked)".

## Orphaned symbols in a diff

Look at every symbol the diff adds or changes, and every symbol a removed line called or imported. An **orphaned symbol** is one the diff adds outside test code, or whose last caller outside tests the diff removes, that still exists at the head and that nothing outside tests calls or imports afterwards. A symbol that existed at the base commit without such a caller is none: this is no repo-wide audit.

One line per orphaned symbol: `<symbol> | <file:line> | <source> | finding or question`. A checked source makes it a low finding that names the source. An unchecked source makes it a question ("is `x` reached through a registry, dependency injection or codegen?"), never a finding. With an unchecked source, a symbol reached only through a string name, dependency injection or codegen also gets a question line.

## Findings the linter already blocks

- Run the lint command from `commands` over the changed files that exist at the head and that its config covers, or over the project when it takes no file list. A command that only compiles or type-checks is no lint command; one that reports named lint rules, or the one the CI lint step runs, is, whatever its name. Never a variant that fixes files.
- A violation **blocks** when it makes the CI lint step fail (`ci`): errors, and warnings only where CI fails on warnings. No CI lint step found: nothing blocks. A violation at a lower severity blocks nothing, so a finding about it stays.
- Answer `Lint: <command> -> exit <code>`, then one line per blocking violation: `Lint: <file:line> <rule>`.
- No lint command: `Lint: none, no lint command found`. The command fails to run (a missing tool, a config error, a crash, as opposed to violations found): `Lint: none, <command> did not run: <error line>`. Both count as a repo without a linter, where every documented rule is judged by reading.
