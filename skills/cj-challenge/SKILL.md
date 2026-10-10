---
name: cj-challenge
description: Challenge an issue or idea like a sceptical project lead before anyone grills it - goals, evidence, smaller levers - and block what does not hold up. Step 0 of the spec-driven flow.
disable-model-invocation: true
---

# Challenge

Step 0 of the flow `cj-challenge` → `cj-grill-with-docs` → `cj-to-spec` → `cj-to-tickets` → `cj-implement` → `cj-retro`. It decides **whether** to build, and what the smallest thing is that solves the problem. How to build it is `cj-grill-with-docs`' job.

You are the user's critical self: the project lead who has seen a hundred features nobody used. The burden of proof is on the idea. Not hostile, not contrarian: every objection names its source or its concrete scenario, never a generic risk. A good answer moves the idea forward; no answer blocks it.

Call the Skill tool with "cj-project-context" for: product-docs, teams, analytics.

## Rules

- **Documented or it did not happen.** Only what is written down counts: the issue and its thread, linked issues and PRs, the docs tool (`product-docs`), analytics. A claim the user makes in chat counts once it is written into the issue or a docs page; the challenge result (step 8) is that record for answers given in this session. A goal link the user cannot point to in `product-docs` counts as "none".
- **One question per turn**, as a choice with the recommended answer first. Typical options: a source that settles it, "no source: counts as a gap", and when it fits, the smaller alternative. Facts you can look up yourself never go to the user.
- **A gap is a block.** Any gate in step 7 still open after its question means the verdict cannot be "build". The block is not final: it reopens when the gap is documented.
- **A decided item gets a short check.** A product owner or stakeholder already decided it with a documented goal link: run steps 1, 3 and 5 only.

## Steps

1. **Input.** An issue: read the body, every comment, linked issues and the affected code. An idea without an issue: the user's words. Separate the **problem** (who has which need, how often, what it costs today) from the **proposed solution** (a proposal, not a requirement). Note who wrote it and from which role.
2. **Documented decisions.** Search `product-docs` for decisions on the topic. A contradiction with a documented decision goes to the user first.
3. **Goal link.** The owning team (`teams`) and its goals for the current period (`product-docs`): name the objective or key result this moves, and how much. None visible: say so.
4. **Data.** Frequency and impact from `analytics` and the tracker: event counts, funnels, usage of a similar feature, how many reports. Only numbers read from a source; an unreachable source is "not available".
5. **The smaller lever.** Find the least invasive change that solves the core problem. Climb and stop at the first rung that holds:
   1. Do nothing: what does it cost if this is never built?
   2. Already there: an existing feature, setting or combination covers it; the user just does not find it.
   3. Remove or change instead of add: a default, a copy text, a step taken out, a limit raised.
   4. Non-code: process, documentation, a manual workaround for the few cases.
   5. One small change in a shared place that fixes this and its siblings.
   6. Validate first: a fake-door test, a prototype, a manual run for a week.

   Then dispatch one fresh subagent with only the issue link and the sources from steps 2 to 4, not this conversation, and ask for the strongest case against the proposal and the smallest change that would solve the core problem. A second view without this session's momentum catches what one pass confirms. Merge what holds up.
6. **Pre-mortem.** It is six months after launch and this failed. Write the two most likely reasons, each tied to something found in steps 1 to 5. For each, ask whether the plan prevents it.
7. **Gates.** Question the user, one gap at a time, the gap that blocks the most first:
   - The core problem is stated without the solution and has someone who has it.
   - A documented goal link, or a documented reason it matters anyway (legal, security, a contract).
   - Frequency or impact from a source, or an explicit "unknown" with a validate-first step.
   - No rung of step 5 solves the core problem as well for less. If one does, the verdict is "build differently" with that rung.
   - No contradiction with a documented decision, or the user takes it to whoever owns that decision.
   - An owner after launch.
   - Each pre-mortem reason has an answer.
8. **Verdict**: build / build differently / validate first / don't build, following from the gates. "Build" and "build differently" carry one **kill criterion**: a measurable signal and a date at which it gets rolled back or stopped. Write it in [REPORT.md](REPORT.md) format. It goes on the issue as a comment; an idea without an issue asks the user, as a choice: create the issue (call the Skill tool with "cj-filing-issues"), or a docs page in `product-docs`. Call the Skill tool with "cj-posting" before posting.

Done when the verdict is posted where the user chose, and the reply sums it up in chat (verdict, open gaps, kill criterion). Write it scannable: one item per line, its core in a few words first; reason, source or path on an indented line below it, or left out when the user does not need it to decide; group under headings that name the topic in the reader's terms, not generic labels like Scope or Process; no item longer than two lines. The reply ends with the next step under a bold "Next:" (in the reply's language) as a numbered list, one command per line, each followed by ` · ` and its reason in a few words. No table. "Build" or "build differently": `/cj-grill-with-docs <issue>` now, in this session, without `/clear` first, because the grilling starts from this conversation. "Validate first": the validation step and who runs it. "Don't build": nothing, or closing the issue if the user agrees.
