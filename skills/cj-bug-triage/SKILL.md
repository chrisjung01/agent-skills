---
name: cj-bug-triage
description: 'Use when a possible bug needs a verdict and maybe a GitHub issue: the user pastes a Sentry or other error-tracker link, short ID or issue ID, or a bug arrives as a chat thread, support message, screenshot, screen recording, support ticket or verbal report; triggers on "triage this", "ist das ein Bug?", "mach daraus ein Ticket", "kannst du das reproduzieren?", "is this error real?".'
---

# Bug triage

One report in, one qualified verdict out. Two inputs, one shape: an **error-tracker issue** starts from a stack trace, a **human report** from what a person saw. The failure this skill prevents is a ticket for a network blip, or a ticket that says "user reports X does not work" and nothing else; every phase is a chance to find out it is not a bug.

Call the Skill tool with "cj-project-context" for: repos, commands, conventions, architecture, issue-tracker, releases, teams, core-flows, channels, error-tracker, analytics, product-docs, glossary.

Call the Skill tool with "cj-posting" before any outbound action. Leave the issue state in the error tracker unchanged; never resolve, ignore or assign it.

Cheap phases before expensive ones: a duplicate found early makes the code trace, a live run or a business-data query pointless.

## Input

- Error-tracker link, issue ID or short ID (organisation from `error-tracker`): follow [ERROR-TRACKER.md](ERROR-TRACKER.md) for phases 0 to 2.
- Chat permalink or thread, screenshot, recording, pasted support text, or a sentence from the user: follow [HUMAN-REPORT.md](HUMAN-REPORT.md) for phases 0 to 2.
- Nothing given: ask which report is meant.

## Phases 0 to 2: evidence

From the input's file. Both end with: every claim on its own line, the duplicate search done (call the Skill tool with "cj-filing-issues" (Find); duplicate and fixed-not-released end the analysis, fixed-and-released means a regression), and the code path that produces the behaviour named, or where the trace stops.

## Phase 3: verdict

Exactly one per claim:

| Verdict | Meaning |
|---|---|
| Confirmed bug | Real defect: code path or live reproduction, user impact, no duplicate |
| Expected behaviour | Works as designed; the alert or the report is the problem |
| False positive | Tracker noise: expected failure, handled state, test traffic (error tracker only) |
| UX finding | Not broken, but misleading (human report only) |
| Duplicate | Already tracked. Link it |
| Not enough data | Cannot decide; list what would decide it |

"Confirmed bug" requires all of: a code path or a live reproduction; impact in one sentence; no duplicate; not confined to a non-production environment; for a human report also the reporter's role, and the app when the path is variant-specific. One missing: "Not enough data", plus the concrete questions. State confidence; a guess is labelled as a guess.

Impact tier (one reporter does not mean one affected user; tiers 1 and 2 are about what breaks):

| Tier | When |
|---|---|
| 1 Major incident | a core flow (`core-flows`) broken, or wrong data reaches other users |
| 2 Minor incident | a core flow broken, workaround exists |
| 3 Low impact | wrong but harmless, or rare |
| 4 Glitch | cosmetic |

## Phase 4: output

1. **Report** in the user's language, through the `unslop` skill: verdict per claim, evidence per source, reproduction status (traced / reproduced live / neither), impact numbers, tier, confidence, open questions. Every claim cites its source. The user asked only "is it a bug?": stop here and offer the draft.
2. **Issue** on "Confirmed bug" or "UX finding": call the Skill tool with "cj-filing-issues" (File) with the presets in [ISSUE.md](ISSUE.md) and the relations found. The issue describes the problem; a reporter's proposed fix is a proposal. Never file on "Not enough data".
3. **Close the chat loop** when the input came from or was alerted in the chat: a thread reply in the thread's language with the verdict in one sentence and the issue link, or the concrete questions on "Not enough data". Ask the user to mark the alert or thread done, except on "Not enough data", which stays open.

Done when the report is given and every planned issue and reply is posted or explicitly not.
