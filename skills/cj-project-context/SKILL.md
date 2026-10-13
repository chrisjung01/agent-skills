---
name: cj-project-context
description: Use when a skill or task needs a fact about the project or organisation that the code alone does not answer, such as check commands, commit or branch conventions, issue labels and templates, the CI gate, team ownership, chat channels, the error tracker, analytics, goals or where decisions are documented.
user-invocable: false
---

# Project context

Skills in this repo name the **facts** they need, never their values. This skill finds the values for the project at hand, so one skill works in any repo and any organisation. A fact has exactly one of three states: **found** (with its source), **asked** (the user answered), or **missing**. A missing fact skips the step that needs it, and the report says so.

## Lookup order

Work down the ladder per fact; stop at the first rung that answers it.

1. **Already loaded.** `AGENTS.md`, `CLAUDE.md`, personal instructions and memory, plus every doc they point to. In the repo, read all of `docs/agents/*.md` if the folder exists.
2. **The environment.** Look it up where the "Found in" column of the fact table points. A value read from a config file beats a value written in a doc about that file.
3. **Connected tools.** Search chat, docs, the tracker or the error tracker with the fact's own words ("sentry alerts", "OKR", "agreements").
4. **The user.** Ask for the facts still open one at a time, each as a choice: the proposed answer built from rungs 1 to 3 first, then the alternatives you found, then "missing, skip the step". Use the harness's choice tool where it offers one. Write it scannable: the first line is the result or the decision needed. One sentence per bullet, at most 25 words. A list has 3 to 5 items, never more than 7; a longer one splits under two headings or goes onto a page. Three or more items go one per line, never inside a sentence. No nested lists, and no tables unless this skill names one; code, log and command blocks and the `Next:` block keep their own shape. Bold only as a label at the start of a line, code font for paths and commands. Headings name the topic in the reader's terms, not generic labels like Scope or Process. No made-up abbreviations: write glossary terms out unless the glossary defines the abbreviation as the term; code identifiers, paths and issue or PR numbers stay. Before a question, the reply text holds the full list with details, numbered; with nothing to list, one line of context is enough. A question has 3 options, a fourth only as a real alternative, the recommended one first; with only one real option, the proposal and something else. Each option names its item and says in full words what it means and its one consequence, so the user decides without scrolling up.
5. **Record.** Answers from rung 3 or 4 that will be needed again: offer to write them down. The user picks the place: the repo (`docs/agents/facts.md`, shared with the team) or a personal file their instructions already point to. Show the text first; write after a yes.

A value from another project, an earlier session or general knowledge is not a source. Leave the fact missing instead.

## Fact table

| Fact | What it answers | Found in |
|---|---|---|
| `repos` | which repos belong to the product and which track each owns (app, backend, ...) | `git remote -v`, `gh repo view`, `AGENTS.md` |
| `commands` | analyze or lint, a targeted test run, the full suite, codegen, and commands to avoid with the reason | `AGENTS.md`, package scripts, `Makefile`, the CI workflow |
| `conventions` | commit message and branch format, where worktrees go, language of issues, PRs and review replies | `AGENTS.md`, commit hooks (`lefthook.yml`, `.husky/`), PR title checks in `.github/workflows/` |
| `architecture` | layers from screen to data, where user-visible strings live, downstream apps or variants, end-to-end test flows and their selector rule | `AGENTS.md`, `README.md`, folder layout |
| `standards` | agreements a reviewer checks, including guideline docs the instructions name only for some tasks, and which test goals apply to which files | `AGENTS.md`, `CODING_STANDARDS.md`, `CONTRIBUTING.md`, every guideline folder `AGENTS.md` points to, lint config |
| `issue-tracker` | issue templates, PR template and its checklist, labels and their meaning (type, impact or severity, team), project board and its fields | `.github/ISSUE_TEMPLATE/`, `.github/PULL_REQUEST_TEMPLATE.md`, `gh label list`, `gh project list --owner <owner>`, `docs/agents/issue-tracker.md` |
| `ci` | the main workflow, the aggregate gate job, notification-only jobs, end-to-end or device jobs and how they retry | `.github/workflows/*.yml` (`needs:` of the gate job), the workflow's own comments |
| `releases` | how a release is tagged, so a commit maps to a version | `git tag --sort=-creatordate | head`, release workflow |
| `teams` | which team owns which area, and its label | facts file, label descriptions, `CODEOWNERS` |
| `core-flows` | the flows whose breakage is an incident, and the incident process | facts file |
| `channels` | where error alerts land, where support reports arrive, the team channel | facts file; else chat search for the alert bot or "support" |
| `error-tracker` | organisation, projects, which environments count as production | facts file; else the error tracker's own org and project list |
| `analytics` | the product analytics tool, how events are named in code, the filter for internal and test users; the business-data tool | `AGENTS.md`, the tracking code, facts file |
| `product-docs` | where product decisions, goals (OKRs) and team agreements are documented, and the current goal period | facts file; else docs search |
| `glossary` | domain terms, user roles and personas | `GLOSSARY.md`, `docs/agents/domain.md`, `AGENTS.md` |

A skill may need a fact that is not in this table. Treat it the same way and name it plainly.

## Output

Before the calling skill continues, show one line per fact it asked for. Inside a dialogue (grilling), fold them into one sentence before the next question instead.

```
<fact>: <value> (source: <file, command, tool or "asked">)
<fact>: missing, skips <step>
```

## Common mistakes

| Mistake | Instead |
|---|---|
| Filling a gap from memory of another project | Leave it missing and ask. |
| Asking the user something `gh` or a file answers | Rungs 1 to 3 first. |
| Asking for several facts in one turn | One fact per question, as a choice with the proposed answer first. |
| Writing the answer into `AGENTS.md` without asking | Rung 5: show, user picks the place, then write. |
