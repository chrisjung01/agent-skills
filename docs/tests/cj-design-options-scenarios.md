# Scenarios: cj-design-options

Skill under test: `cj-design-options`. Before the change there is no skill, so "before" is a fresh agent answering the same prompt without it.

## 1. "Should we use a queue?" gets forces before candidates

**Setup:** repo `acme/shop`, a TypeScript monolith on one Postgres database, owned by a team of three. `OrderService.checkout()` in `src/orders/checkout.ts` sends the confirmation mail with `mailer.send()` right after the order row commits. `git log`: `checkout.ts` changed 3 times in 12 months, `src/mail/mailer.ts` twice. Issue #88 "checkout hangs 30 s when the mail provider is slow": the error tracker shows 4 timeouts in about 20,000 checkouts last month. `mailer.send()` has two callers, checkout and password reset. There is no queue or broker in the stack.
**Prompt:** "Sollen wir die Bestellmail über eine Queue schicken? Hab gelesen, so macht man das. RabbitMQ oder so."
**Expected:** the reply names the forces with their sources before the first candidate. The candidates include a simple one that adds no broker. RabbitMQ, if it stays, names the force it solves here and its cost. Adding a broker counts as hard to reverse, so three subagents with different constraints design the candidates. The reply ends with one recommendation, the force it rests on, a flip condition, and a choice question; it decides nothing. A reply that recommends a queue without naming forces, or that settles the decision, fails.
**Origin:** agent-skills#86, acceptance criteria 1, 2, 3 and 5.

## 2. "Where does the architecture hurt?" points to the scan instead of framing a decision

**Setup:** repo `acme/shop` as in scenario 1, no issue, no decision named. Skill under test: `cj-design-options` with its description and step 1 at the commit named in Results.
**Prompt:** "Wo tut die Architektur gerade am meisten weh? Was sollten wir refactoren?"
**Expected:** the description of `cj-design-options` matches these words, and step 1 names `/cj-architecture-scan` as the next step and stops, because no decision is framed. An agent that frames a decision from the words, lists forces, or proposes candidates for the whole repo fails.
**Origin:** agent-skills#153, acceptance criterion 12.

## Runs

- 2026-10-13, before (no skill): one run of scenario 1. Named the forces and a simple candidate, priced RabbitMQ, ended with a choice. Opened with "Nein" before any candidate, dispatched no subagents, named no flip condition: missed expected.
- 2026-10-13, after: one run of scenario 1. Forces with sources and two marked "assumed", three subagents under the three constraints, the pattern names last, a recommendation with two flip conditions, a choice question: met expected.
- 2026-10-21, scenario 2, one blind run per version by a fresh subagent. Before, text at 3138735: missed expected; the description names no hurt or refactor phrase, so the skill does not fire, and if it did, step 1 would frame a decision from the words. After, text at 70dd943: met expected; the description matches "was sollten wir refactoren?" word for word, and step 1 names `/cj-architecture-scan` and stops with no forces and no candidates.

Dropped: a scenario on an interface "for later" in front of Stripe. Without the skill the agent already refused the interface and named a test fake as the second adapter, so it could not tell the versions apart. Its run with the skill found two contradictions, now fixed: step 1 ruled out a page for cheap decisions while step 5 asked for one at three candidates, and the third-party category read as an order to add a port.
