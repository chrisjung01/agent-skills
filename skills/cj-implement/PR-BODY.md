# PR description

Language from `conventions`. Run the `unslop` skill over it. The title names the target behaviour in the imperative ("Show the best wave in the run summary"), never a phase label or "fix bug".

Five blocks in this order. Each starts with its bold name on its first line (`**Why**: ...`); every further line, bullet or step is one line. Together they take at most 15 lines; images, the Stack line, the tickets checklist and the links do not count. The body needs more lines to be true: the unit is too big, say so and propose a split, as the 400-line size rule does. In `one-pr` the split means per-ticket PRs; the mode's own cost, not a reason to exceed the cap.

- **Stack** (per-ticket, stacked only, above Why): `Stacked on #<PR>`, merge that one first.
- **Why**: one or two sentences, with the links to the ticket and the spec comment. Merge danger lives in the spec, deviations are comments on its issue; both are reachable through these links, so neither is repeated here.
- **What**: three to five bullets of changed behaviour in the user's terms. A UI change in a project that can take a screenshot: one before/after pair here.
- **How to test**: one to five numbered steps, each one action with its expected result, that a person can run without reading the diff. Nothing to try by hand: the one command that shows the change instead.
- **Verified**: one line per piece of evidence (test name, command with result, or manual check with its observation): the AC and edge-case rows it covers, met or not met; rows with the same evidence share a line. Then one line for the checks and one for the review outcome.
- **Risk**: what is untested or could break, or "none".
- **Tickets** (`one-pr` only): a checklist, one line per ticket of this repo, checked when its commits are on the branch. It is the resume state for the next run.
- **Links** (last): per-ticket: `Closes #<ticket>` and `Part of #<spec issue>`. `one-pr`: `Closes #<n>` for each done ticket of this repo, and `Part of #<spec issue>`. The spec's issue is closed by hand once every ticket is merged. Other repos: full reference (`owner/repo#n`).

Nothing else: no deviations, "deliberately unchanged" or merge-danger section.

Before the PR opens or its description changes, read the body against `git diff <remote>/<base>...HEAD`: every behaviour, file and test the body names is in the diff, every test named under Verified exists there. A claim the diff does not back: fix the body or the diff, read again. No PR until the body matches.
