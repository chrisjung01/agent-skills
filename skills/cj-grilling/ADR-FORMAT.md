# ADR format

The repo's existing ADRs win: same folder, same numbering, same headings, same language. Use this only for the first ADR of a repo.

```md
# {Short title of the decision}

{One to three sentences: the context, what was decided, and why.}
```

An ADR may be one paragraph. Add a section only when it carries something:

- **Status** (`proposed | accepted | superseded by ADR-NNNN`) when decisions get revisited
- **Considered options** when the rejected alternatives would otherwise be suggested again
- **Consequences** when the downstream effects are not obvious

Numbering: the highest existing number plus one, four digits, a short slug (`0004-campaign-progress-module.md`).

## The three-part test

Offer an ADR only when all three hold:

1. **Hard to reverse**: changing the mind later costs real work.
2. **Surprising without context**: a future reader would wonder why it was done this way.
3. **A real trade-off**: there were genuine alternatives and one was picked for reasons.

Typical cases: architectural shape, how two parts of the system talk to each other, technology with lock-in, scope and boundary decisions (the explicit "no" included), deliberate deviations from the obvious path, constraints the code does not show, and a rejected candidate whose rejection a future review would otherwise reopen.
