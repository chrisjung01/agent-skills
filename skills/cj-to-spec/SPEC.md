# Spec shape

The comment, or the body of an issue `cj-to-spec` creates, starts with the line `## Spec` so later steps find it. Teammates who open the issue see only the note and the summary; the full spec sits in a collapsed block that anyone can expand. Headings, the note, the summary and the `<summary>` label in the issue's language (`conventions`); the content rules below hold in any language.

```
## Spec

_Input for the agent that implements this issue. Nothing to review here; expand the block below for the full spec._

**Summary.** The problem and the goal in at most two sentences, for a teammate who reads nothing else.

<details>
<summary>Full spec</summary>

**Problem.** What is wrong or missing today, from the user's point of view. One to three sentences, no solution.

**Solution.** What changes in behaviour, from the user's point of view.

**Acceptance criteria**
- AC-1: Given <state>, when <action>, then <observable result>.
- AC-2: ...

**Edge cases** (only rows that are real for this feature)
| Input or state | Expected |
|---|---|

**Non-functional requirements.** Measurable requirements such as speed, limits, security or accessibility, each with its measured value (within 2 s, up to 500 notes); `none` when empty. Every requirement that can be checked gets an AC.

**Decisions.** One line each with its reason, plus the remaining risk when one is left and, for a decision tried against the hard case, `tried: <procedure> → <observed result>`: modules and interfaces that change, data or schema changes, contracts, permissions. How the change reaches the default branch (branch, pull request, direct commit) is `cj-implement`'s, not a decision here. Module and entity names, no file paths, no code; a snippet only when a prototype pinned a decision more precisely than prose can.

**Test seams.** Per seam: what it observes, what it catches, what it misses.

**Out of scope.** What this deliberately leaves out, at least one item when the scope has an obvious neighbour. The coverage table's Out of scope rows go here.

**Recorded in repo.** Glossary terms and ADRs written during the grilling, by name, for `cj-implement` to commit. "none" when empty.

**Merge danger.** One-way or two-way door, the blast radius in one line, and whether old clients and old code still running during the rollout stay valid.

Acceptance: <by hand | verified is enough> · <trigger | no trigger applies>

</details>
```

GitHub renders Markdown inside `<details>` only with a blank line after `</summary>` and one before `</details>`; without them headings and tables show as raw text.

## Acceptance line

The last line before `</details>` says whether the user checks the spec by hand before the last merge (`cj-accept`), or whether the Verified lines of the PRs are enough. Plain text, English words in any issue language, so later steps find it.

- By hand when a trigger applies: behaviour a user sees, money, data, security, an irreversible step, or someone else signs the feature off. Name the first that applies, in these words.
- No trigger applies: `verified is enough · no trigger applies`. Effort, ticket count and tier are no triggers.
- The user picks the other value: keep the trigger and append `· user chose`, e.g. `Acceptance: verified is enough · behaviour a user sees · user chose`.

## Checks before showing

- [ ] Every AC is observable by someone who did not write the code, answerable with yes or no. Never "works", "is tested", "handles errors".
- [ ] An AC about what a screen shows names the screen sizes it must hold at, one per layout the app's own breakpoints define (terminal columns, viewport widths, device classes), and its seam renders at those sizes through the app's real layout, not in a test frame of its own choosing.
- [ ] Every AC and every edge-case row can be covered by a test at a listed seam, or says how it is checked by hand.
- [ ] A rule the change applies in many places (every skill that asks, every copy of a block, every step that confirms) lists those places, found by what they do and not only by a word search, and an AC checks each place for the new behaviour. Each place is read for what it already fixes (a last line, a block order, a required label); an AC that contradicts one names the place and how the two fit. An AC that only greps for old words passes a place that does the same in other words.
- [ ] A change to a building block (a skill, function or module other parts call) lists its callers, found with `cj-static-analysis` or a search for what calls it, and a decision says for each whether the new behaviour reaches it. A caller the grilling did not decide goes back to grilling as a question, asked before the test seams; the spec never assumes it.
- [ ] A number in an AC (baseline, threshold, count) was computed with the spec's own definition, and the spec names the command or query that produced it.
- [ ] An AC about a test or scenario asks for the test and its recorded results, never for a result nobody has seen: "red on the commit before" is a prediction until that run happened, so the AC says both outcomes are recorded with their commits and leaves which one falls to the run.
- [ ] An AC that applies two steps to one value (check and trim, cap and roll, round and compare) names their order. A cap or limit an AC sets is reached by some input a test tells apart; one no input reaches is dead and leaves the spec.
- [ ] The ACs hold together: a limit one AC sets (a line cap, a count, a size) still fits what the other ACs demand, worked out with the spec's own numbers.
- [ ] A quantity, formula or growth rule in a decision or AC was checked against the limits the repo already states (design docs, decision lists, ADRs, load or performance tests), and the spec names the limit it keeps. A conflict goes back to grilling.
- [ ] A value the user feels (reward, price, duration, amount) has at least one absolute anchor the user stated, such as "the first quest pays about 30 silver"; ratios and growth rules alone leave the size to the implementer. No anchor: back to grilling.
- [ ] A decision that relies on how a tool, platform or check behaves (when a file is written, what a count catches) was tried once against the case it must handle, including the hard case (a leftover with a fixed name, a delayed write), read-only or on a scratch copy. A try that would change shared state (repo settings, labels, a remote) is not run; it is an open question. Not tried, failed, inconclusive or unavailable: back to grilling, like any open question. The observed result goes into the decision's `tried:` part; a planned try is not observed evidence.
- [ ] Only `## Spec`, the note and the summary sit above `<details>`, and the blank lines around the block's content are there.
- [ ] The last line before `</details>` is the Acceptance line with its value and its trigger, or `no trigger applies`; a `Sources` block from `cj-posting` step 2 stands right before it; by hand comes from a trigger or the user's choice, never from effort, ticket count or tier.
- [ ] No open question, no "TBD". Something still open goes back to grilling; it is not published.
- [ ] The spec is shorter than the code it describes would be. Over a screen and a half for a small feature means it holds implementation.
- [ ] Every term is the glossary's.
- [ ] Run the `unslop` skill over it as `cj-posting` step 2 says.
