---
name: cj-to-spec
description: Turn the grilled conversation into a spec, post it as a "## Spec" comment on the issue (or as a new issue), and slice a spec that needs more than one ticket into sub-issues. Step 2 of the spec-driven flow.
disable-model-invocation: true
---

# To spec

Step 2 of the spec-driven flow, in the normal and big tiers. The spec is the one source the tickets, the implementation, the review and the PR point to. It says **what** and **why**, never **how** in file terms. A spec that needs more than one ticket is sliced into sub-issues in the same run, with [TICKETS.md](TICKETS.md).

Call the Skill tool with "cj-project-context" for: repos, glossary, issue-tracker, conventions.

## Steps

**Spec already posted.** The issue has a `## Spec` comment, or a body starting with `## Spec`. The prompt names a change to it, or a grill ran in this session after it: "Changing a posted spec" applies first; an edit that changes the ticket count applies step 4's count and its `Not challenged` rule to the edited comment. Either way, draft no new spec and skip steps 1 to 6. A spec that needs more than one ticket (step 4) and whose sub-issues do not deliver every AC goes to step 7 for the ACs none delivers; any other spec ends with the one-ticket next step under "Done when".

1. **Synthesize, do not interview.** Work from the conversation and the code. Use the glossary's terms and respect the ADRs in the area. If this session did not grill (no decision list in the conversation), call the Skill tool with "cj-grilling" first; a spec built on guesses is the failure this flow exists to prevent.
2. **Test seams.** Propose where the behaviour is tested: the highest public interface that observes it, existing seams before new ones, as few as possible. Per seam: what it catches, what it misses. Write it scannable: the first line is the result or the decision needed. One sentence per bullet, at most 25 words. A list has 3 to 5 items, never more than 7; a longer one splits under two headings or goes onto a page. Three or more items go one per line, never inside a sentence. No nested lists, and no tables unless this skill names one; code, log and command blocks and the `Next:` block keep their own shape. Bold only as a label at the start of a line, code font for paths and commands. Headings name the topic in the reader's terms, not generic labels like Scope or Process. No made-up abbreviations: write glossary terms out unless the glossary defines the abbreviation as the term; code identifiers, paths and issue or PR numbers stay. Before the first question to the user, call the Skill tool with `cj-asking` and ask as it says. Show the seams in the reply text, never as a prose paragraph and never only inside the question, numbered so the user can name one, in this shape (labels in the reply's language):

   ```
   **Test seams**

   **1. <seam in a few words>** (<where: command, file or interface>)
   - catches: <one failure>
   - catches: <the next failure, one per line>
   - misses: <one blind spot, one per line>
   ```

   Then, before the draft, ask the user to confirm them as a yes/no choice question, the whole list directly above it: yes confirms every seam, no lets the user name the seam to change or drop.
3. **Size.** One spec is one goal that ships on its own. Two goals that could ship separately: propose two specs, one per issue, and continue with the one the user picks first.
4. **Draft** the spec in the shape of [SPEC.md](SPEC.md), with the Acceptance line its triggers propose, and run its checks. Judge the ticket count: more than one ticket when the work needs more than one fresh context window, or touches more than one repo. More than one ticket and no challenge verdict (no `## Challenge` comment on the issue, none in this conversation, none on a docs page the issue or conversation links): the Decisions hold the line `Not challenged: more than one ticket, no challenge ran`. A one-ticket spec never holds it; a posted line stays when a revised slice later fits one ticket, since no challenge ran.
5. **Fresh reader.** Dispatch one subagent on the Opus tier with effort high where the harness takes a model and an effort, as `cj-reviewing` does for its reviewer. Its prompt holds this task, the draft and the issue if one exists; no conversation, no decision list. It reports every AC or edge-case row that a wrong implementation would still pass, naming that implementation. A finding that sharpens an AC without a new decision goes into the draft. For a finding that changes behaviour or scope, call the Skill tool with "cj-grilling" and ask it as one question that quotes the AC or row; it goes into the draft only with the user's answer. After the reader's findings or the user's answers change the draft, run the checks of step 4 again. No subagent available: say so above the draft.
6. **Show and post.** Above the draft, one line states whether the spec needs one ticket or more, with its reason (fresh context windows, repos). The user's answer to the spec can override the count; the `Not challenged` line follows the count that is posted. Show the full text, and below it one line per reader finding, `<finding> → worked in` or `<finding> → rejected: <reason>`. These lines are not posted. Call the Skill tool with "cj-posting":
   - The issue exists: post the spec as a **comment** on it. Never edit the issue body; it belongs to its author. Leave its title unchanged, including any existing prefix.
   - No issue yet: call the Skill tool with "cj-filing-issues" (Find, then File) and create one whose body is the spec. Use the approved title without automatically adding `[Epic] `.
   - So `cj-filing-issues` runs here only without an issue. Sub-issues are filed in step 7 with [TICKETS.md](TICKETS.md), and only when the spec needs more than one ticket. A one-ticket spec goes from its comment straight to `cj-implement`, with no new issue.
   - Publishing a spec does not propose or authorize Epic marking. The slice in [TICKETS.md](TICKETS.md) proposes the Epic title when it has multiple sub-issues.
   - Apply the ready label from `issue-tracker`, if the repo has one.
7. **Slice**, only when the posted spec needs more than one ticket: follow [TICKETS.md](TICKETS.md). The slice and the Epic title preview are a question of their own after the spec is posted; the yes for the spec does not cover them, and nothing is filed before their yes. The user rejects the slice as a whole, not one ticket or edge of it: ask whether to revise it (recommended) or build everything in one PR. Revise runs the slicing again from its step 4; one PR files nothing, proposes no title change, and ends with the one-PR next step. Write it scannable: the first line is the result or the decision needed. One sentence per bullet, at most 25 words. A list has 3 to 5 items, never more than 7; a longer one splits under two headings or goes onto a page. Three or more items go one per line, never inside a sentence. No nested lists, and no tables unless this skill names one; code, log and command blocks and the `Next:` block keep their own shape. Bold only as a label at the start of a line, code font for paths and commands. Headings name the topic in the reader's terms, not generic labels like Scope or Process. No made-up abbreviations: write glossary terms out unless the glossary defines the abbreviation as the term; code identifiers, paths and issue or PR numbers stay. Before the first question to the user, call the Skill tool with `cj-asking` and ask as it says.

Done when the spec is posted, its link is in the reply, and the reply ends with the next step. One ticket: `/clear`, then `/cj-implement <issue>` (`cj-implement` reads the issue and the spec comment itself and needs a fresh context window). More than one ticket: the next step [TICKETS.md](TICKETS.md) names once the slice is filed. Slice rejected and one PR chosen: `/clear`, then `/cj-implement <issue> one-pr`; never `/cj-implement <issue>` without `one-pr` for that spec. Write the next step under a bold "Next:" (in the reply's language) as a numbered list, one command per line, each followed by ` · ` and its reason in a few words. A step that names a PR, issue or comment carries its full link, even when the link appears earlier in the reply. No table.

## Changing a posted spec

The approved sections (problem, acceptance criteria, out of scope, the Acceptance line) change only with the user's yes; then edit the spec comment and add a one-line comment below it saying what changed. Technical deviations during implementation never edit the spec; they are separate comments (see `cj-implement`).
