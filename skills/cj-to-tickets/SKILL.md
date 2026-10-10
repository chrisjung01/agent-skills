---
name: cj-to-tickets
description: Slice a posted spec into small vertical tickets with blocked-by edges, filed as sub-issues of the spec's issue. Step 3 of the spec-driven flow.
disable-model-invocation: true
---

# To tickets

Step 3 of the flow `cj-challenge` → `cj-grill-with-docs` → `cj-to-spec` → `cj-to-tickets` → `cj-implement` → `cj-retro`. Each ticket becomes one small draft PR in `cj-implement`, so a ticket's size is a PR's size.

Call the Skill tool with "cj-project-context" for: repos, glossary, issue-tracker, conventions.

## Steps

1. **Read** the issue, its latest `## Spec` comment and the comments after it, and the affected code. No spec comment: stop and name `/cj-to-spec <issue>`.
2. **One ticket?** The spec fits one fresh context window in one repo: say so, file nothing, and name `/cj-implement <issue>`.
3. **Existing work.** Call the Skill tool with "cj-filing-issues" (Find) with the spec's modules and entities. Open work this builds on becomes a blocked-by edge; a duplicate shrinks the scope and goes to the user.
4. **Slice.**
   - Each ticket is a **tracer bullet**: a narrow but complete path through every layer it touches (data, logic, UI, tests), verifiable on its own and green on main without the tickets after it, small enough for one fresh context window and one quick review (aim for 100 to 200 changed lines without tests; never over about 400). Behaviour that must not show yet goes behind a flag or stays unwired.
   - Preparatory refactoring ("make the change easy, then make the easy change") is its own first ticket.
   - **Several repos** (for example app and backend): the interface between them (schema, API signature, event format) is the first ticket, in the repo that owns it; the vertical slices in each repo build on it.
   - A wide mechanical change (rename, retype across many call sites) goes expand, migrate in batches, contract, each its own ticket.
   - Every ticket names the spec's AC ids it delivers. Every AC belongs to exactly one ticket.
5. **Ticket body**, passed to `cj-filing-issues` as the body-shape preset instead of its task shape (its writing rules still apply): what to build (behaviour), the AC ids with their text copied from the spec (one is enough), blocked by, a link to the spec comment. No file paths, no implementation steps.
6. **Fresh eyes.** Dispatch a subagent with no session context, only the spec and the ticket bodies. Per ticket: could it implement this from ticket plus spec alone, what is open, which AC is missing or doubled. Fix and repeat, at most 2 rounds.
7. **Show the slice** as a numbered list (title, repo, blocked by, AC ids, what it delivers). Write it scannable: one item per line, its core in a few words first; reason, source or path on an indented line below it, or left out when the user does not need it to decide; group under headings that name the topic in the reader's terms, not generic labels like Scope or Process; no item longer than two lines. Then every ticket's full body, and ask one question at a time, each as a choice: granularity right, then edges right, then merge or split anything. More than four tickets: call the Skill tool with "cj-html-report" for a page with the blocked-by graph. Iterate until the user says go. That go is the yes for filing, linking and setting fields; this skill carries it into `cj-filing-issues`.
8. **File.** Call the Skill tool with "cj-filing-issues" (File) with the batch and these presets: the body shape from step 5, each ticket a sub-issue of the spec's issue (also across repos), blocked-by edges as approved, project fields equal to the parent's, parent body untouched (the sub-issue list shows the children).

Done when every edge read back from GitHub matches the approved slice and the reply ends with the next step: `/clear`, then `/cj-implement <issue>`, run once per repo that has tickets. Write the next step under a bold "Next:" (in the reply's language) as a numbered list, one command per line, each followed by ` · ` and its reason in a few words. No table.
