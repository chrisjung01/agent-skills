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
| ledger | previous findings and rejected ones with reasons, on a re-review |

Dispatch one subagent with the brief below, on the Opus tier with effort high where the harness takes a model and an effort. A smaller model misses findings that only show outside the diff.

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

Judge three sections, each on its own. Every defect is reported once, in the first section that owns it: a criterion missing or built wrong is SPEC; a bug the spec does not cover is CORRECTNESS; a broken rule or smell is STANDARDS. The SPEC criteria lines still list every criterion. A criterion that fails only because of a CORRECTNESS finding gets its line marked partial with that finding's id, and no SPEC finding of its own.

CORRECTNESS: bugs, edge cases, error handling, security, data loss, races, contract breaks, missing tests for non-trivial logic, performance (a query or request inside a loop over a list, unbounded fetching, a list endpoint without pagination), Loosened checks.

A Loosened check makes an existing check verify less: a test skipped or deleted while the code it tests stays, an assertion removed, a suppression comment added (`@ts-ignore`, `eslint-disable`), a threshold lowered, an error swallowed by an empty `catch`. A tightened check (a threshold raised, an assertion added) is no finding in any section. A Loosened check whose reason the spec, Deviations or a commit message gives is no finding either, only a LOOSENED line.

STANDARDS: (a) every place the diff breaks a documented rule, citing file and rule; (b) baseline smells as judgement calls, dropped where a documented rule allows the pattern: mysterious name, duplicated code, feature envy, data clump, primitive obsession, repeated switch, shotgun surgery, speculative generality, middle man, complexity moved instead of reduced, silent fallback over an unclear invariant, conditional added to an unrelated flow. Skip what a linter or the build already enforces.

READ: every standards file you opened. A listed file you did not open: "not read: <file>, <why>".

WIRING: one line per entry point the diff adds or changes (resolver, route, handler, job, command, screen, public export): <entry point> | <file:line where the running app composes or registers it> | wired | missing. Find that place outside the diff, starting from the app's composition root. A test that calls the entry point's own module directly does not show wiring. Missing: a high CORRECTNESS finding.

CHECKS: one line per test goal the standards require for the touched files: <goal> | ran | not run. A goal counts as ran only when "Checks already run" names it. Not run: a medium CORRECTNESS finding.

LOOSENED: one line per Loosened check in the diff: <file:line> | <what it checks less> | <reason and where it stands> | <finding id when no reason is given>.

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

READ:
- <file>

WIRING:
<entry point lines>

CHECKS:
<goal lines>

LOOSENED:
<loosened lines>

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

Show the WIRING, CHECKS and LOOSENED lines, then the three sections as they came, in that order, without reranking across them. End with one line: findings per section and the worst one in each. What happens next (fix, ask, post) is the caller's job.
