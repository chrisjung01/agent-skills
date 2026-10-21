# Scenarios: cj-architecture-scan

Skill under test: `cj-architecture-scan`. Before the change there is no skill, so "before" is a fresh agent answering the same prompt without it. Every scenario is a paper exercise: the agent gets the skill text and the tool output below, describes its next steps and the text it would show, and runs nothing.

Shared setup unless a scenario says otherwise: repo `acme/shop`, a TypeScript monolith, 286 commits. `GLOSSARY.md` defines "Order intake" (the module that takes an order from the cart to a stored order) and "Pricing" (discounts and tax). `docs/adr/0004-pricing-stays-synchronous.md` (2026-03-02) decides that pricing runs inside the order request because the cart page shows the final price. No commits touch pricing since then except a tax-rate table.

```
$ git log -200 --name-only --format= | sort | uniq -c | sort -rn | head -10
  29 src/orders/checkout.ts
  24 src/orders/validate.ts
  22 src/pricing/discounts.ts
  21 src/orders/repo.ts
  20 src/mail/mailer.ts
  11 src/pricing/tax.ts
   9 src/cart/cart.ts
   8 src/api/orders.ts
   7 src/users/session.ts
   6 src/api/cart.ts
```

## 1. No direction: the hot spots are the scope (AC-1)

**Prompt:** "/cj-architecture-scan"
**Expected:** the scope is exactly the ten files above, found through the commit count, and the subagent's prompt names them; the page header reads "the last 200 commits" and lists the ten files. An agent that scans the whole `src/` tree, picks folders by name, or names no window on the page fails.

## 2. A direction replaces the window (AC-2)

**Prompt:** "/cj-architecture-scan src/mail, the mails are hard to change"
**Expected:** the scope is `src/mail` and the code that calls into it; no commit count runs for the scope; the page header names the direction ("the mail module, hard to change"), not a commit window.

## 3. Five cards at most, ordered, with a closing card (AC-3)

**Setup:** the subagent returns seven candidates: three Strong, two Worth exploring, two Speculative.
**Prompt:** "/cj-architecture-scan"
**Expected:** the page holds five candidate cards, strongest first, and one closing card that names card 1 or gives a reason for another; the two weakest candidates stay off the page. Every card has files, problem, change, benefit, a before-and-after diagram and exactly one badge from Strong, Worth exploring, Speculative.

## 4. The idea comes before the architecture term (AC-4)

**Setup:** candidate 1 merges `validate.ts`, `repo.ts` and the order part of `checkout.ts` into one place.
**Prompt:** "/cj-architecture-scan"
**Expected:** the card says in plain words and glossary terms what changes ("the order intake takes an order from the cart to a stored order in one place; validation and storage stop being separate files callers must know"), uses "Order intake", not "OrdersService" or "checkout handler", and names an architecture term (deep module, seam) at most once, at the card's end, with `/cj-teach <term>` as the way to learn it. A card that opens with "Deepen the OrdersService module" fails.

## 5. One message: summary, path, one question, nothing after (AC-6)

**Prompt:** "/cj-architecture-scan"
**Expected:** after the page is written, one message holds the chat summary, the page path and, at its end, one choice question in the shape of `cj-asking`: the candidates in card order, each with its card title, the change as meaning line and a consequence line, then the last option "none, keep the page". No second question, no "Next:" and no further text after the question. An agent that asks "which one?" without numbered options, or that lists the options only on the page, fails.

## 6. The pick becomes one issue, with one confirm and no grilling (AC-7)

**Setup:** the user answered "1".
**Prompt:** (the answer above)
**Expected:** the agent calls `cj-filing-issues` with one draft: kind task, a title naming the change in structure, a body with the problem, the change in structure, the benefit and the candidate's code files. The only question is the confirm `cj-filing-issues` asks through `cj-posting` before creating the issue. An agent that asks what the merged module should be called, what sits behind the seam or which tests survive, or that files two issues, fails.

## 7. No glossary, no ADR folder: nothing created, nothing asked (AC-9)

**Setup:** repo `acme/tools`, 120 commits, no `GLOSSARY.md`, no `docs/adr/`, no `AGENTS.md`. The count's top lines are `18 lib/parse.ts`, `15 lib/emit.ts`, `9 bin/cli.ts`.
**Prompt:** "/cj-architecture-scan"
**Expected:** the agent treats glossary and decisions as missing facts, asks no question about where the glossary or the ADRs are, creates no `GLOSSARY.md` and no ADR folder, and the cards use the repo's own file, module and function names. The window reads "all 120 commits". An agent that creates `GLOSSARY.md` lazily, or asks "where do you keep decisions?", fails.

## 8. Scattered hot spots: ask for a direction (edge case)

**Setup:** repo `acme/widgets`, 80 commits in the window, the count's top line is `2 src/a.ts` and no file appears more often.
**Prompt:** "/cj-architecture-scan"
**Expected:** the agent asks for a direction, as `cj-asking` says, naming the window (80 commits) and that no file appears in more than 2 of them. It does not widen the window, scan the whole repo or pick an area itself.

## 9. The user keeps the page (edge case)

**Setup:** the page shows three candidates; the user answered "none, keep the page".
**Prompt:** (the answer above)
**Expected:** no issue, no further question; the reply names the page path for later and ends. An agent that files an issue anyway, or asks whether to grill one candidate, fails.

## Runs

- 2026-10-21, scenarios 1 to 9, one blind run per version by a fresh subagent. Before (no skill, prompt read as "scan this repo for refactoring candidates"): 1 to 8 missed expected (whole `src/` scanned, no commit window, a markdown list of all seven candidates, "deeper module" before the idea, "want me to start?" instead of numbered options, the refactor started on "1", no window named, scatter never noticed); 9 met expected, so it stays as a regression guard. After, first draft of the skill text: 2 and 6 missed expected (callers not in the scope, `BODY.md` task rules and label questions unsettled), the rest met. After the review fixes, skill text at 70dd943: 1, 2, 3, 8 and 9 met expected on the full run, 4, 5, 6 and 7 met expected on the rerun; nine of nine.
