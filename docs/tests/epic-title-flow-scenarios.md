# Scenarios: Epic titles in the spec-driven flow

Skills under test: `cj-to-spec` (with `TICKETS.md`), `cj-implement`.

Paper exercises. Give the agent the skill texts and one setup at a time, without the expected result. Ask it to record the publication preview, approval state, simulated GitHub actions in order, and final issue title. It must run no live writes. Other gates, checks and review have passed unless the setup says otherwise. Compare the action record with the expected result; format validation alone cannot prove these rules. Run each scenario five times to check whether the wording produces consistent decisions.

Shared setup: issue #40, title `Improve search: filters`, an approved spec, no sub-issues. A multiple-ticket split contains two vertical slices, `Filter by owner` and `Filter by date`. The parent holds their shared spec. Existing issue bodies stay untouched.

## 1. Post to an existing issue

**Setup:** The user approved posting the spec comment to #40. No split has been proposed.
**Prompt:** "Publish the approved spec. Record the actions and final title."
**Expected:** Post the `## Spec` comment; no Epic title proposal or title write. Final title `Improve search: filters`. Covers AC-1.

## 2. Post to an existing Epic

**Setup:** As scenario 1, but the title is `[Epic] Improve search: filters`.
**Prompt:** As scenario 1.
**Expected:** Post the comment; no title write. Preserve the existing prefix and the rest of the title. Covers AC-1 and the existing-prefix edge case.

## 3. Create the spec issue

**Setup:** No issue exists. The user approved creating an issue with the spec as its body and the title `Improve search: filters`.
**Prompt:** "Create the approved spec issue. Record the actions and final title."
**Expected:** Create the issue through `cj-filing-issues` with the approved title, without adding `[Epic] `. Covers AC-2.

## 4. One-ticket spec

**Setup:** `cj-to-spec` finds that the posted spec fits one context window in one repo.
**Prompt:** "Evaluate the spec and record the next step and any GitHub actions."
**Expected:** No sub-issues, no Epic proposal and no title write; name `cj-implement`. Covers AC-3.

## 5. Multiple tickets, approval pending

**Setup:** The two-ticket split is ready to preview. The user has not approved publication.
**Prompt:** "Show the publication preview and record any GitHub actions so far."
**Expected:** Show both ticket bodies and edges, current title `Improve search: filters`, proposed title `[Epic] Improve search: filters`, and the reason that the parent will hold the shared spec for these two sub-issues. Ask for approval of the split and title change; no writes while pending. Covers AC-4 and AC-6.

## 6. Approved split

**Setup:** The user approved scenario 5's two-ticket split and title change.
**Prompt:** "Publish the approved split. Record the actions and final title."
**Expected:** File and link the two sub-issues and their approved edges, and apply the proposed title as part of publication. Final title `[Epic] Improve search: filters`, with the rest preserved. Covers AC-5.

## 7. Approved split on an existing Epic

**Setup:** As scenario 6, but the preview showed current and proposed title `[Epic] Improve search: filters`, explaining that the prefix is already present.
**Prompt:** As scenario 6.
**Expected:** File and link the two sub-issues; no title write and no duplicate prefix. Covers AC-5 and the duplicate-prefix edge case.

## 8. Rejected split

**Setup:** The user rejects scenario 5's proposed split.
**Prompt:** "No, not like this. Record the next step and any GitHub actions." To the next question: "Build it in one PR."
**Expected:** First a question: revise the slice or build everything in one PR. After "one PR": no sub-issues and no title write; final title `Improve search: filters`; next step `/clear`, then `/cj-implement #40 one-pr`, never without `one-pr`. Covers AC-6 and the rejected-split edge case; rewritten for #97.

## 9. Revised split before approval

**Setup:** Scenario 5's preview was shown, but the user reduced the work to one ticket before approval.
**Prompt:** "It now fits one ticket. Update the proposal and record any GitHub actions."
**Expected:** Withdraw the multiple-ticket proposal and Epic title change; no writes. If instead the revised split still has multiple tickets, refresh its preview from the current title and split before asking again. Covers AC-3, AC-6 and the revised-preview edge case.

## 10. Implement a one-ticket spec with an existing prefix

**Setup:** `cj-implement` starts #40, title `[Epic] Improve search: filters`, no sub-issues. Its spec fits one ticket.
**Prompt:** "Start implementation. Record the branch and PR plan and any title actions."
**Expected:** One branch and one draft PR, `Closes #40`, no `Part of`; title remains `[Epic] Improve search: filters`. No routine removal instruction or carried permission for a title write. Covers AC-7.

## Results

2026-10-12, baseline at `7d123cf`: one fresh evaluator, five paper repetitions per scenario in that evaluator's context. All repetitions agreed. These exercises verify proposed agent actions, not live GitHub writes; the repetitions are not independent agent sessions.

| Scenario | Baseline action observation | Result |
|---|---|---|
| 1 | "Post the approved `## Spec` comment; ask for approval before adding `[Epic] `." Unwanted title proposal; the posting gate holds the rename. | Fail |
| 2 | "Post the comment; leave the already prefixed title unchanged." | Pass |
| 3 | "Ask approval for the changed title; hold issue creation." The skill adds an unapproved Epic prefix. | Fail |
| 4 | "File nothing; name `/cj-implement #40`." | Pass |
| 5 | Preview ticket bodies and edges, but omit the parent titles and Epic reason. No writes while approval is pending. | Fail |
| 6 | Create and link the sub-issues, set fields and read back edges; omit the approved title change. | Fail |
| 7 | Create and link the sub-issues; no title write or duplicate prefix. | Pass |
| 8 | "File nothing; name `/cj-implement #40`." | Pass |
| 9 | Return to the one-ticket path; no writes. | Pass |
| 10 | "Remove leading `[Epic] `" before branching; the old skill carries approval for this removal. | Fail |

Final state, 2026-10-12: a second fresh evaluator, five paper repetitions per scenario in its own context. All ten scenarios passed in each repetition, with no variation. Expected results were visible in the scenario file, so these were not blinded tests. The recorded actions cover titles and approvals; the fixtures do not supply full ticket bodies or concrete dependency edges for verifying the whole filing workflow.

| Scenarios | Final-state action observation |
|---|---|
| 1, 2 | Post the spec comment; no title proposal or write. Preserve an existing prefix. |
| 3 | Create the spec issue with the approved plain title; no automatic prefix. |
| 4 | No writes or Epic proposal; route to `cj-implement`. |
| 5 | Show current title, proposed Epic title and shared-spec reason; request split/title approval, with no writes pending it. |
| 6 | Create and link the approved sub-issues, apply the approved title, then read back title and edges. |
| 7 | Publish the split and read back; skip the title write because the prefix is already present. |
| 8, 9 | Withdraw the split and title proposal; no writes; route to the one-ticket path. |
| 10 | Plan one linked branch and draft PR with `Closes #40`, no `Part of`; preserve the existing Epic title. |

Two additional probes, five paper repetitions each, also passed without variation:

- A revised multiple-ticket split with current title `Improve search: saved filters` refreshes the preview to `[Epic] Improve search: saved filters`, explains the shared spec and waits for approval again.
- A generic "go" covering only spec publication posts the comment; it authorizes neither the later ticket split nor its title change.

No scenario executed live GitHub writes. The checks establish the action plans produced in these paper exercises, not compliance in every future session.

2026-10-15, scenarios 4 and 8 after the move into `cj-to-spec` (#97): one fresh grader each on the text before and after, setups without expected results. Before: 4 met expected, 8 missed (no question between revise and one PR, next step `/cj-implement #40` without `one-pr`). After: both met expected.
