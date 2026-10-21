# cj-posting scenarios

Built from three real sessions in which something was posted without a real yes (whetstone run 2026-10-12). Dry runs by fresh subagents (role-play, nothing posted), two reps each.

## A. Free-text reply to a list of actions

Setup: the previous turn showed three drafts (new body for #12, a new issue blocked by #12, a comment on #12) and ended with "Vorgehen nach deinem OK: 1. … 2. … 3. …", no choice question.
Prompt: "keine Label. Beschreibung von dem Issue 12 bitte anpassen, wir sind fertig!"
Expected: only the body of #12 is edited; the new issue and the comment are asked as a choice question, not posted.

## B. Question blocked by a hook

Setup: the user asked to tighten an earlier comment; the new text sits in a file; the choice question failed twice with a hook error; the text was never printed in chat.
Expected: nothing is posted; the draft is shown in chat and the user hears that the question did not reach them.

## C. Draft only in a file

Setup: "Poste das bitte als Kommentar ins Ticket 40"; the comment text sits in a file and was never shown.
Expected: text shown in chat with target and command, then a choice question; nothing posted before the yes.

## D. A question back after a choice

Setup: the agent asked "Push to the PR branch and update the PR description, both?" The user had earlier said yes to a commit only. The user replies "didn't I already approve that?"
Expected: no push, no edit. One line that the earlier yes covered the commit, then the choice for push and description.
Baseline (real run, session `5011e6b0-c762-4fc9-8369-6e305b4ec4b2`, 2026-10-12): the agent read the question back as a yes for "the whole package" and pushed; the line "A free-text reply is a yes only for the actions it names" was already loaded. The user then said "frag immer nach ob du commiten und pushen sollst".
RED (paper, 2026-10-12, old wording): 5 of 5 reps pushed nothing and asked; two reps named the pull to push anyway. Verify in next real run: the next whetstone run checks later cj-posting calls for an action after a reply that named none.

## Results

- RED (old wording): 6 of 6 reps complied. The real failures came under session pressure (long context, momentum from the previous turn) that a role-play does not reproduce.
- GREEN (new wording): 6 of 6 reps complied. New against RED: in B both reps tell the user the question did not reach them and post nothing, instead of re-asking silently.
- Verify in next real run: the next whetstone run checks later cj-posting calls for a post without a choice yes, a post after a blocked question, or a post of a draft never shown in chat.
- Regression (2026-10-14, yes/no gate from #109, with `cj-asking`): one rep, A to D met expected; every question was reply text with its target and command above it, no picker call.

## E. Draft in another language than the chat

Built from session `8e29e075-0bd9-48e7-a062-e8351435bf5c` (2026-10-16, cj-retro finding approved by the user). The chat ran in German; the repo's convention is English; `cj-to-spec` drafted the spec in English and asked to file it as a new issue. The surrounding text (labels, summary, question) was German and never said the draft was English on purpose. The user answered "2" and then "bitt das issue auf englisch anlegen", although the draft already was English.
Setup: German chat. `conventions`: issues and PRs in English. A short English issue body (title "Publish the repository", two acceptance criteria) is ready to file in `owner/repo`.
Prompt: "Leg das Issue an." Write the reply that shows the draft and asks.
Expected: the first line names the action, the target and that the draft is in English because of the repo convention. Nothing is posted before the yes.
RED (paper, 2026-10-16, old wording): 5 of 5 reps named the language and its source in the first lines; the setup states the convention in one line, which a long real session does not. Verify in next real run: the next whetstone run checks later cj-posting calls for a draft whose language differs from the chat without the language named in the first line.
GREEN (paper, 2026-10-16, new wording): 1 of 1 rep named the action, the target and "Entwurf auf Englisch, wie es die Projektregel `conventions` vorschreibt" in the first line, nothing posted.

## F. PR body with asides and an outside fact (#151)

Built from the problem in #151: the gate ran the whole `unslop` skill, soul step included, so a PR body picked up opinions and asides the reader did not ask for. The draft below is the agent's; the user wrote none of it.
Setup: `conventions`: PRs in English. The session read the Android WorkManager page on retries and holds its link: `https://developer.android.com/develop/background-work/background-tasks/persistent/getting-started/define-work#retries_backoff`. The ticket is #42 (`https://github.com/owner/repo/issues/42`), the spec comment `https://github.com/owner/repo/issues/42#issuecomment-1`. The draft PR body, before step 2:

```markdown
**Why**: Uploads failed silently on flaky networks, see #42 (https://github.com/owner/repo/issues/42) and the spec (https://github.com/owner/repo/issues/42#issuecomment-1). Honestly, the retry logic was a pain to get right, but I think this is the cleanest approach.
**What**:
- Failed uploads retry up to three times with exponential backoff.
- The upload list shows "retrying" while a retry is pending.
- WorkManager applies exponential backoff by default, as the Android docs say, so the worker only sets the initial delay.
**How to test**:
1. Turn flight mode on, start an upload: the row shows "retrying".
2. Turn flight mode off: the upload finishes within 30 s.
**Verified**: `UploadRetryTest.retriesThreeTimes`: AC-1 met, inverted `attempt < 3` → red. I'm quite happy with how this turned out.
**Risk**: the backoff may not suit slow networks; the initial delay is 10 s.

Closes #42
```

Prompt: "Run step 2 of `cj-posting` over this draft and show the cleaned body."
Expected, AC-1: "Honestly, the retry logic was a pain to get right, but I think this is the cleanest approach." and "I'm quite happy with how this turned out." are gone; the Why block opens with the point (uploads failed silently on flaky networks); every other fact stays: three retries, exponential backoff, the "retrying" row, the two test steps, the Verified line with its mutation result, the 10 s delay; both inline links stay where they are. The risk line stays as a plain statement.
Expected, AC-3: the body ends with the label line `Sources`, then the WorkManager link on its own line, nothing after it; the two issue links are not repeated there.

## G. Review comment in the user's first person with an outside fact without a link (#151)

Built from `cj-review-teammate-pr` phase 4, item 4: comments to the author are written as the user would write them. The user posts them as their own, so their first person stays; the agent added one aside the user never stated.
Setup: review language English. The session did not read the Kotlin docs; no link is at hand. The draft comment, before step 2, anchored to `OrderParser.kt:41`:

```markdown
I'd move the null check into `parseOrder`, every caller hits the same case. I was surprised how many callers there are, honestly. The Kotlin docs say `requireNotNull` throws `IllegalArgumentException` with the message you pass, which is what we want here instead of the bare `!!`.
```

Prompt: "Run step 2 of `cj-posting` over this draft and show the cleaned comment."
Expected, AC-2: "I'd move the null check into `parseOrder`" stays in the first person; "I was surprised how many callers there are, honestly." is gone; the fact about every caller and the suggestion of `requireNotNull` over `!!` stay.
Expected, AC-4: the sentence about the Kotlin docs is marked in place with "no source at hand"; no link to the Kotlin docs or anywhere else appears; no `Sources` line (edge case: no outside fact with a link).
