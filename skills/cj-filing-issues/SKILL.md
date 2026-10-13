---
name: cj-filing-issues
description: Use when a GitHub issue is about to be created or a report needs its duplicates, related issues and dependencies found first, whether a bug, a follow-up, a flaky test, a task or an epic with sub-issues, from any skill or a direct request ("mach ein Issue", "leg ein Ticket an", "gibt es dazu schon ein Issue?").
---

# Filing issues

Two entry points. **Find** answers "is this already tracked, and what does it touch?" before anyone decides. **File** creates issues and wires their relations so the tracker shows the real dependency graph. The calling skill passes its **presets** (kind, title convention, template, body shape, labels, parent, parent body untouched, blockers, project fields); everything it leaves open comes from the repo.

Call the Skill tool with "cj-project-context" for: repos, conventions, issue-tracker, releases, glossary.

Commands for both entry points: [COMMANDS.md](COMMANDS.md).

## Find

Input: what the issue is about (screen or module, symptom or goal, in glossary terms), and code paths when known.

1. Search issues in every repo from `repos`, all states, with at least two keyword sets: one in the issue language, one in English. Search by module and file name too.
2. Search open PRs that touch the same files or module.
3. Class every match:

| Class | Meaning |
|---|---|
| duplicate | open, same trigger or same goal |
| fixed | closed, same trigger. Compare the fix's first release tag with the reporter's or affected version (unknown: ask). Released: a new occurrence is a regression, linked to it. Not released: duplicate, "fixed in <version>, pending release". No release tags in the repo: related, and say the release state is unknown |
| blocked by | open work that changes the same code or behaviour and should land first; the new issue builds on it or its fix would be overwritten by it |
| blocks | open work that needs this one first |
| parent | an open epic whose scope contains this one |
| related | same area, different trigger; linked in the body only |

Direction rule: the issue that needs the other's change is blocked by it. When unsure which lands first, class it related and say why.

Matches that fit no class stay out of the table; say how many were dropped. Done when every kept match has a class with a one-line reason, and the caller has the table.

## File

Input from the caller: one or more drafts (kind, title, body sections) and its presets.

1. **Body.** A request that cites a source ("as the spec says") gets the source checked first; a mismatch goes to the user before drafting. Draft each issue by its kind and the writing rules in [BODY.md](BODY.md); every check at its end passes before step 5.
2. **Template.** The matching file in `.github/ISSUE_TEMPLATE/` for the kind. `gh` cannot fill YAML forms: use the form's sections as headings in a Markdown body. Language from `conventions`.
3. **Labels.** Only labels that exist (`gh label list`), as the presets and `issue-tracker` say. A needed label that does not exist: ask; labels are never created silently. A label that is a judgement call (who implements it, a priority) is proposed with its reason in the question.
4. **Relations.** From Find, or the caller's plan for a batch: parent, blocked by, blocks; related goes into the body as a list of links. Several issues with edges between them: show the graph (Mermaid in chat; a page through `cj-html-report` when the caller asks for one).
5. **Show and ask.** Call the Skill tool with "cj-posting": per issue its title, repo, labels, relations and body. A caller that states it carries the yes for this batch skips the question.
6. **Create** in dependency order (blockers and parents first, so edges can name real numbers), then **link** (sub-issues, blocked-by), edit a parent's body to link its children once they exist (unless the presets keep the parent body untouched), and set the **issue type** and **project fields** (`issue-tracker`; the scripts in `scripts/` for a batch). No board configured: skip it and say so. A field `gh` cannot set: one-line instruction for the user.
7. **Read back** every parent and blocked-by edge from GitHub and compare it with the plan. A wrong direction blocks the team: fix it before reporting.

Done when every issue exists with its labels, every edge reads back as planned, and the report lists each issue with its link and relations. Write it scannable: the first line is the result or the decision needed. One sentence per bullet, at most 25 words. A list has 3 to 5 items, never more than 7; a longer one splits under two headings or goes onto a page. Three or more items go one per line, never inside a sentence. No nested lists and no tables, unless this skill shows that shape; code, log and command blocks and the `Next:` block keep their own shape. Bold only as a label at the start of a line, code font for paths and commands. Headings name the topic in the reader's terms, not generic labels like Scope or Process. No made-up abbreviations: write glossary terms out unless the glossary defines the abbreviation as the term; code identifiers, paths and issue or PR numbers stay. Before a question, the reply text holds the full list with details, numbered; with nothing to list, one line of context is enough. A question has 3 options, a fourth only as a real alternative, the recommended one first; with only one real option, the proposal and something else. Each option names its item and says in full words what it means and its one consequence, so the user decides without scrolling up.
