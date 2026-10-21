---
name: cj-architecture-scan
description: Scan a repo for up to five refactoring candidates, show them on a page, and file the one you pick as an issue.
disable-model-invocation: true
argument-hint: "[module, folder or pain point]"
---

# Architecture scan

For the moment the user suspects the architecture hurts but cannot name a decision yet. The scan finds candidates, the page shows them, the user picks one, and the pick becomes an issue. Everything after that, the decisions, the glossary terms, the ADRs, happens in `/cj-grill-with-docs <issue>`, so the normal flow with its tier and its challenge runs on the pick. This skill writes no code, no glossary term and no ADR, and creates or changes no file in the repo it scans.

Call the Skill tool with "cj-project-context" for: glossary, decisions. Both facts come from the repo or stay missing; this skill asks the user no project-fact question. A repo without `GLOSSARY.md`, or without an ADR folder, leaves that fact missing, and the page uses the repo's own names (files, modules, functions as the code calls them).

## 1. Scope

The argument or the user's words may name a direction: a module, a folder, a pain point ("the checkout is hard to change").

- **With a direction:** scan that area and the code that calls into it. The page header names the direction, no commit window.
- **Without:** hot spots. Count the files by the number of commits touching them in the last 200 commits:

  ```bash
  git log -200 --name-only --format= | sort | uniq -c | sort -rn
  ```

  Fewer than 200 commits: the whole history. Drop paths that no longer exist at `HEAD`; the top 10 of the rest are the scope, nothing beyond them. The page header names the window ("the last 200 commits", or "all 140 commits") and lists those files.
- **Scattered hot spots:** the window holds at least 50 commits and no file appears in more than 2 of them. Ask for a direction, as `cj-asking` says, instead of widening the scan; the question names the window and that count. Under 50 commits: no scattered check; the scope is still the top 10 of the count.

## 2. Scan

One subagent walks the scope; only its candidate list comes back to the main context. Its prompt holds the scope (the files, or the direction), the glossary terms and the ADR list when found (title, date, decision in one line), the friction questions, the deletion test, the return shape with its cap, the card text rule of step 3, and the path of the diagram patterns file beside `cj-html-report`'s `SKILL.md`. Never this conversation.

Friction to look for:

- understanding one concept means bouncing between several small files
- a module whose interface is nearly as wide as what it hides
- behaviour a test cannot reach through the current interface, or reaches only by faking the insides
- one module's details leaking into another: callers that know its storage, its call order or its error codes
- pure functions pulled out for testing while the bugs hide in how they are called

For each suspicion, the deletion test: imagine the pieces deleted or merged. The complexity reappears in one place: a candidate. It vanishes, or spreads across the callers as before: no candidate.

The subagent returns at most 5 candidates, strongest first, each with: files as `path:line-range`; the problem in one sentence; the change in structure in one sentence; the benefit as bullets of six words or fewer; before and after as a diagram, patterns in `cj-html-report`; a strength from the scale Strong, Worth exploring, Speculative. A candidate that goes against an ADR also names the ADR, a fact that changed since the ADR's date, and the consequence of the ADR that fact breaks; a candidate that cannot name all three stays out of the list.

No candidate: the reply says so and names the scanned area, the direction or the window with its files. No page, no issue, no question. Write it scannable: the first line is the result or the decision needed. Be extremely concise. Sacrifice grammar for the sake of concision. Text others read keeps full sentences. One sentence per bullet, at most 25 words. A list has 3 to 5 items, never more than 7; a longer one splits under two headings or goes onto a page. Three or more items go one per line, never inside a sentence. No nested lists, and no tables unless this skill names one; code, log and command blocks and the `Next:` block keep their own shape. Bold only as a label at the start of a line, code font for paths and commands. Headings name the topic in the reader's terms, not generic labels like Scope or Process. No made-up abbreviations: write glossary terms out unless the glossary defines the abbreviation as the term; code identifiers, paths and issue or PR numbers stay. Before the first question to the user, call the Skill tool with `cj-asking` and ask as it says.

## 3. Page

Call the Skill tool with "cj-html-report". The header carries the window and its files, or the direction. One card per candidate in the subagent's order, at most 5 (more came back: keep the strongest 5), then the closing card: card 1, or one reason for another. The badge scale is Strong, Worth exploring, Speculative; the legend names it. An ADR conflict is the card's callout with its three parts: the ADR, the fact that changed, the consequence it breaks.

Card text: the idea and the change first, in plain words and the repo's glossary terms ("the order intake keeps its pricing rules in one place"), never the class name where the glossary has a term. An architecture term (deep module, seam, adapter, locality) appears at most once per card, at its end, with `/cj-teach <term>` as the way to learn it.

The page cannot be opened, no display: print the path and go on.

## 4. Pick

One message: the chat summary `cj-html-report` asks for, the page path, and at its end one choice question as `cj-asking` says. Its options are the candidates in card order, so the numbers match the page, each with its card title, the change as its meaning line and "one issue, grilled next" as its consequence, plus a last option "none, keep the page". The recommendation line names the closing card's pick; when that is not card 1, the recommended option is not first, the one exception the skills repo's ADR 0002 records, so the numbers keep matching the page. Nothing follows the question. Write it scannable: the first line is the result or the decision needed. Be extremely concise. Sacrifice grammar for the sake of concision. Text others read keeps full sentences. One sentence per bullet, at most 25 words. A list has 3 to 5 items, never more than 7; a longer one splits under two headings or goes onto a page. Three or more items go one per line, never inside a sentence. No nested lists, and no tables unless this skill names one; code, log and command blocks and the `Next:` block keep their own shape. Bold only as a label at the start of a line, code font for paths and commands. Headings name the topic in the reader's terms, not generic labels like Scope or Process. No made-up abbreviations: write glossary terms out unless the glossary defines the abbreviation as the term; code identifiers, paths and issue or PR numbers stay. Before the first question to the user, call the Skill tool with `cj-asking` and ask as it says.

"None, keep the page": no issue. The reply names the page path for later and ends.

## 5. Issue

Call the Skill tool with "cj-filing-issues", Find first with the candidate's module and files: a duplicate (an open issue for the same change) gets no second issue; the reply names it and the `Next:` block points to it. Then File with these presets: kind task; the title names the change in structure; the body holds the problem, the change in structure, the benefit, the candidate's code files and, when the card has one, its ADR callout; acceptance criteria and non-goals read "n/a, decided in the grill", and the files stand on purpose against the task rules in `BODY.md`, because the files are the candidate and its criteria are the grill's job; labels and template only as the repo has them, a missing one is skipped, never asked; the same for every fact the filing and the posting look up (conventions, releases, issue tracker): the repo answers or the fact stays missing, and the issue language falls back to the repo's recent issues, else the user's language. Exactly one issue, and exactly one confirm: the one `cj-filing-issues` asks through `cj-posting`. This skill asks no question about the candidate itself; what the deepened module looks like, what sits behind the seam, which tests survive, all of that is the grill's job.

## Done when

The issue exists and the reply ends with the next step under a bold `Next:`, as a numbered list with one command per line, each followed by ` · ` and its reason: `/cj-grill-with-docs <issue link>` · decisions, tier and challenge for the pick. Or the scan found nothing, or the user kept the page, and the reply said so. Write it scannable: the first line is the result or the decision needed. Be extremely concise. Sacrifice grammar for the sake of concision. Text others read keeps full sentences. One sentence per bullet, at most 25 words. A list has 3 to 5 items, never more than 7; a longer one splits under two headings or goes onto a page. Three or more items go one per line, never inside a sentence. No nested lists, and no tables unless this skill names one; code, log and command blocks and the `Next:` block keep their own shape. Bold only as a label at the start of a line, code font for paths and commands. Headings name the topic in the reader's terms, not generic labels like Scope or Process. No made-up abbreviations: write glossary terms out unless the glossary defines the abbreviation as the term; code identifiers, paths and issue or PR numbers stay. Before the first question to the user, call the Skill tool with `cj-asking` and ask as it says.
