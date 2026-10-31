---
name: cj-reviewing
description: Use when a diff needs an independent review by a fresh reviewer, whether before a PR, inside a review-and-fix loop, for one part of a stack, or for someone else's pull request, and whenever a skill needs the shared severity scale for findings.
user-invocable: false
---

# Reviewing

One brief for every review. A fresh reviewer gets the diff, the spec and the standards, never the session history, and reports in three **sections** that stay separate: CORRECTNESS, STANDARDS, SPEC. A change can meet the spec and break the standards, or follow every standard and build the wrong thing; merging the sections hides one behind the other.

## Gather before dispatching

| Input | Where from |
|---|---|
| base and diff command | the caller (`git diff <base>...HEAD`, or the PR diff) |
| spec with acceptance criteria verbatim | issue, spec file, plan part, PR description; none found: "none" |
| deviation comments | the `Deviation:` comments on the spec's issue, verbatim; none: "none" |
| standards | Call the Skill tool with "cj-project-context" for: standards. Pass the files and sections, not a summary, plus every guideline folder the repo's instructions point to, even when they name it only for some tasks |
| checks already run | commands and results, if the caller has them |
| lint and callers source | Call the Skill tool with "cj-static-analysis": it runs the lint command over the changed files and names the best source for callers. Pass its `Lint:` lines and its source line as they came, and its worktree as the repo when it made one. A lint run with an exit code also goes under checks already run |
| ledger | previous findings and rejected ones with reasons, on a re-review |

The caller passes `low` or `high`; no argument is `low`. Ask no cost or model question, whatever the diff touches. The argument sets the model and effort of every subagent of the calling skill, reviewer and verifiers alike:

- `low`: the model and effort of the orchestrating agent.
- `high`: Opus with effort high.
- `high` is never weaker than the orchestrator. When the orchestrator is known to run the Opus model `high` dispatches, with effort high, xhigh or max by the order low < medium < high < xhigh < max, every subagent runs on that model with that effort. Known means stated in its system or harness context or by the user in this session, for the model and the effort alike: the model is known to match only when that context also names it as the newest Opus (such as the Opus it lists among the most recent models) or as the one the harness picks for Opus, so an Opus of another version, or one the context does not name so, is unknown; a guessed effort, or one assumed from a default, is unknown. Anything else, an unknown model or effort included, runs Opus with effort high.

Pass the model and the effort explicitly to every subagent where the harness takes them. An effort the orchestrator does not know is not passed; the subagent inherits it. A harness that takes only one of the two gets that one; one that takes neither dispatches on its own default, and the reply says so and names that default.

The reply holds one line `Reviewer and verifiers: <model>, effort <effort>` with the actual values the subagents got. An effort the orchestrator does not know reads `unknown`, never a guess. The calling skill copies this line into its report. Write it scannable: the first line is the result or the decision needed. Be extremely concise. Sacrifice grammar for the sake of concision. Text others read keeps full sentences. One sentence per bullet, at most 25 words. A list has 3 to 5 items, never more than 7; a longer one splits under two headings or goes onto a page. Three or more items go one per line, never inside a sentence. No nested lists, and no tables unless this skill names one; code, log and command blocks and the `Next:` block keep their own shape. Bold only as a label at the start of a line, code font for paths and commands. Headings name the topic in the reader's terms, not generic labels like Scope or Process. No made-up abbreviations: write glossary terms out unless the glossary defines the abbreviation as the term; code identifiers, paths and issue or PR numbers stay. Before the first question to the user, call the Skill tool with `cj-asking` and ask as it says. Then dispatch with the brief below.

## Severity

The one scale every skill uses:

- **high**: bug, crash, data loss, security hole, an acceptance criterion missing or built wrong, a standards break others will copy, a Loosened check without a stated reason. Blocks the merge.
- **medium**: likely failure on an edge case, missing error handling, missing test for non-trivial logic, a criterion only partly met, a documented rule broken, behaviour that touches a stated non-goal, other scope creep with real cost.
- **low**: naming, style, taste, a baseline smell without a concrete consequence.

## Reviewer brief

```
You review a change. Read and report only; change no file.

Repo: <path>
Diff: <command>
Commits: <git log <base>..HEAD>, full messages
Spec: <path, URL or pasted criteria, or "none">
Deviations: <`Deviation:` comments on the spec's issue, or "none">
Standards: <files and sections to read, plus guideline folders to pick from by the layers the diff touches, or "none documented">
Checks already run: <command -> result, or "none">
Lint: <the `Lint:` lines from cj-static-analysis>
Callers source: <the source line from cj-static-analysis, as it came>

Judge three sections, each on its own. Every defect is reported once, in the first section that owns it: a criterion missing or built wrong is SPEC; a bug the spec does not cover is CORRECTNESS; a broken rule or smell is STANDARDS. The SPEC criteria lines still list every criterion. A criterion that fails only because of a CORRECTNESS finding gets its line marked partial with that finding's id, and no SPEC finding of its own.

CORRECTNESS: bugs, edge cases, error handling, security, data loss, races, contract breaks, missing tests for non-trivial logic, performance (a query or request inside a loop over a list, unbounded fetching, a list endpoint without pagination), Loosened checks.

A Loosened check makes an existing check verify less: a test skipped or deleted while the code it tests stays, an assertion removed, a suppression comment added (`@ts-ignore`, `eslint-disable`), a threshold lowered, an error swallowed by an empty `catch`. A tightened check (a threshold raised, an assertion added) is no finding in any section. A Loosened check whose reason the spec, Deviations or a commit message gives is no finding either, only a LOOSENED line.

STANDARDS: (a) every place the diff breaks a documented rule, citing file and rule; (b) baseline smells as judgement calls, dropped where a documented rule allows the pattern: mysterious name, duplicated code, feature envy, data clump, primitive obsession, repeated switch, shotgun surgery, speculative generality, middle man, complexity moved instead of reduced, silent fallback over an unclear invariant, conditional added to an unrelated flow. Drop a finding that a Lint line already names as blocking (same file, line and rule); a violation the linter flags without blocking stays. Lint "none": judge every documented rule by reading, including those a linter could check. Skip what the build already enforces.

READ: every standards file you opened. A listed file you did not open: "not read: <file>, <why>".

WIRING: one line per entry point the diff adds or changes, meaning what the composition root is meant to register (resolver, route, handler, job, command, screen): <entry point> | <file:line where the running app composes or registers it> | wired | missing. Find that place outside the diff, starting from the app's composition root. A test that calls the entry point's own module directly does not show wiring. Missing: a high CORRECTNESS finding. Any other symbol nothing uses goes to ORPHANED, and in a repo without a composition root, so does every symbol nothing uses. With a Callers source marked unchecked, missing is a finding only when you read the composition root and found no registration there; otherwise the symbol goes to ORPHANED.

CHECKS: one line per test goal the standards require for the touched files: <goal> | ran | not run. A goal counts as ran only when "Checks already run" names it. Not run: a medium CORRECTNESS finding.

LOOSENED: one line per Loosened check in the diff: <file:line> | <what it checks less> | <reason and where it stands> | <finding id when no reason is given>.

ORPHANED: find the callers of every symbol the diff adds or changes, and of every symbol a removed line called or imported, using the Callers source. A code index has not seen the diff: take its callers, add those the diff adds, drop those it removes, and for an unchecked index fill in with `grep` at the head. An orphaned symbol is one the diff adds outside test code, or whose last caller outside tests it removes, that still exists at the head and that nothing outside tests calls or imports afterwards; one that existed at the base commit without such a caller is none. One line per orphaned symbol, none for any other symbol, "none" when there is no orphan: <symbol> | <file:line> | <source> | <finding id> or question: <what to confirm>. An entry point a WIRING line already reports missing cites that finding, under any source. Otherwise a source without "unchecked" makes it a low STANDARDS finding titled "Orphaned symbol" that names the source, and an unchecked source makes it a question, never a finding; under it, a symbol reached only through a string name, dependency injection or codegen also gets a question line. Every claim about callers in the report (none, only tests, all updated) names its source, and an unchecked one says "unchecked".

SPEC: first one line per acceptance criterion: <n> | met | partial | missing | wrong | not verifiable | <code or test that shows it>. Then findings for every criterion not met, and for behaviour the spec did not ask for, quoting the non-goal it touches if one exists. No spec: "SPEC: no spec".

Severity:
- high: bug, crash, data loss, security hole, an acceptance criterion missing or built wrong, a standards break others will copy, a Loosened check without a stated reason
- medium: likely failure on an edge case, missing error handling, missing test for non-trivial logic, a criterion only partly met, a documented rule broken, behaviour that touches a stated non-goal, other scope creep with real cost
- low: naming, style, taste, a baseline smell without a concrete consequence

Previous findings, re-check each: <id | severity | location | title | status, or "none">
The ledger adds to the review; still review the whole diff as if it were the first round.
Rejected findings, report again only with a concrete refutation: <id | title | reason, or "none">

Answer in exactly this shape:

PREVIOUS:
- <id>: resolved | still_open, <one sentence why>

LINT:
<the Lint lines as passed>

READ:
- <file>

WIRING:
<entry point lines>

CHECKS:
<goal lines>

LOOSENED:
<loosened lines>

ORPHANED:
<orphaned lines>

CORRECTNESS:
- id: F<n>
  severity: high | medium | low
  location: <file:line>
  title: <short>
  problem: <what goes wrong, with the input or state that triggers it>
  suggestion: <how to fix>

STANDARDS:
- (same fields; title starts with the rule or the smell name)

SPEC:
<criteria lines>
- (same fields; location may be "spec: criterion <n>")

Number ids on from the highest id in the ledger. An empty section is "<SECTION>: none".
Report only what you can point at in the code or the spec.
```

## After the review

Show the LINT, WIRING, CHECKS, LOOSENED and ORPHANED lines, then the three sections as they came, in that order, without reranking across them. End with one line: findings per section and the worst one in each. What happens next (fix, ask, post) is the caller's job. Remove a worktree cj-static-analysis made for the review.
