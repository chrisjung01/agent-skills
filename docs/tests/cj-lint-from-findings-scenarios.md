# cj-lint-from-findings scenarios

Dry runs by fresh subagents (role-play, nothing posted or pushed). RED: the subagent gets the caller's current text and no `cj-lint-from-findings`. GREEN: it also gets the new skill and the caller's new call line.

## A. Banned internal import

Setup: `cj-review-loop` in loop mode, a TypeScript repo with ESLint. Finding F2 (STANDARDS, medium) at `src/orders/list.ts:3`: "imports `src/billing/internal/tax.ts` directly; other modules must use `src/billing/index.ts`". Fixed in the working tree, uncommitted.
Expected: rung 2 (restricted imports with the pattern `**/billing/internal/*`), probe from `git show HEAD:src/orders/list.ts`, the line "fires on probe, fix clean, 0 hits in repo", one choice question with own PR first.

## B. Smell only

Setup: `cj-review-loop` final report with F1 "feature envy in `OrderMapper`" and F3 (medium) "mysterious name `tmp2`", both fixed.
Expected: one line "nothing to mechanize", no question, no file touched.

## C. Needs a custom AST rule

Setup: `cj-address-pr-feedback`, fixed thread: "every `useQuery` call must pass `staleTime` when the key starts with `user`".
Expected: rung 4, no plugin code, an issue draft with the pattern and the fix commit; the choice offers only issue and drop.

## D. Many hits elsewhere

Setup: as A, but the whole-repo run finds 12 other files with the same import.
Expected: the hits listed (`file:line`, at most 10, then "and 2 more"), the own-PR option says the rule lands at `warn`; nothing decided without the question.

## E. Config with uncommitted changes

Setup: as A, and `eslint.config.js` already has an uncommitted edit from the review fixes.
Expected: the agent copies the config before the proof and restores it after; `git status --short` before and after are equal and the earlier edit is still there.

## F. Retro with a mechanical standards finding

Setup: `/cj-retro`; the session shows a reviewer missing a `console.log` left in production code.
Expected: the finding is listed with the change "lint rule via cj-lint-from-findings"; after the user's yes in retro step 5, the agent calls the skill instead of filing a plain issue.

## G. Linter not runnable

Setup: as A, but `node_modules` is missing and the lint command fails with "Cannot find module".
Expected: the proof stops, the reply names the error, the finding is offered as an issue; config unchanged.

## Results

- RED (2026-10-13, one rep per scenario, callers before #63): 7 of 7 reps drafted no rule.
  - A, B, C, E, G: the run ends at the final report or output block, with no question and no rule.
  - D: the 12 other imports go unmentioned. "Change only what the findings name" keeps the loop off them, and nothing reports them.
  - F: `cj-retro` files a plain issue "enable `no-console`" without proving the rule fires.
  - G: the rep notes that "Red: repair first" pushes toward installing dependencies.
