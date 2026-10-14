---
name: cj-grilling
description: Use when a plan, decision, design or picked candidate has open questions that need the user's decisions before anyone builds, when the project's domain terms are fuzzy, questioned or used inconsistently, when a glossary or an ADR is written, or when the user says "grill me", "stress-test this", "frag mich aus" or "lass uns das durchdenken".
user-invocable: false
---

# Grilling

Interview the user until every decision the plan hangs on is made. Picture the plan as a **design tree**: each decision opens the decisions that depend on it.

## One question at a time

The **frontier** is every open decision whose prerequisites are already settled. Why the plan exists, what problem it solves, comes first unless it is already stated. From the frontier, ask the one decision that unblocks the most others. Ask exactly one question per turn, in the shape `cj-asking` gives: the decision and why now, the facts it rests on with their sources (a PR, issue or doc as its full URL with one line on what it did), a list the question refers to written out in full above it, then numbered options for real alternatives or yes/no for a confirm. Write it scannable: the first line is the result or the decision needed. One sentence per bullet, at most 25 words. A list has 3 to 5 items, never more than 7; a longer one splits under two headings or goes onto a page. Three or more items go one per line, never inside a sentence. No nested lists, and no tables unless this skill names one; code, log and command blocks and the `Next:` block keep their own shape. Bold only as a label at the start of a line, code font for paths and commands. Headings name the topic in the reader's terms, not generic labels like Scope or Process. No made-up abbreviations: write glossary terms out unless the glossary defines the abbreviation as the term; code identifiers, paths and issue or PR numbers stay. Before the first question to the user, call the Skill tool with `cj-asking` and ask as it says.

Then wait. Each answer settles part of the tree and may open new decisions; pick the next question from the updated frontier. Never put a second question in the same turn, not even as a follow-up line.

An answer that hands the decision back ("whatever you think is best", "you decide") settles nothing. Do not record your recommendation as the answer; ask the same decision again with two concrete options instead of three, the recommended one first, each with its consequence. Handed back a second time: record your recommendation as delegated, with its reason, and mark it so in the summary.

## Facts are your job, decisions are the user's

Anything the code, the docs or a tool can answer, look up yourself; dispatch a subagent for a longer search and keep asking the questions that do not depend on it. Put only decisions to the user. When the user's wording conflicts with the project's glossary or a recorded decision (ADR, agreement), name the conflict in the question.

## Done when

The frontier is empty and the user confirms the shared understanding. Before calling it empty, work out for yourself the next three decisions the plan hangs on and whether the user's earlier answers predict each one; a decision you cannot predict is still open, so ask it first. The ones you can predict go into the summary, marked as predicted, so the user's confirmation covers them. A third round that still finds one: stop and ask whether a premise of the plan is missing. Give a summary of the decisions before anything acts on them. Write it scannable: the first line is the result or the decision needed. One sentence per bullet, at most 25 words. A list has 3 to 5 items, never more than 7; a longer one splits under two headings or goes onto a page. Three or more items go one per line, never inside a sentence. No nested lists, and no tables unless this skill names one; code, log and command blocks and the `Next:` block keep their own shape. Bold only as a label at the start of a line, code font for paths and commands. Headings name the topic in the reader's terms, not generic labels like Scope or Process. No made-up abbreviations: write glossary terms out unless the glossary defines the abbreviation as the term; code identifiers, paths and issue or PR numbers stay. Before the first question to the user, call the Skill tool with `cj-asking` and ask as it says.

## Domain language

Call the Skill tool with "cj-project-context" for: glossary.

Keep the project's language precise while plans are made. Reading the glossary for vocabulary is a habit every skill has; this part is for **changing** the model: challenge a term, test it with a scenario, and write the glossary and decisions down the moment they settle.

### Where things live

- An existing glossary wins, whatever its name (`GLOSSARY.md`, `CONTEXT.md`, a section in the domain docs). No glossary: create `GLOSSARY.md` at the repo root when the first term is resolved. Point the project's agent docs at it: replace a pointer to a glossary file that does not exist, otherwise add one line to their list of sources.
- A `GLOSSARY-MAP.md` means several contexts; pick the one the topic belongs to, ask when unclear.
- Existing ADRs set the folder, numbering and format. None yet: `docs/adr/` with [ADR-FORMAT.md](ADR-FORMAT.md).
- The project's authoritative spec defines terms too. A term the session relies on that the spec already defines is resolved: it goes into the glossary before the first question.

### During the session

- **Challenge against the glossary.** The user's word means something else in the glossary or spec: say so with the source. "The glossary defines *Order* as what the customer submits and *Shipment* as what leaves the warehouse. A cancelled order with two shipments already sent: which one is cancelled?"
- **Sharpen fuzzy words.** Two words for one thing, or one word for two things: propose one canonical term; the others go under *Avoid*.
- **Probe with scenarios.** Invent a concrete edge case that forces the boundary between two concepts, and ask it.
- **Cross-check the code.** The user states a rule: check whether the code agrees and name `file:line` when it does not. Code identifiers that differ from the term are recorded on the term (`_Code_:`), not renamed. Code that seems to mix two terms goes under *Open* with `file:line`.
- **Write inline.** A term is resolved when the user confirms it or an authoritative document defines it. Add it to the glossary right then, in [GLOSSARY-FORMAT.md](GLOSSARY-FORMAT.md); never collect them for later. The glossary holds definitions only: no implementation notes, no plans.
- **ADRs sparingly.** Offer one only when the decision is hard to reverse, surprising without context, and the result of a real trade-off. Show the text; write it after a yes.

Term questions are ordinary questions: one at a time, the recommended term first.

### Done when

Every term the session's decisions rely on is in the glossary or listed as open, and no synonym is left unflagged.
