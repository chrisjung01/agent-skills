# Architecture Decision Records

Decisions that shape how these skills work and that are not obvious from the skill text alone.

## Index

Check this index before changing a skill, debugging one or reviewing a change, and read only the ADRs that match the task.

| ADR | Decision | Read when |
| --- | --- | --- |
| [0001](0001-text-others-read-carries-no-injected-voice.md) | Text others read carries no injected voice; `unslop` runs for its pattern work only, a `Sources` block ends the text | Changing `cj-posting` step 2, a body template, or how a skill calls `unslop` |

## Writing a new ADR

Write one when the decision passes the three-part test in [ADR-FORMAT.md](../../skills/cj-grilling/ADR-FORMAT.md).

1. Copy the template below to `NNNN-short-kebab-title.md` with the next free number.
2. Keep it short. Link PRs and issues instead of repeating them.
3. Add it to the index in the same PR.
4. When a decision changes, update the ADR in place and add a dated entry with the PR to its `## Changelog`.
5. When a decision no longer applies at all, delete the ADR and its index entry. Git history keeps it. Do not reuse its number.

## Template

```markdown
# NNNN. Title as a statement of the decision

- Status: Proposed | Accepted
- Date: YYYY-MM-DD

## Context

What problem forced a decision? Include a concrete example.

## Decision

What we do now. Name the skills and files that implement it.

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
