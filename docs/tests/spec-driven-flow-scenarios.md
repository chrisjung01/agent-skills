# Scenarios: spec-driven flow

Skills under test: `cj-grill-with-docs`, `cj-to-spec`, `cj-implement`, `cj-accept`, `cj-retro`.

Shared setup: app repo `acme/shop-client` and backend repo `acme/shop-backend`, default branch `main`, branch prefix `agent/`. Issue `acme/shop-client#412` "Push notification on new message" was written by a product manager. The agent has a subagent tool and `gh`.

## 1. Spec on someone else's issue

**Setup:** grilling for #412 finished in this session; the user confirmed the decision list.
**Prompt:** "/cj-to-spec #412"
**Expected:** a draft with AC-1… in Given/When/Then, edge cases, test seams, out of scope, merge danger; no file paths; test seams put to the user; posted as a comment starting with `## Spec` after a yes; the issue body is not edited; above the draft it says more than one ticket, since two repos are involved; after posting, the slice and the Epic title preview follow in the same run as a question of their own, nothing filed before its yes.

## 2. Implement without a spec

**Setup:** #415 "Show unread badge on chat tab" has no spec comment and three open product questions in its thread.
**Prompt:** "/cj-implement #415, ist eigentlich klar"
**Expected:** stops before any branch; names `/cj-grill-with-docs #415` and `/cj-to-spec #415`. No code, no branch.

## 3. Tickets across two repos

**Setup:** #412 has a spec and three sub-issues: `shop-backend#88` (trigger sends push), `shop-client#413` (show push, tap opens chat, blocked by #88), `shop-client#414` (setting "pushes off", blocked by #413). #88 is closed.
**Prompt (in shop-client):** "/cj-implement #412"
**Expected:** no mode given, two tickets in this repo: one choice question first ("One PR per ticket (recommended)" / "One PR for the whole spec"); answered with per ticket: #413 on a branch from `gh issue develop 413` with base `main` (blocker #88 closed), its own draft PR with `Closes #413` and `Part of #412`; then #414 on a branch from `gh issue develop 414` stacked on #413's branch (its blocker is open with a draft PR), draft PR against that branch with "Stacked on"; #88 not touched; #412 not closed by any PR. Each PR: checks, review, a Verified line per own AC; no merge-danger section, the spec holds it.

## 4. Deviation versus scope change

**Setup:** during #413 the agent finds the spec's chosen notification channel id collides with an existing one and picks another; later the user would need a new AC to cover muted chats.
**Expected:** the channel id goes as a `Deviation: … | … | …` comment on #412 and work continues, spec unchanged; the muted-chat need stops the run with a question, no code for it.

## 5. Context runs out

**Setup:** #500 has six tickets; after ticket 3 the window is heavy.
**Expected:** tickets 1–3 each have their own draft PR; report ends with `/clear`, then `/cj-implement #500`. A second run finds tickets 1–3 by their linked branches with open PRs, skips them, and starts ticket 4 (stacked on ticket 3's PR if it is blocked by it). If ticket 2's PR was squash-merged meanwhile, ticket 3's branch is restacked with `rebase --onto` and `--force-with-lease` first.

## 6. Spec that fits one ticket

**Prompt:** "/cj-to-spec #420", then "ja, posten" (grilled; the spec changes one label text and its test).
**Expected:** says above the draft that the spec is one ticket; after posting shows no slice, files nothing, proposes no title change; names `/clear`, then `/cj-implement #420`. Rewritten for #97.

## 7. Retro routing

**Setup:** in the session the agent twice used a banned logging call the reviewer caught; `cj-to-spec` had sliced one ticket too big for one context.
**Prompt:** "/cj-retro"
**Expected:** the logging call becomes a proposed lint rule or CI check (issue via `cj-filing-issues` after a yes), not a sentence in `AGENTS.md`; the oversized ticket is routed as `/cj-whetstone cj-to-spec`; no skill file is edited.

## 8. One-PR mode for a trial

**Setup:** as scenario 3.
**Prompt (in shop-client):** "/cj-implement #412 one-pr"
**Expected:** first line names the mode; one branch from `gh issue develop 412`; #413 and #414 as commits on it; one draft PR with a ticket checklist, `Closes #413`, `Closes #414`, `Part of #412`.

## 9. Ticket too big

**Setup:** while building #413 the diff reaches 650 changed lines without tests.
**Expected:** the run stops before checks and proposes a split (preparatory refactoring as its own ticket); no PR for the oversized branch.

## 10. Chat output against the rule block

**Setup:** a grader reads `cj-grilling`, `cj-to-spec` and `cj-posting` literally and judges each reply below: does the skill text forbid it, citing the line? The replies come from the Fortress-Architect session of 2026-10-12 and the evidence in #47. Each case is one catch of the test seams in #47.
**Cases:**
- a) Inline enumeration: "Open are the HP curve, the elite peak and the boss count."
- b) Nested list: a decision list whose items carry sub-bullets with the reasons.
- c) Table in chat: the decision list as a three-column Markdown table.
- d) Option names only a number: the reply text lists test seams 1 to 3, the confirmation question reads "A) confirm B) change seam 2".
- e) Made-up abbreviation: an option reads "1 HP linear ab W1 (Daten), Empf."
- f) Long draft, no summary: a spec of 4,600 characters, then right after it the question whether to post it.
**Expected:** every case is marked failed, each with the line that forbids it.

## 11. Handoff to acceptance

**Setup:** as scenario 3, #413 merged through PR #420. The run builds #414 and opens draft PR #421, base `main`.
**Prompt (in shop-client):** "/cj-implement #412 per-ticket"
**Expected:** the report's next step is `/cj-accept #412` with #421's link, ahead of merging #421, closing #412 and `/cj-retro`. Same with `one-pr` once its single PR is open. Had #420 still been open, the next step names merging #420 first and no `/cj-accept`.
**Variants** (the spec's Acceptance line and comments on #412 change; everything else as above):
- a) The spec ends with `Acceptance: by hand · behaviour a user sees`: the next step names `/cj-accept #412` as above.
- b) The spec ends with `Acceptance: verified is enough · no trigger applies`; no `## Challenge` comment, no `Deviation:` comment: the next step names merging #421 and closing #412, `/cj-retro` only after a session with friction, and no `/cj-accept`. Same once #421 is merged.
- c) As b), plus one `Deviation:` comment on #412: the next step names `/cj-accept #412` for the records only.
- d) The spec has no Acceptance line, as specs posted before #95: the next step names `/cj-accept #412` as above.
- e) Issue #430 "Fix typo in settings title" has no `## Spec` comment; the run used the issue as its spec and opened draft PR #431: the next step names merging #431 and closing #430, no `/cj-accept`.
- f) As c), the user runs `/cj-accept #412`: no row is asked; the deviation is sorted and written as step 4 says; the `## Acceptance` comment says verified is enough and lists the deviation.
- g) As c), after the run of f) posted its `## Acceptance` comment and #421 merged: `/cj-implement #412` names closing #412, not `/cj-accept`.
- h) As b), the user runs `/cj-accept #412`: it says nothing is to be checked or recorded and posts nothing.

## 12. Fix-now rows from acceptance

**Setup:** after scenario 11, #412 holds an `## Acceptance` comment checked on #421 at `abc1234`: AC-3 "not met: the tap opens the inbox, not the chat", choice fix now; the muted row has follow-up #440. #421 is still open.
**Prompt (in shop-client):** "/cj-implement #412"
**Expected:** no mode question and no `gh issue develop`; `git fetch`, then continues on #421's branch in its worktree, fast-forwarded to `origin`; a test for AC-3 that fails before the fix; checks, review, a Verified line for AC-3 citing the Acceptance comment in #421's updated description; the muted row is not built. Next step `/cj-accept #412`. A second run after the push finds that line, builds nothing and names `/cj-accept #412` again. Had #421's own description already marked AC-3 met before the comment, the row is still built.

## Results

2026-10-07, fresh subagent reading the skills literally.

- Run 1: scenarios 1, 2, 3, 4, 6, 7 pass; 5 fails. The resume worked "open" tickets (still open on GitHub until merge), had no way to find the branch or PR, and autosquash could rewrite pushed commits. Also found: `cj-filing-issues` edited the parent body against `cj-to-tickets`; the multi-repo `Closes` rule could not be followed; ticket bodies were not shown in full before the carried yes; a harness-specific worktree path.
- Fixed: step 1 finds earlier runs, step 3 skips tickets checked in the PR, fixups only on unpushed commits, plain push only, a new base only as a merge; `parent body untouched` and `body shape` presets in `cj-filing-issues`; `Closes #<spec>` only from the PR that finishes the last open ticket; full ticket bodies before the go; `--branch-repo` for a second repo.
- Run 2: scenarios 3, 4, 5 (first run and resume) pass. Its three remaining points (checkout of an existing branch, fixups on pushed commits, `--branch-repo`) were fixed afterwards.
- 2026-10-07, per-ticket PRs: `cj-implement` now opens one draft PR per ticket (stacked while one blocker is open) and keeps one PR per spec as mode `one-pr`; the spec's issue is closed by hand, which replaces the `Closes #<spec>` rule above. Run 3: scenarios 3, 4, 5 (first run, resume), 9 pass; 5 (restack after squash merge) and 8 (`one-pr` readiness) failed and were fixed: restack by the `Stacked on #P` line and #P's head commit, and a blocker inside the same `one-pr` unit counts as done once on the branch.
- 2026-10-13, scenario 10 (#47), fresh grader reading `cj-grilling`, `cj-to-spec`, `cj-posting` literally. Before the rule block: missed expected in a, b, c, e, f. Case d first named a grilling choice, which the old template's consequence slot half caught; rewritten as the seam question `cj-to-spec` prescribed, it missed expected before and met it after. With the block: met expected in all six; f rests on "longer than one screen", a judgement call. The run found that `cj-to-spec`'s seam shape was a nested list; the user chose a flat shape, one `catches:` or `misses:` line per case.
- 2026-10-13, handoff to acceptance (#56), fresh grader reading `cj-implement` literally. Before: scenarios 11 and 12 missed expected; the report had no step between an open last PR and `/cj-retro`, and fix-now rows counted as done tickets. After: 2, 3, 5, 8, 9, 11, 12 met expected. Also found and fixed: the next step dropped the mode argument, a fix-now row on a merged PR could loop back to `/cj-implement`, the `/cj-accept` line lacked the PR link. Review rounds 1 and 2: a fix-now row counted as done through the PR's own earlier Verified line, the worktree was not fetched, not-ready tickets and tickets in another repo looped or had no next step; all fixed, scenario 12 extended. After the fixes: 3, 5, 11, 12 met expected on the text before three wording fixes in step 2 and 4.
- 2026-10-14, scenario 11 variants (#95), fresh grader reading `cj-implement` and `cj-accept` literally. Before: base, a, d and g met expected, b, c, e, f and h missed. a, d and g stay as guards that specs by hand, specs without the line and a finished records-only run keep today's next step. After: all nine met expected. Review then found that a spec in an issue body could read as the small tier, and that the gate of `cj-accept` stopped records-only runs on such specs; both fixed with the `## Spec` start of the body as the marker. Review round 2 found four more gaps (an edited line after a records-only comment, step 1 of `cj-implement`, two wordings of records waiting, a needless merge); after their fixes all nine still met expected.
