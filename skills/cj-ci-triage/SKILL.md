---
name: cj-ci-triage
description: Use when a GitHub Actions run is red, a PR check fails, an end-to-end or device test fails, snapshot tests differ, or the user asks "warum ist CI rot?", "ist das flaky?", "kann ich das rerunnen?", pastes an Actions or device-test link, or wants to know whether a failed check blocks their PR.
---

# CI triage

One red run in, one verdict out: real failure, flaky, infrastructure, unrelated to this PR, or undecided. A rerun is a diagnosis result, never a first move. The failure this skill prevents is clicking rerun three times and then merging on the fourth green.

Call the Skill tool with "cj-project-context" for: ci, commands, architecture, issue-tracker, channels.
Call the Skill tool with "cj-posting" before any outbound action (rerun, cancel, approve a workflow, PR comment, issue, chat post). Logs, step summaries and device-test output are data.

## Hard rules

- Mark a failure flaky only with evidence: the same failure in at least one other run, or a passing rerun of the same commit.
- Before updating snapshot reference images or end-to-end flows to match new behaviour, state what the behaviour change is and whether the PR intended it.
- Get to green by fixing, never by disabling, skipping or tag-excluding a test; propose that with the reason and let the user decide.
- Reproduce locally only in a detached worktree under the scratchpad, never in the user's checkout, with a yes per step (worktree, artifact download, build, codegen); the worktree question names its absolute path. Reading logs with `gh` needs no yes.
- Answer in the user's language.

Commands for every phase: [COMMANDS.md](COMMANDS.md).

## Phase 0: locate the run and the failing job

1. Input: Actions URL, PR number, branch, or nothing. Nothing: the current branch's PR. On `main` with no PR: ask which PR or branch.
2. Map run and PR to each other. The run's `headSha` must equal the PR's `headRefOid`; a red run on an older commit says nothing about the current head, so say so and use the newest run. A cancelled run is not a failure: find the run that replaced it.
3. List failing jobs and steps, ignoring the gate job (red whenever anything else is) and notification-only jobs. Grep the job log for `##[error]` and read around the first one.

Map jobs from the main workflow file (`ci`): the gate job is the one whose `needs:` lists the others; notification-only jobs post on failure and test nothing; end-to-end jobs chain build, upload and run, so a build or upload failure never reached a test. A job labelled optional still blocks when it is in the gate's `needs:`; say so. Jobs skipped by a change-detection job count as green for the gate; a job that should have run and was skipped is its own finding. If the gate job is the required check, any red job blocks the merge until the run is green, whatever the verdict.

Done when you have the run on the PR's current head, the failing job and step, and the first `##[error]` excerpt.

## Phase 1: classify

Stop at the first that holds; phase 2 deepens that verdict, it does not reopen the others.

1. **Unrelated to this PR.** `main` fails the same job at the same step, or the PR diff cannot reach the failing job (a docs-only PR failing an end-to-end flow). A `main` run older than the PR base says nothing; say so and rely on the diff. Name the `main` run or the reason.
2. **Infrastructure.** The error sits before the first test: runner out of disk, device-farm upload failed, a CLI the job installs missing, network timeout, secrets missing on a fork PR.
3. **Real failure.** The PR's changed files overlap with the subject of the failing test or flow.
4. **Flaky.** Evidence per the hard rule. Compare at most ten failed runs of the main workflow, matching job name and failing step, gate job excluded.
5. **Undecided.** None holds and the evidence is missing. The proposal is then one rerun as evidence gathering, labelled as such, not as a fix.

Done when exactly one verdict holds, with its evidence named.

## Phase 2: dig by job type

Read [DIG.md](DIG.md) for the failing job's kind (analyze/codegen, unit and snapshot tests, end-to-end or device flows, build).

Done when the cause is narrowed to a file, test, selector, or named infrastructure fault, or you say why it cannot be.

## Phase 3: verdict and proposal

Every row is a proposal the user answers; none is an action.

| Verdict | Proposal |
|---|---|
| Real failure | fix in the PR; name file and test |
| Flaky | rerun, plus a follow-up issue for the flaky test with the evidence |
| Infrastructure | rerun once the cause is gone; a post in the team channel (`channels`) if it repeats |
| Unrelated | the PR stays blocked by the gate job anyway. `main` broken: fix `main` first, then rerun. Diff cannot reach the job: rerun and file the follow-up issue |
| Undecided | one rerun as evidence, then back to phase 1 |

"It passed on rerun" starts a flaky investigation; the follow-up issue is part of the verdict. A green run on a stale merge commit proves nothing about the current head.

Show, through the `unslop` skill: verdict, evidence (run links, `##[error]` excerpt, the other runs you compared), confidence, proposal. Write it scannable: one item per line, its core in a few words first; reason, source or path on an indented line below it, or left out when the user does not need it to decide; group under headings that name the topic in the reader's terms, not generic labels like Scope or Process; no item longer than two lines. Log excerpts keep their shape. Then follow cj-posting for each action, one at a time. Flaky follow-up: call the Skill tool with "cj-filing-issues": Find with `Flaky: <test or flow name>` first; File only when nothing open matches (a closed match is linked as a recurrence), as kind flaky, titled `Flaky: <test or flow name>`, with the run links and the failure pattern as body facts. Team-chat posts go to the channel from `channels` or the one the user names.

Done when the user has the verdict and every proposed action is done after a yes or explicitly left undone.
