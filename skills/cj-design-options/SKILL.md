---
name: cj-design-options
description: Use when a design decision is hard to reverse or crosses a module boundary, such as where new code or data belongs and who owns it, how two parts talk (call, event, queue, shared database), adopting a library, framework, database or external service, or adding an interface, layer, plugin mechanism or config "for later"; also on "wie würde man das schneiden?", "welches Pattern passt hier?", "brauchen wir dafür X?", "architecture decision"; "wo tut die Architektur weh?", "was sollten wir refactoren?", "where does the architecture hurt?". Not for choices inside one function, and not for UI or visual design.
---

# Design options

Help the user make an architecture decision they understand. You find the forces and the candidates, explain them in the user's own code, and recommend one; the user decides. A pattern from a book is a candidate like any other, never a reason on its own.

Called inside another flow (grilling, a spec, an implementation): run the steps, then hand the decision back to that flow's decision list and continue there.

## 1. Frame the decision

1. Write the decision as one open question that names the need, not a solution: "where does sending the order mail live?", not "should we add a queue?".
   No decision to frame, the user asks where the architecture hurts or what to refactor: name `/cj-architecture-scan` (with the area, when they named one) as the next step and stop; it finds the candidates and files the pick as an issue.
2. Read the code it touches. Call the Skill tool with "cj-project-context" for: glossary, decisions. A recorded decision (ADR, agreement) that already settles it: name it with its link and stop, unless the user wants to reopen it.
3. Decide the reversibility yourself and say it in one line. **Hard to reverse**: a data model or persisted format, a contract other teams or apps call, technology with lock-in, a split into separately deployed parts. Everything else is **cheap**: steps 3 and 5 run short, no subagents, no page.

## 2. Forces

A force is a fact about this codebase that pushes the decision one way. Find them from the code, `git log` on the touched files, the issue and the docs; each one carries its source.

- What changes often, and who changes it.
- Who calls it, and how many callers there are.
- What a failure costs, and who notices it.
- Load and timing, as numbers from a source, else "unknown".
- Who owns the parts, and how many people change them.

A force you cannot back with a source is marked "assumed". A force only the user knows (a plan, a deadline, a team change) is a question, asked through `cj-grilling` before step 3. Generic quality attributes ("scalability", "maintainability") are not forces until a fact from this list stands behind them.

## 3. Candidates

Two or three, always including the simplest thing that holds: often no new structure, a change in place.

- **Cheap:** you write them.
- **Hard to reverse:** dispatch three subagents in parallel. Each gets the decision, the forces with sources and the file paths, not this conversation, and one constraint: (1) the smallest change that holds; (2) make the change the forces predict most often a change in one place; (3) put a seam where the forces say something varies, with its adapters. Each returns the shape in a few lines, the forces it serves, what it costs and what it hides. Keep the two or three most different; drop near-duplicates.

## 4. Check every candidate

- **Deletion test.** Imagine deleting each new module, layer or interface. Complexity vanishes: it is a pass-through, cut it. Complexity reappears across callers: it earns its place.
- **One adapter is a hypothetical seam, two make a real one.** An interface earns its place with a second adapter that exists now: another implementation, or a fake the tests use. "We might switch providers" is not a second adapter.
- **Dependency category** for each dependency behind the decision. It says what a seam would take and how tests reach the dependency; whether a seam is due is still the one-adapter rule:
  - in-process (pure logic, memory): merge and test directly, never a seam;
  - local stand-in exists (an in-memory database, a temp filesystem): test with the stand-in, never a seam;
  - own remote service: a seam takes a port with a real adapter and an in-memory one for tests;
  - third-party service: a seam takes a port and a fake; without one, tests use the vendor's test mode.
- **Named pattern.** It names the force from step 2 it solves here and what it costs (files, indirection, operations). It solves none: drop it and say so in one line.

## 5. Explain and recommend

Per candidate, plain words first:

1. The idea in one or two sentences, without the pattern name.
2. A change scenario from this code: "next, <change the forces predict> comes; in this candidate you touch <files>."
3. The cost: new files, new concepts, what gets harder.
4. The pattern name last, if there is one, with one line on what it is called elsewhere.

Then the recommendation: the candidate, the force it rests on, and the flip condition under which another candidate wins ("if a second team starts changing billing, B wins"). A hard-to-reverse decision with three candidates, or one where a diagram says it faster (dependencies, before and after), calls the Skill tool with "cj-html-report": one card per candidate, the closing card is the recommendation.

The user asks what a concept means, or the same concept comes up a second time: explain it in two sentences with this code, and name `/cj-teach <concept>` as a next step for learning it properly.

Write it scannable: the first line is the result or the decision needed. Be extremely concise. Sacrifice grammar for the sake of concision. Text others read keeps full sentences. One sentence per bullet, at most 25 words. A list has 3 to 5 items, never more than 7; a longer one splits under two headings or goes onto a page. Three or more items go one per line, never inside a sentence. No nested lists, and no tables unless this skill names one; code, log and command blocks and the `Next:` block keep their own shape. Bold only as a label at the start of a line, code font for paths and commands. Headings name the topic in the reader's terms, not generic labels like Scope or Process. No made-up abbreviations: write glossary terms out unless the glossary defines the abbreviation as the term; code identifiers, paths and issue or PR numbers stay. Before the first question to the user, call the Skill tool with `cj-asking` and ask as it says.

## 6. The user decides

Call the Skill tool with "cj-grilling" and put the decision to the user as one question: the candidates as options, the recommended one first, each with its consequence. Decide nothing yourself, a cheap decision included. Follow-up decisions the answer opens (where the seam goes, what the new part is called) go through the same grilling, one per turn.

After the decision, offer an ADR only when it passes the three-part test in `cj-grilling`. A rejected candidate counts when a later session would otherwise suggest it again.

## Done when

The user picked a candidate or chose none, and the reply sums up the decision, the force it rests on, the flip condition, and the ADR written or the reason none was. Write it scannable: the first line is the result or the decision needed. Be extremely concise. Sacrifice grammar for the sake of concision. Text others read keeps full sentences. One sentence per bullet, at most 25 words. A list has 3 to 5 items, never more than 7; a longer one splits under two headings or goes onto a page. Three or more items go one per line, never inside a sentence. No nested lists, and no tables unless this skill names one; code, log and command blocks and the `Next:` block keep their own shape. Bold only as a label at the start of a line, code font for paths and commands. Headings name the topic in the reader's terms, not generic labels like Scope or Process. No made-up abbreviations: write glossary terms out unless the glossary defines the abbreviation as the term; code identifiers, paths and issue or PR numbers stay. Before the first question to the user, call the Skill tool with `cj-asking` and ask as it says.
