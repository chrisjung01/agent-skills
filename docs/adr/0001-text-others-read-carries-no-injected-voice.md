# 0001. Text others read carries no injected voice

- Status: Proposed
- Date: 2026-10-21

## Context

Skills clean every text for people with the `unslop` skill. The local copy of that skill adds a step "Adding soul": opinions, first person, "let some mess in". A PR body or review reply written that way reads as a long personal text; the reader wants the point, the facts and their links. The rule block already says "Text others read keeps full sentences", but nothing stopped the soul step. #151, challenge comment: https://github.com/chrisjung01/agent-skills/issues/151#issuecomment-6319694268

## Decision

`cj-posting` step 2 states what text others read has after cleaning: it leads with the point, one idea per sentence, no opinion, first person or persona the facts do not carry, and a text that cites a fact from outside the repo (web, docs tool, error tracker, analytics, chat) ends with a `Sources` line of links. The `unslop` skill runs for its pattern work only, whichever copy is installed. The places that run `unslop` before the gate (`PR-BODY.md`, `BODY.md`, `SPEC.md`, the review text in `cj-review-teammate-pr`) point to that step instead of repeating it.

## Alternatives considered

- A new `cj-` writing discipline with a comparison of the three sources: rejected, no evidence of a gap beyond the one contradiction, and a fourth source to keep in sync.
- Copying the clause into every call site with a `validate.sh` check, like the rule block: rejected, the gate runs after every call site anyway.
- Changing the upstream `unslop` skills: out of scope, they are copies.

## Consequences

- A skill that wants voice in a text others read (a blog draft, a talk) names it as a target that keeps its own style; the gate already allows that.
- Chat text to the user is untouched; the rule block covers it.
- Kill criterion from the challenge: two or more user corrections about length or structure of text others read by 2026-12-20 reopen #151.

## References

- #151 and its challenge comment
- `skills/cj-posting/SKILL.md`, `GLOSSARY.md` "Text others read"
