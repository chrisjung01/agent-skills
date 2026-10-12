# Spec shape

The comment starts with the line `## Spec` so later steps find it. Headings in the issue's language (`conventions`); the content rules below hold in any language.

```
## Spec

**Problem.** What is wrong or missing today, from the user's point of view. One to three sentences, no solution.

**Solution.** What changes in behaviour, from the user's point of view.

**Acceptance criteria**
- AC-1: Given <state>, when <action>, then <observable result>.
- AC-2: ...

**Edge cases** (only rows that are real for this feature)
| Input or state | Expected |
|---|---|

**Decisions.** One line each with its reason: modules and interfaces that change, data or schema changes, contracts, permissions. Module and entity names, no file paths, no code; a snippet only when a prototype pinned a decision more precisely than prose can.

**Test seams.** Per seam: what it observes, what it catches, what it misses.

**Out of scope.** What this deliberately leaves out, at least one item when the scope has an obvious neighbour.

**Recorded in repo.** Glossary terms and ADRs written during the grilling, by name, for `cj-implement` to commit. "none" when empty.

**Merge danger.** One-way or two-way door, and the blast radius in one line.
```

## Checks before showing

- [ ] Every AC is observable by someone who did not write the code, answerable with yes or no. Never "works", "is tested", "handles errors".
- [ ] An AC about what a screen shows names the screen sizes it must hold at, one per layout the app's own breakpoints define (terminal columns, viewport widths, device classes), and its seam renders at those sizes through the app's real layout, not in a test frame of its own choosing.
- [ ] Every AC and every edge-case row can be covered by a test at a listed seam, or says how it is checked by hand.
- [ ] A number in an AC (baseline, threshold, count) was computed with the spec's own definition, and the spec names the command or query that produced it.
- [ ] A quantity, formula or growth rule in a decision or AC was checked against the limits the repo already states (design docs, decision lists, ADRs, load or performance tests), and the spec names the limit it keeps. A conflict goes back to grilling.
- [ ] A decision that relies on how a tool, platform or check behaves (when a file is written, what a count catches) was tried once against the case it must handle, including the hard case (a leftover with a fixed name, a delayed write). Not tried: back to grilling, like any open question.
- [ ] No open question, no "TBD". Something still open goes back to grilling; it is not published.
- [ ] The spec is shorter than the code it describes would be. Over a screen and a half for a small feature means it holds implementation.
- [ ] Every term is the glossary's.
- [ ] Run the `unslop` skill over it.
