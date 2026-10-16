# Spec shape

The comment, or the body of an issue `cj-to-spec` creates, starts with the line `## Spec` so later steps find it. Headings in the issue's language (`conventions`); the content rules below hold in any language.

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

**Decisions.** One line each with its reason: modules and interfaces that change, data or schema changes, contracts, permissions. How the change reaches the default branch (branch, pull request, direct commit) is `cj-implement`'s, not a decision here. Module and entity names, no file paths, no code; a snippet only when a prototype pinned a decision more precisely than prose can.

**Test seams.** Per seam: what it observes, what it catches, what it misses.

**Out of scope.** What this deliberately leaves out, at least one item when the scope has an obvious neighbour.

**Recorded in repo.** Glossary terms and ADRs written during the grilling, by name, for `cj-implement` to commit. "none" when empty.

**Merge danger.** One-way or two-way door, the blast radius in one line, and whether old clients and old code still running during the rollout stay valid.

Acceptance: <by hand | verified is enough> · <trigger | no trigger applies>
```

## Acceptance line

The last line says whether the user checks the spec by hand before the last merge (`cj-accept`), or whether the Verified lines of the PRs are enough. Plain text, English words in any issue language, so later steps find it.

- By hand when a trigger applies: behaviour a user sees, money, data, security, an irreversible step, or someone else signs the feature off. Name the first that applies, in these words.
- No trigger applies: `verified is enough · no trigger applies`. Effort, ticket count and tier are no triggers.
- The user picks the other value: keep the trigger and append `· user chose`, e.g. `Acceptance: verified is enough · behaviour a user sees · user chose`.

## Checks before showing

- [ ] Every AC is observable by someone who did not write the code, answerable with yes or no. Never "works", "is tested", "handles errors".
- [ ] An AC about what a screen shows names the screen sizes it must hold at, one per layout the app's own breakpoints define (terminal columns, viewport widths, device classes), and its seam renders at those sizes through the app's real layout, not in a test frame of its own choosing.
- [ ] Every AC and every edge-case row can be covered by a test at a listed seam, or says how it is checked by hand.
- [ ] A rule the change applies in many places (every skill that asks, every copy of a block, every step that confirms) lists those places, found by what they do and not only by a word search, and an AC checks each place for the new behaviour. An AC that only greps for old words passes a place that does the same in other words.
- [ ] A number in an AC (baseline, threshold, count) was computed with the spec's own definition, and the spec names the command or query that produced it.
- [ ] The ACs hold together: a limit one AC sets (a line cap, a count, a size) still fits what the other ACs demand, worked out with the spec's own numbers.
- [ ] A quantity, formula or growth rule in a decision or AC was checked against the limits the repo already states (design docs, decision lists, ADRs, load or performance tests), and the spec names the limit it keeps. A conflict goes back to grilling.
- [ ] A decision that relies on how a tool, platform or check behaves (when a file is written, what a count catches) was tried once against the case it must handle, including the hard case (a leftover with a fixed name, a delayed write), read-only or on a scratch copy. A try that would change shared state (repo settings, labels, a remote) is not run; it is an open question. Not tried: back to grilling, like any open question.
- [ ] The last line is the Acceptance line with its value and its trigger, or `no trigger applies`; by hand comes from a trigger or the user's choice, never from effort, ticket count or tier.
- [ ] No open question, no "TBD". Something still open goes back to grilling; it is not published.
- [ ] The spec is shorter than the code it describes would be. Over a screen and a half for a small feature means it holds implementation.
- [ ] Every term is the glossary's.
- [ ] Run the `unslop` skill over it.
