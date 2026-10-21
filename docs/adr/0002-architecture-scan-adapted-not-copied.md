# 0002. The codebase scan is adapted from upstream, not copied, and hands its pick to the issue flow

- Status: Accepted
- Date: 2026-10-21

## Context

`improve-codebase-architecture` from mattpocock/skills scans a codebase for deepening candidates, shows them as an HTML report and grills the picked one at once, writing glossary and ADRs inline. An earlier adaptation, `cj-architecture-review`, was parked on 2026-10-07 after 45 days without a call; the user had forgotten it existed, not found it useless. Its text is not in this repo's history. #153 asked whether to adopt, adapt or leave out the upstream skill.

## Decision

A new user-invoked skill `cj-architecture-scan`, adapted from the upstream skill at commit d80fa0f:

- The page goes through `cj-html-report`, the pick question through `cj-asking`, glossary and ADR lookups through `cj-project-context`; no `HTML-REPORT.md`, no `agents/openai.yaml`.
- The picked candidate becomes an issue through `cj-filing-issues`; grilling, glossary and ADRs happen in `/cj-grill-with-docs`, so the big tier's challenge is not bypassed.
- Cards explain idea before name, as `cj-design-options` does (#86); no fixed architecture vocabulary.
- Without a direction from the user: hot spots of the last 200 commits, at most 5 cards.
- Visibility: a row in the README table "Flow and tools you start" and in the `AGENTS.md` skills table, and a pointer in `cj-design-options` for "where does the architecture hurt?"; the 45-day parking rule stays.
- The pick question lists the candidates in card order, so the numbers match the page; when the closing card names a card other than 1, the recommended option is not first. This is the one exception to the rule that the recommended option comes first.

## Alternatives considered

- Copy unchanged like `cj-teach`: calls `codebase-design`, `grilling` and `domain-modeling`, which are not installed here.
- Leave out: rejected because the parked skill failed on visibility, not on content.
- A shared vocabulary building block `cj-codebase-design`: two users would justify it, but #86 chose idea-before-name.

## Consequences

A scan ends in an issue, never in code. If `cj-architecture-scan` is parked again for no use, the visibility measures failed and this ADR should be revisited.

## References

- #153, the evaluation issue with the spec
- #86, `cj-design-options`
