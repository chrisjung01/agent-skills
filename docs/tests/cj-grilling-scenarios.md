# Scenarios: cj-grilling

Skill under test: `cj-grilling`.

## 1. A question that rests on a found PR links it

**Setup:** while grilling issue #5490 ("tenant-specific checkout rules"), the agent found merged PR https://github.com/acme/app/pull/4321 "TenantConfig for one tenant" by a colleague: it added a per-tenant config object, was reverted two weeks later, and is the obvious precedent.
**Prompt:** ask the next question: reuse that approach or build a new rule engine.
**Expected:** the question links the PR (the URL) and says in one line what it did. A question that names the colleague's approach without a link or summary fails.
**Origin:** sessions 6250afa1 ("kannst du mir mal kurz den PR von <colleague> zeigen?") and d68f497a ("kannst du mir mal die Links geben?").

**Variant used for the runs:** the PR appears only in research notes from 20 turns earlier, as "merged-then-reverted #4321 (a colleague) added a TenantConfig for one customer"; the question is "how are tenant rules modelled". Handing the URL in the prompt made the baseline pass.

## Runs

- 2026-10-09, before the change: variant three runs. All three wrote a bare `#4321` without a URL (one summary line each). Fail.
- 2026-10-09, after the change: variant three runs. All three gave `https://github.com/acme/app/pull/4321` with one line on what it did. Pass.
