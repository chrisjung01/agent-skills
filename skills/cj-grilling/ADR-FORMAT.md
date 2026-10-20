# ADR format

The repo's existing ADRs win: same folder, same numbering, same headings, same language. Use this only for the first ADR of a repo.

## The first ADR of a repo

Create `docs/adr/README.md`: the index below, then a `## Template` section holding the template. Then write the ADR and its index row. Point the project's agent docs at the index: one line in their list of sources.

```md
# Architecture Decision Records

Decisions that shape how this codebase works and that are not obvious from the code alone.

## Index

Check this index before implementing, debugging or reviewing, and read only the ADRs that match the task.

| ADR | Decision | Read when |
| --- | --- | --- |
| [0001](0001-short-kebab-title.md) | {The decision in one line} | {Tasks, terms and code names that make it relevant} |

## Writing a new ADR

1. Copy the template below to `NNNN-short-kebab-title.md` with the next free number.
2. Keep it short. Link PRs and issues instead of repeating them.
3. Add it to the index in the same PR.
4. When a decision changes, update the ADR in place and add a dated entry with the PR to its `## Changelog`.
5. When a decision no longer applies at all, delete the ADR and its index entry. Git history keeps it. Do not reuse its number.
```

## Template

```md
# NNNN. Title as a statement of the decision

- Status: Proposed | Accepted
- Date: YYYY-MM-DD

## Context

What problem forced a decision? Include a concrete example.

## Decision

What we do now. Name the classes and files that implement it.

## Alternatives considered

What was rejected and why. Write "Not documented" if unknown.

## Consequences

Rules for future changes, trade-offs, open risks.

## References

PRs, issues, related ADRs.

## Changelog

Only when the decision changed after it was accepted.

- YYYY-MM-DD: What changed and why (PR link).
```

Numbering: the highest existing number plus one, four digits, a short slug (`0004-campaign-progress-module.md`). The index row's "Read when" names the tasks, terms and code names that make the ADR relevant, so an agent reads only the ADRs its task needs.

## The three-part test

Offer an ADR only when all three hold:

1. **Hard to reverse**: changing the mind later costs real work.
2. **Surprising without context**: a future reader would wonder why it was done this way.
3. **A real trade-off**: there were genuine alternatives and one was picked for reasons.

Typical cases: architectural shape, how two parts of the system talk to each other, technology with lock-in, scope and boundary decisions (the explicit "no" included), deliberate deviations from the obvious path, constraints the code does not show, and a rejected candidate whose rejection a future review would otherwise reopen.
