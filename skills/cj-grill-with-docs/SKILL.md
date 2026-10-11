---
name: cj-grill-with-docs
description: Grill an issue or idea one question at a time until every decision is made, writing glossary terms and ADRs as they settle. Step 1 of the spec-driven flow.
disable-model-invocation: true
---

# Grill with docs

Step 1 of the flow `cj-challenge` → `cj-grill-with-docs` → `cj-to-spec` → `cj-to-tickets` → `cj-implement` → `cj-retro`. It ends in a shared understanding, not in code or a spec.

1. **Input.** An issue reference: read the body, every comment, linked issues and the affected code. An idea without an issue: start from the user's words.
2. **Challenged?** A `## Challenge` comment in the thread or a challenge result earlier in this session: its verdict and smaller lever are settled decisions; anything but "build" or "build differently" stops here. None: the challenge is optional. For a new feature or real effort, say in one line that `/cj-challenge <issue>` is recommended, then go on and put "Not challenged" in the decision list of step 5. A trivial fix or a refactor without behaviour change skips the line.
3. **Grill.** Call the Skill tool with "cj-grilling". Facts come from code, docs and tools; only decisions go to the user.
4. **Questions only the issue author can answer** (product intent, a business rule nobody in the session owns): collect them into one comment on the issue, numbered, each with a proposed answer. Call the Skill tool with "cj-posting" before posting it. Stop there. When the answers arrive (even days later), the user starts `/cj-grill-with-docs <issue>` again: it reads the answers from the thread as settled decisions and grills only what is still open.
5. **Close.** List every decision, plus the glossary terms and ADRs written during the session (left uncommitted; `cj-to-spec` lists them, `cj-implement` commits them on the feature branch). Write it scannable: one item per line, its core in a few words first; reason, source or path on an indented line below it, or left out when the user does not need it to decide; group under headings that name the topic in the reader's terms, not generic labels like Scope or Process; no item longer than two lines. Show the list in the reply text, never as a prose paragraph and never only inside the question, decisions numbered so the user can name one. Ask the user to confirm the list, as a choice: confirm, or change decision <n>.

Done when the frontier is empty, the user confirmed the decision list, and the reply ends with the next step: `/cj-to-spec <issue>` now, in this session, without `/clear` first, because spec and tickets are cut from this conversation. Write the next step under a bold "Next:" (in the reply's language) as a numbered list, one command per line, each followed by ` · ` and its reason in a few words. No table.
