# Phase 2: dig by job type

Read the section for the failing job's kind. Every local step (worktree, `gh run download`, build, codegen) needs its own yes and runs in the detached worktree under the scratchpad.

| Job kind | Typical cause |
|---|---|
| Analyze, lint, codegen check | lint, formatting, stale generated files |
| Unit tests | real failure or snapshot diff |
| Optional sync or template check | change not propagated; the fix is the sync command in `commands` |
| Build | dependency manifest, native deps |
| Upload to a device farm | infrastructure |
| End-to-end or device flows | selector break, backend state, device flake |

## Analyze / codegen

Run the analyze command and the codegen command from `commands`, then `git status`. The fix is a commit on the PR, not a rerun.

## Unit tests

Run the failing test with the targeted test command (`commands`). Snapshot test: propose `gh run download <id>` into the worktree; after the yes compare the images and decide: intended UI change (update the reference snapshots, say so in the PR) or regression (fix code). Sub-pixel diffs across many unrelated snapshots at once are font or platform drift, which is infrastructure.

## End-to-end or device flows

Facts first: the device farm's result link is usually in the `##[error]` line; if the workflow retries failed flows (`ci`), the log shows two attempts. Then:

- Which flow (the flow list from `architecture`), which step, which selector? Both attempts failing at the same step with the same selector: selector or real. Failing at different steps: device or backend state, not the selector.
- Selector not found: did the PR change the element's test identifier or its visible text? Check where identifiers and user-visible strings live (`architecture`) and the selector rule there. A copy change breaks text selectors.
- Selector present but state wrong: does the flow create its own test data per run, or depend on shared state in a test backend that another run may have changed? The latter is a flow-robustness issue, not a PR bug.
- Timeout on a screen that loads: device slowness. Say so; a rerun still needs the yes.
- Device-farm tool connected? Ask before using it, then fetch screenshots and step logs. Not connected? Work from the Actions log and say so.

## Build

Dependency change? Check the PR template's local-build checkboxes (`issue-tracker`). Reproduce with the build command the job runs.
