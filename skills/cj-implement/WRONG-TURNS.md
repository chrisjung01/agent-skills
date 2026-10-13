# Typical wrong turns

| Situation | Required reaction |
| --- | --- |
| No spec, issue looks clear, time pressure | Only a trivial change uses the issue as its spec. Everything else goes back to `/cj-to-spec`. |
| No mode given, several tickets | Ask once with the two options. Never pick `one-pr` yourself. Fix-now rows from acceptance: no question, the last PR's mode. |
| A ticket grows past about 400 lines | Stop and propose a split. A big PR is the problem this skill avoids. |
| Two blockers still open | The ticket waits. Never stack on a local merge of two branches. |
| Stacked base was squash-merged | `rebase --onto` the default branch, `--force-with-lease` on this ticket's branch only. |
| Tempted to run the full suite after each commit | Targeted tests while working, full checks once per PR. |
| A technical detail differs from the spec | Deviation comment and continue. Never edit the spec for it. |
| Inline implementation, tests green | Still run the independent review. No reviewer: no push. |
| An inverted condition leaves its test green | A test is missing. Write it, invert again until it turns red, and name the new test in the Verified line. Never call the AC met on the old test. No input can tell the inverted condition apart: remove the dead condition. |
| A test fails and the expectation looks wrong | Ask. Never change the expectation to match the code. |
| Body names a test or file the diff lacks | Fix the body or the diff. No PR until they match. |
| Body needs more than 15 lines to be true | The unit is too big. Propose a split, as with 400 changed lines. |
| Everything green, PR text ready | Push and open the draft PR without asking. Never merge. |
| Branch name contains the issue number, so it is linked | Only `gh issue develop` links it. |
