# Evidence from a human report

Phases 0 to 2 when the input is what a person saw. Running the app comes last.

A quoted sentence without a link: say you are searching the team chat, search it for the quoted words, and ask for the permalink if nothing turns up. No support-ticket tool connected: work from what the user pastes.

**Incident first.** A core flow (`core-flows`) unusable or producing wrong results for its users, not merely ugly or slow: say so in the first sentence, before any analysis, and point to the incident process (`core-flows`; the bug template may name it too). Then continue; incidents get a ticket too, with tier 1 or 2. `core-flows` missing: no incident on a guess; put the tier question to the user with your proposal.

## Phase 0: read the report

- Chat: the whole thread; colleagues may already have answered, reproduced or linked a ticket. Note the reporter and their company or user role (`glossary`).
- Screenshot or recording: identify the screen by its widgets and strings, then find it in the code by grepping the user-visible strings (`architecture`). Without a thread there is no reporter to ask; say who you would need to ask.
- Extract what the user did, expected and got; app or variant (`architecture`), platform, version if visible. Several claims become one line each, each with its own verdict.

Missing did, expected or got: the first open question for the reporter. Missing app or platform is a question only when the code path turns out to be variant-specific.

## Phase 1: duplicates and history

- GitHub: `cj-filing-issues` (Find) with the screen or module, the symptom and the files you know. Blocked by, blocks, parent and related go into the issue.
- Error tracker (`error-tracker`): the module, screen or message. A match has the better evidence: continue from it with [ERROR-TRACKER.md](ERROR-TRACKER.md) and link it.
- Chat: the support channel (`channels`) for the same symptom from other reporters. Two reporters change the impact.
- Product docs (`product-docs`): known-issue or agreement pages describing this as intended.

## Phase 2: reproduce on paper

- Find the screen: grep the string resources (`architecture`) for a visible string, then the source for that key; from the screen, its feature module.
- Trace the whole path: screen, its state holder (view model, controller, store), then whatever that calls, often a shared repository, use case or sibling module. A pass-through state holder is not the end of the trace.
- Find the branch that produces the observed behaviour and the state, input, timing or role that reaches it, and check the reporter's role can reach it. A timing path (debounce, optimistic update, race) counts when you can name the two operations and their order.
- A handled state that merely looks wrong (error banner, empty state, permission block) is expected behaviour or a UX finding.
- An invariant that lives in the backend repo (`repos`): say so; the ticket names both repos.
- `git log` on the touched files; name a recent PR that changed this path.

Running the app (`commands`) is optional, needs a yes, and is skipped (say so) when the environment has no credentials for the reporter's role. A live reproduction beats a trace; say which one you have.

Done when every claim has did / expected / got or an open question, each source was searched, and you can name the branch and what reaches it, or where the trace stops.
