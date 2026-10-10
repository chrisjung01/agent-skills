# PR description

Language from `conventions`. Run the `unslop` skill over it.

- **Goal**: what this PR delivers in two sentences, with links to the ticket and the spec comment.
- **Stack** (per-ticket, stacked only): `Stacked on #<PR>`, merge that one first.
- **Tickets** (`one-pr` only): a checklist, one line per ticket of this repo, checked when its commits are on the branch. It is the resume state for the next run.
- **Changes**: concrete behaviour that changed, in the user's terms. For UI changes, a before/after screenshot when the project can produce one.
- **Acceptance**: table with AC id, evidence, status. Edge-case rows the same way.
- **Deviations**: links to the deviation comments, or "none".
- **Deliberately unchanged**: areas a reader might expect to change, backed by the diff.
- **Checks and review**: commands run with results; the review outcome with fixed and rejected findings.
- **Merge danger**: one-way or two-way door, and the blast radius.
- **Links**: per-ticket: `Closes #<ticket>` and `Part of #<spec issue>`. `one-pr`: `Closes #<n>` for each done ticket of this repo, and `Part of #<spec issue>`. The spec's issue is closed by hand once every ticket is merged. Other repos: full reference (`owner/repo#n`).
