---
name: cj-to-tickets
description: Slice a posted spec into small vertical tickets with blocked-by edges, filed as sub-issues of the spec's issue. Step 3 of the spec-driven flow.
disable-model-invocation: true
---

# To tickets

Step 3 of the spec-driven flow, used in the big tier when the spec needs more than one ticket. Each ticket becomes one small draft PR in `cj-implement`, so a ticket's size is a PR's size.

Call the Skill tool with "cj-project-context" for: repos, glossary, issue-tracker, conventions.

## Steps

1. **Read** the issue, its latest `## Spec` comment and the comments after it, and the affected code. No spec comment: stop and name `/cj-to-spec <issue>`.
2. **One ticket?** The spec fits one fresh context window in one repo: say so, file nothing, propose no title change, and name `/cj-implement <issue>`.
3. **Existing work.** Call the Skill tool with "cj-filing-issues" (Find) with the spec's modules and entities. Open work this builds on becomes a blocked-by edge; a duplicate shrinks the scope and goes to the user.
4. **Slice.**
   - Each ticket is a **tracer bullet**: a narrow but complete path through every layer it touches (data, logic, UI, tests), verifiable on its own and green on main without the tickets after it, small enough for one fresh context window and one quick review (aim for 100 to 200 changed lines without tests; never over about 400). Behaviour that must not show yet goes behind a flag or stays unwired.
   - Preparatory refactoring ("make the change easy, then make the easy change") is its own first ticket.
   - **Several repos** (for example app and backend): the interface between them (schema, API signature, event format) is the first ticket, in the repo that owns it; the vertical slices in each repo build on it.
   - A wide mechanical change (rename, retype across many call sites) goes expand, migrate in batches, contract, each its own ticket.
   - Every ticket names the spec's AC ids it delivers. Every AC belongs to exactly one ticket.
5. **Ticket body**, passed to `cj-filing-issues` as the body-shape preset instead of its task shape (its writing rules still apply): what to build (behaviour), the AC ids with their text copied from the spec (one is enough), blocked by, a link to the spec comment. No file paths, no implementation steps.
6. **Fresh eyes.** Dispatch a subagent with no session context, only the spec and the ticket bodies. Per ticket: could it implement this from ticket plus spec alone, what is open, which AC is missing or doubled. Fix and repeat, at most 2 rounds.
7. **Show the slice** as a numbered list (title, repo, blocked by, AC ids, what it delivers). Write it scannable: the first line is the result or the decision needed. One sentence per bullet, at most 25 words. A list has 3 to 5 items, never more than 7; a longer one splits under two headings or goes onto a page. Three or more items go one per line, never inside a sentence. No nested lists, and no tables unless this skill names one; code, log and command blocks and the `Next:` block keep their own shape. Bold only as a label at the start of a line, code font for paths and commands. Headings name the topic in the reader's terms, not generic labels like Scope or Process. No made-up abbreviations: write glossary terms out unless the glossary defines the abbreviation as the term; code identifiers, paths and issue or PR numbers stay. Before a question, the reply text holds the full list with details, numbered; with nothing to list, one line of context is enough. A question has 3 options, a fourth only as a real alternative, the recommended one first; with only one real option, the proposal and something else. Each option names its item and says in full words what it means and its one consequence, so the user decides without scrolling up. Ask the question through the harness's choice tool where one exists, with the list in the reply text above it, never as options typed into the reply; without a choice tool, number the options 1), 2), 3). Then every ticket's full body, and ask one question at a time, each as a choice: granularity right, then edges right, then merge or split anything. More than four tickets: call the Skill tool with "cj-html-report" for a page with the blocked-by graph. Iterate until the user says go.
   - For a split into multiple sub-issues, the publication preview includes the parent's current title, proposed title and concrete reason: it will hold the shared spec for the listed sub-issues. Propose `[Epic] ` followed by the current title, preserving the rest exactly. If it already starts with `[Epic] `, show the same title and say no title write is needed. Use this prefix only, not a new Epic label.
   - Refresh the preview from the current title and split after a revision, before asking for approval again. If it now fits one ticket, return to step 2 and withdraw the Epic proposal.
   - Call the Skill tool with "cj-posting" for publication. The user's go must cover the shown split and title change. It carries the yes for filing, linking, setting fields and that title change into `cj-filing-issues`. Approval still pending or split rejected: no publication or title write.
8. **File.** Call the Skill tool with "cj-filing-issues" (File) with the batch and these presets: the body shape from step 5, each ticket a sub-issue of the spec's issue (also across repos), blocked-by edges as approved, project fields equal to the parent's, parent body untouched (the sub-issue list shows the children). As part of publishing the approved multiple-ticket split, apply the approved parent title through `cj-posting`; skip the title write if the prefix is already present. Read back the title as well as the edges.

Done when the parent title and every edge read back from GitHub match the approved preview and slice and the reply ends with the next step: `/clear`, then `/cj-implement <issue>`, run once per repo that has tickets. Write the next step under a bold "Next:" (in the reply's language) as a numbered list, one command per line, each followed by ` · ` and its reason in a few words. A step that names a PR, issue or comment carries its full link, even when the link appears earlier in the reply. No table.
