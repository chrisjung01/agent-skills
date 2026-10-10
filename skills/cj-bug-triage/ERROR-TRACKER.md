# Evidence from an error-tracker issue

Phases 0 to 2 when the input is an error-tracker issue. The evidence starts with a stack trace; believing it too early is the failure.

## Phase 0: find the chat alert

Search the alert channel (`channels`) for the issue ID and short ID first. Found: keep permalink and timestamp and read the whole thread; colleagues may already have diagnosed it, linked a ticket or marked it noise, which may end the analysis here. Not found: note it (not every issue is alerted) and continue.

## Phase 1: exhaust the error tracker

Compare several events, not only the latest:

- Frequency over time: a spike points at a release or an outage; steady background noise points at low impact.
- Affected users and companies in absolute numbers, not just event counts.
- Tag distribution: app variant, release version and build, user or company type, environment.
- Breadcrumbs before the crash: what did the user actually do?
- The tracker's AI analysis, if any, as a first hypothesis, not a finding.
- Any database or backend deep link the event carries.

Only non-production environments (`error-tracker`): not a production bug; say so and go to the verdict.

## Phase 2: code, duplicates and surroundings

Code:

- Map the stack frames to real files and read around them.
- Does the error travel through the app's own error-reporting path as a deliberately reported, handled state rather than a crash?
- An expected API, auth or connectivity failure with a handled UI path and a visible error state is not a bug.
- Fingerprinting: does the app or tracker group by a normalised message or custom fingerprint, merging unrelated causes into one issue?
- `git log` and `git blame` on the touched frames: does the first-seen date line up with a release (`releases`) or a commit? Name it.

Duplicates and surroundings:

- GitHub: `cj-filing-issues` (Find) with the module, the error message and the files from the stack frames.
- Error tracker: similar titles, same module tag, related fingerprints. A symptom of a larger family is reported as that family.
- Chat: the support channel (`channels`) around the spike. A human reporting the same thing raises confidence and impact sharply.
- Product docs (`product-docs`): known-issue pages or agreements covering this behaviour.
- Business data (`analytics`): only when impact numbers decide the verdict.

Done when frequency, affected users, tags and breadcrumbs are known, each source was searched, and you can name the path that throws and whether it is handled.
