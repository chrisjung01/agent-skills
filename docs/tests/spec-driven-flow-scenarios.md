# Scenarios: spec-driven flow

Skills under test: `cj-grill-with-docs`, `cj-to-spec`, `cj-to-tickets`, `cj-implement`, `cj-retro`.

Shared setup: app repo `acme/shop-client` and backend repo `acme/shop-backend`, default branch `main`, branch prefix `agent/`. Issue `acme/shop-client#412` "Push notification on new message" was written by a product manager. The agent has a subagent tool and `gh`.

## 1. Spec on someone else's issue

**Setup:** grilling for #412 finished in this session; the user confirmed the decision list.
**Prompt:** "/cj-to-spec #412"
**Expected:** a draft with AC-1… in Given/When/Then, edge cases, test seams, out of scope, merge danger; no file paths; test seams put to the user; posted as a comment starting with `## Spec` after a yes; the issue body is not edited; next step named (`/cj-to-tickets #412`, since two repos are involved).

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

**Prompt:** "/cj-to-tickets #420" (spec: change one label text and its test).
**Expected:** files nothing; says the spec is one ticket; names `/cj-implement #420`.

## 7. Retro routing

**Setup:** in the session the agent twice used a banned logging call the reviewer caught; `cj-to-tickets` had produced one ticket too big for one context.
**Prompt:** "/cj-retro"
**Expected:** the logging call becomes a proposed lint rule or CI check (issue via `cj-filing-issues` after a yes), not a sentence in `AGENTS.md`; the oversized ticket is routed as `/cj-whetstone cj-to-tickets`; no skill file is edited.

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
- d) Option names only a number: the reply text lists decisions 1 to 3, the choice reads "A) 1 (recommended) B) 2 C) 3".
- e) Made-up abbreviation: an option reads "1 HP linear ab W1 (Daten), Empf."
- f) Long draft, no summary: a spec of 4,600 characters, then right after it the question whether to post it.
**Expected:** every case is marked failed, each with the line that forbids it.

## Results

2026-10-07, fresh subagent reading the skills literally.

- Run 1: scenarios 1, 2, 3, 4, 6, 7 pass; 5 fails. The resume worked "open" tickets (still open on GitHub until merge), had no way to find the branch or PR, and autosquash could rewrite pushed commits. Also found: `cj-filing-issues` edited the parent body against `cj-to-tickets`; the multi-repo `Closes` rule could not be followed; ticket bodies were not shown in full before the carried yes; a harness-specific worktree path.
- Fixed: step 1 finds earlier runs, step 3 skips tickets checked in the PR, fixups only on unpushed commits, plain push only, a new base only as a merge; `parent body untouched` and `body shape` presets in `cj-filing-issues`; `Closes #<spec>` only from the PR that finishes the last open ticket; full ticket bodies before the go; `--branch-repo` for a second repo.
- Run 2: scenarios 3, 4, 5 (first run and resume) pass. Its three remaining points (checkout of an existing branch, fixups on pushed commits, `--branch-repo`) were fixed afterwards.
- 2026-10-07, per-ticket PRs: `cj-implement` now opens one draft PR per ticket (stacked while one blocker is open) and keeps one PR per spec as mode `one-pr`; the spec's issue is closed by hand, which replaces the `Closes #<spec>` rule above. Run 3: scenarios 3, 4, 5 (first run, resume), 9 pass; 5 (restack after squash merge) and 8 (`one-pr` readiness) failed and were fixed: restack by the `Stacked on #P` line and #P's head commit, and a blocker inside the same `one-pr` unit counts as done once on the branch.
