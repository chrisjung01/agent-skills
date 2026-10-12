---
name: cj-to-spec
description: Turn the grilled conversation into a spec and post it as a "## Spec" comment on the issue (or as a new issue). Step 2 of the spec-driven flow.
disable-model-invocation: true
---

# To spec

Step 2 of the flow `cj-challenge` → `cj-grill-with-docs` → `cj-to-spec` → `cj-to-tickets` → `cj-implement` → `cj-retro`. The spec is the one source the tickets, the implementation, the review and the PR point to. It says **what** and **why**, never **how** in file terms.

Call the Skill tool with "cj-project-context" for: repos, glossary, issue-tracker, conventions.

## Steps

1. **Synthesize, do not interview.** Work from the conversation and the code. Use the glossary's terms and respect the ADRs in the area. If this session did not grill (no decision list in the conversation), call the Skill tool with "cj-grilling" first; a spec built on guesses is the failure this flow exists to prevent.
2. **Test seams.** Propose where the behaviour is tested: the highest public interface that observes it, existing seams before new ones, as few as possible. Per seam: what it catches, what it misses. Write it scannable: one item per line, its core in a few words first; reason, source or path on an indented line below it, or left out when the user does not need it to decide; group under headings that name the topic in the reader's terms, not generic labels like Scope or Process; no item longer than two lines. Show the seams in the reply text, never as a prose paragraph and never only inside the question, numbered so the user can name one, in this shape (labels in the reply's language):

   ```
   **Test seams**
   1. <seam in a few words> · <where: command, file or interface>
      catches: <what breaks it>
      misses: <what it does not see>
   ```

   Then ask the user to confirm them as one question with a choice (confirm, change seam <n>), before the draft.
3. **Size.** One spec is one goal that ships on its own. Two goals that could ship separately: propose two specs, one per issue, and continue with the one the user picks first.
4. **Draft** the spec in the shape of [SPEC.md](SPEC.md) and run its checks.
5. **Show and post.** Show the full text. Call the Skill tool with "cj-posting":
   - The issue exists: post the spec as a **comment** on it. Never edit the issue body; it belongs to its author. Leave its title unchanged, including any existing prefix.
   - No issue yet: call the Skill tool with "cj-filing-issues" (Find, then File) and create one whose body is the spec. Use the approved title without automatically adding `[Epic] `.
   - So `cj-filing-issues` runs here only without an issue. Sub-issues are filed later by `cj-to-tickets`, and only when the spec needs more than one ticket. A one-ticket spec goes from its comment straight to `cj-implement`, with no new issue.
   - Publishing a spec does not propose or authorize Epic marking. `cj-to-tickets` proposes it when the split has multiple sub-issues.
   - Apply the ready label from `issue-tracker`, if the repo has one.

Done when the spec is posted, its link is in the reply, and the reply ends with the next step: `/cj-to-tickets <issue>` now, in this session, without `/clear` first, when the spec needs more than one ticket (more than one fresh context window of work, or work in more than one repo); else `/clear`, then `/cj-implement <issue>` (`cj-implement` reads the issue and the spec comment itself and needs a fresh context window). Write the next step under a bold "Next:" (in the reply's language) as a numbered list, one command per line, each followed by ` · ` and its reason in a few words. A step that names a PR, issue or comment carries its full link, even when the link appears earlier in the reply. No table.

## Changing a posted spec

The approved sections (problem, acceptance criteria, out of scope) change only with the user's yes; then edit the spec comment and add a one-line comment below it saying what changed. Technical deviations during implementation never edit the spec; they are separate comments (see `cj-implement`).
