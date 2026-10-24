# PR description

Language from `conventions`. Run the `unslop` skill over it as `cj-posting` step 2 says. The title names the target behaviour in the imperative ("Show the best wave in the run summary"), never a phase label or "fix bug".

Five blocks in this order. Each starts with its bold name on its first line (`**Why**: ...`); every further line, bullet or step is one line. Together they take at most 20 lines; images, the Stack line, the tickets checklist, the links and a `Sources` block do not count. The body needs more lines to be true: the unit is too big, say so and propose a split, as the 400-line size rule does. In `one-pr` the split means per-ticket PRs; the mode's own cost, not a reason to exceed the cap.

- **Stack** (per-ticket, stacked only, above Why): `Stacked on #<PR>`, merge that one first.
- **Why**: one or two sentences, with the links to the ticket and the spec comment. Merge danger lives in the spec, deviations are comments on its issue; both are reachable through these links, so neither is repeated here.
- **What**: three to five bullets in the user's terms. One bullet per affected screen: what was visible before and what is visible now (`Hero: before: no label; now: "Best wave: 5" under the score`); a new screen reads `before: not there`. Bullets beyond the screens describe other changed behaviour. A UI change in a project that can take a screenshot: one before/after pair here too, which complements the bullets and does not replace them. More than five affected screens: the unit is too big, split it.
- **How to test**: one to five numbered steps that a person can run without reading the diff. A step is one action in the app plus its visible result: quoted text, a number, a position or an appearance ("it works" does not count). The first step names the starting point (saved state, screen); each later step continues from the one before. A command counts as a step only when it is the only way to see the change, as for a change with no screen; its expected output stands next to it. For a change that has a screen, a screenshot or test command never replaces a tap.
- **Verified**: one line per piece of evidence (test name, command with result, or manual check with its observation): the AC and edge-case rows it covers, met, not met, or open with who shows it (after-PR rows of step 7), and for a test with a mutation check the inverted condition and its result (`test_x: AC-1 met, inverted <condition> → red`); rows with the same evidence share a line. The evidence comes from the two verifiers of step 7, never from the implementing agent. Then one line for the checks and one for the review outcome.
- **Risk**: what is untested or could break, or "none". A reproduction test the implementing agent wrote itself, because no subagent was available, is named here.
- **Tickets** (`one-pr` only): a checklist, one line per ticket of this repo, checked when its commits are on the branch. It is the resume state for the next run.
- **Links** (last; the `Sources` block from `cj-posting` step 2 comes after it): per-ticket: `Closes #<ticket>` and `Part of #<spec issue>`. `one-pr`: `Closes #<n>` for each done ticket of this repo, and `Part of #<spec issue>`. The spec's issue is closed by hand once every ticket is merged. Other repos: full reference (`owner/repo#n`).

Nothing else: no deviations, "deliberately unchanged" or merge-danger section.

Before the PR opens or its description changes, read the body against `git diff <remote>/<base>...HEAD`: every behaviour, file and test the body names is in the diff, every test named under Verified exists there. A claim the diff does not back: fix the body or the diff, read again. No PR until the body matches.

The same reading rejects the body, with no PR until fixed, for any of these:
- a step without a visible result in the forms above;
- a first step without a starting point;
- a command step without expected output, or one that replaces a tap on a change with a screen;
- a quoted text or number in a step that the diff does not produce;
- a What bullet that names a function or test where a screen needs before/after.
