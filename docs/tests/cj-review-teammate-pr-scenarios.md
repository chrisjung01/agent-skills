# Scenarios: cj-review-teammate-pr

Skill under test: `cj-review-teammate-pr` (phases 3 and 4, with `cj-reviewing`).

Fixture: a small repo and a PR diff. `src/holidays.ts` holds a list used only by `src/reminders.ts` (nightly e-mail job); `src/datePicker.ts` greys out weekends and never reads holidays. The PR adds 2026-11-01 and pins `date-fns` from `^3.6.0` to `3.6.0`. The repo has no Dependabot or Renovate config. The user writes German, the review language is German.

## 1. Drafts the user can paste as their own

**Prompt:** "Review PR #812" (phases 3 and 4 only, nothing posted).
**Expected:** each draft comment is first person in the user's tone, inside a fenced `markdown` block ready to paste. A code name the user may not know (`isHoliday`, `shouldSendReminder`) gets half a sentence on what it is in the findings list. Drafts as blockquotes, or in a neutral reviewer voice ("Es wird empfohlen …"), fail.
**Origin:** sessions 5256fdbc ("etwas menschlicher formulieren, als ob ich das geschrieben habe", "als Markdown formatieren"), da3b6ad3 ("claim, was meinst du damit?"), 8e75d963 ("Was macht das?").

## 2. Claims are checked before they become findings

**Prompt:** same as scenario 1.
**Expected:** no finding says users will see the holiday in the date picker (the picker never reads the list), and no finding says the pin stops Dependabot or Renovate updates (neither is configured). Anything the reviewer could not check is put as a question. A finding with either unchecked claim fails.
**Origin:** sessions 1375e96c (an unchecked Dependabot claim, withdrawn after "wenn wir uns das sehr kritisch anschauen?") and 5256fdbc (a user-visible effect claimed without following the callers).

## Runs

- 2026-10-09, before the change: one run. Scenario 1: drafts as `>` blockquotes, not paste-ready. Fail. Scenario 2: the reviewer read `datePicker.ts` and made no Dependabot claim. Pass: the fixture is too small to tempt the miss seen in large real repos. Verify in next real run.
- 2026-10-09, after the change: one run. Scenario 1: every draft in a fenced `markdown` block, first person, `date-fns` and `HOLIDAYS_2026` explained in the findings. Pass. Scenario 2: still pass; the out-of-diff missing holidays came as a question.
