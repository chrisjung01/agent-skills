# Scenarios: cj-retro

Paper exercises built from a real run (October 2026). The agent under test describes what it would do; it posts nothing and edits nothing. Each scenario runs once against the current skill text (baseline) and once against the changed one.

## 1. A spec finding whose issue another skill wrote

**Setup:** A `cj-implement` session built ticket #204. Its acceptance criterion "the graphic tests run against the module" left open which of about 30 scene tests count as graphic tests; the agent drew the line itself. No `## Spec` comment exists; the ticket body came from an epic. The local transcripts show that #204's body was drafted in a session that ran a skill outside the skills repo.

**Prompt:** "/cj-retro"

**Baseline (real run):** The finding named `cj-to-spec` without checking and routed it to `/cj-whetstone cj-to-spec`; the whetstone run found no `cj-to-spec` call behind the issue and changed nothing.

**Expected with skill:** Before routing, the agent finds the skill that wrote the issue (search the transcripts for the issue title or a body line), names it, reports the finding as outside the skills repo, and does not name `/cj-whetstone`.

## 2. A skill finding whetstone can find later

**Setup:** Same session. A second finding is about `cj-implement` itself: its 400-line size rule counted lines that were only moved. The user answers "Alle" to the question which findings to do.

**Prompt:** "/cj-retro"

**Expected with skill:** The report names `/cj-whetstone cj-implement` and quotes the finding with its evidence (the moment in the session, the skill line), so a later whetstone run reads it in this retro window.

## Results (2026-10-11)

- Baseline, scenario 1: named `cj-to-tickets` "by inference from the ticket itself", searched no transcript ("nothing tells me to verify where a ticket came from"), routed to `/cj-whetstone cj-to-tickets`.
- With skill, scenarios 1 and 2: searched the transcripts for the issue title, named the outside skill and reported it only; quoted the `cj-implement` finding with the moment and the skill line under `/cj-whetstone cj-implement`.
- Left open: the category list has no slot for a skill rule that misfires; such findings reach the report only through step 5.

## 3. The question comes before the findings list

**Setup:** A session in the skills repo fixed the same chat-output defect in several skills. The retro finds two findings. Step 4 already said "never only inside the question".

**Prompt:** "/cj-retro"

**Baseline (real runs, sessions 4aadeed6, ab6be69d, dac7e3aa, a3d7de72, 70f8d77c):** After a last grep the agent called the choice tool at once; the findings stood only in the option descriptions ("1 AGENTS.md-Regel … 2 validate.sh-Check"), or only on the HTML page. In 70f8d77c, a run after the rule was added, the user rejected the question.

**Expected with skill:** The turn that asks shows the numbered findings list as reply text first, then the question as reply text, no picker call; options name findings in full words with their one consequence.

**Status:** since #109 the question is reply text, so nothing can hide the list; `cj-asking-scenarios.md` scenario 6 covers the picker temptation.

## 4. Readable findings followed by a reply-text question

**Setup:** A project's retro has two small documentation findings: reduce planning overhead for reversible playtesting changes, and include log checking in the local smoke-check instructions. A choice tool is available in the active harness. The user wants a quick, readable decision.

**Prompt:** "/cj-retro"

**Baseline (real Codex run, session `01a12c9e-6dca-7c50-8ce4-4ec7373a8846`, 2026-10-11):** Each finding put a bold title and a file path on the same line, followed by a dense explanation. The agent ended with all three answer options embedded in one sentence instead of calling the available choice tool. The user approved both findings but explicitly criticised the list's readability and the missing selectable question.

**Expected with skill:** Show a numbered list with a short bold title per finding, one plain explanation sentence beneath it, and a blank line between findings. Include a linked destination in the explanation only when it helps the decision. After the list, one question as reply text with each answer option on its own line, no picker call (#109).

**Status:** verify in next real run. The baseline failed in the live harness. The revised template can be inspected, but a paper exercise cannot verify the rendered list. The current scanner does not store this Codex call, so no metrics call UUID is available.

## 5. The current session holds only a run with nothing to do

**Setup:** The user ran `/clear`, then `/cj-implement <spec issue> per-ticket`, which found every ticket closed and its PR merged and only reported that. The work on the spec (worktree, reviews, a live test) happened in the session before the `/clear`, a separate transcript in the same project folder.

**Prompt:** "/cj-retro"

**Baseline (real run, session `e8a577ff-694f-4d30-b7c1-8d567bcf3332`, 2026-10-12):** Step 1 says "Default: the current one, before `/clear`". The agent listed the transcripts with `ls -lt`, guessed that the previous one (`4130c213`) was meant, and read it without saying so. The user approved the finding in a second retro.

**Expected with skill:** The agent sees that the current session holds only a run with nothing to do, reads the latest earlier session in this repo on the same issue, and names it in one line before the findings.

**Status:** verify in next real run. A paper run on 2026-10-12 with the old text already chose the earlier session and named it, because the prompt described both transcripts; the live failure was the guess without a rule.

## 6. A repo without draft pull requests

**Setup:** The retro's findings are small edits to agent docs. The repo's `conventions` fact (or a memory) says the repo cannot have draft PRs: a private repo on GitHub Free, where `gh pr create --draft` fails or opens a normal PR.

**Prompt:** "/cj-retro", the user answers "all".

**Baseline (real run, session `f6258d10-edac-46f5-b5ed-a9bd9bc78e89`, 2026-10-11):** Step 6 asks for "a draft PR". The agent opened a normal PR (#296) because the repo has no drafts and named the mismatch only in a later retro, which the user approved as a finding for cj-retro.

**Expected with skill:** The agent opens a normal PR where `conventions` says the repo has no drafts and says so in the report.

**Status:** verify in next real run. A paper run on 2026-10-12 with the old text already opened a normal PR and said so, because the prompt stated the missing drafts; the live run met the mismatch only through `gh`.

## Findings for scenarios 7 to 10

A `cj-implement` session in an app repo (not the skills repo) produced four findings, ranked:

- F1: the agent searched ten minutes for the test helper folder. Change: a one-line pointer in `AGENTS.md`.
- F2: `AGENTS.md` names a lint command that no longer exists. Change: fix the command in `AGENTS.md`.
- F3: the reviewer missed that two screens format prices differently; nobody has decided which format is right. Change: a price-format rule in the standards file.
- F4: `cj-implement` let the PR body grow past its cap. Change: routed to `cj-whetstone cj-implement`.

## 7. One route question per finding

**Setup:** Findings F1 to F3 only. A choice tool is available.

**Prompt:** "/cj-retro"

**Expected with skill:** Three questions, one per turn, in rank order. Each offers direct PR, issue with spec and not at all. The agent's estimate comes first, marked recommended, and its description says why: direct PR for F1 and F2, issue with spec for F3 because its format is still undecided. No question asks "all, pick per finding, none".

## 8. Two direct-PR findings share one PR

**Setup:** Findings F1 to F3. The user answers direct PR for F1 and F2, not at all for F3.

**Prompt:** "/cj-retro"

**Expected with skill:** One branch and one draft PR hold both edits, one commit per finding. F3 is listed as declined. No issue is filed.

## 9. A filed issue leads into grilling

**Setup:** Findings F1 to F3. The user answers not at all for F1, issue with spec for F2 and F3.

**Prompt:** "/cj-retro"

**Expected with skill:** No branch, no PR. Two issues filed through `cj-filing-issues`. The `Next:` block has one `/cj-grill-with-docs <issue link>` line for each. F1 is listed as declined.

## 10. A skill finding keeps its whetstone route

**Setup:** Findings F1 and F4.

**Prompt:** "/cj-retro"

**Expected with skill:** Two questions, one per turn: F1's first, then F4's on its own. F4's question offers only "hand to whetstone" (recommended) and "not at all", never direct PR or issue. No question covers both findings at once. On "hand to whetstone" the report quotes F4 with its evidence and names `/cj-whetstone cj-implement`.

## 11. A small retro PR in a repo that allows auto-merge

**Setup:** one finding answered direct PR; the repo's `conventions` allow auto-merge for small agent PRs and `main` lists required status checks. The committed diff against `origin/main` is 20 insertions and 4 deletions.

**Prompt:** "/cj-retro", the user picks direct PR.

**Expected:** a normal PR, and after the push `gh pr merge <n> --auto --squash`. Variants: 100 or more lines, or `conventions` silent on auto-merge: draft, no merge command. Protection API answers 404: not a draft, no auto-merge, the report names the reason.

**Source:** spec #104 (2026-10-14).

## 12. The PR repo differs from the session's start repo

**Setup:** The session started in the `fortress` checkout and loaded its project context there: `conventions` say every PR carries the label `skip-changelog` and branches are `task/<slug>`. The work then moved to `~/code/agent-skills`, whose `AGENTS.md` names branches `fix/<slug>`, no PR labels, and whose `gh label list` has no `skip-changelog`. One finding, an `AGENTS.md` line in agent-skills, is answered direct PR.

**Prompt:** "/cj-retro", the user picks direct PR.

**Expected:** branch, labels and draft rule come from agent-skills: branch `fix/<slug>`, `gh pr create` without `--label skip-changelog` and without any label from fortress. Missed: a label, branch shape or other convention taken from fortress.

**Source:** session 6bcdc49e (2026-10-12), `gh pr create` failed on the missing label; retro finding handed to whetstone with the user's yes.

## 13. The report names the state of the shared PR

**Setup:** Two findings answered direct PR, one declined. The agent discarded and rewrote one edit along the way, then committed both, pushed the branch and opened draft PR #83.

**Prompt:** "/cj-retro", the user picks the routes.

**Expected:** the report gives, for the shared PR, its branch, that both commits are committed and pushed, and that #83 is a draft (or set to auto-merge), next to its link. Missed: a report with route and link only, which leaves the user to ask whether everything is committed and the draft is open.

**Source:** session 75b4d38d (2026-10-13), the user asked "hast du alles commitet und einen draft aufgemacht?"; retro finding handed to whetstone with the user's yes.

## Results for scenarios 7 to 10 (2026-10-13)

- Baseline, scenario 7: missed expected; one question "which of these three", options all, pick per finding, none.
- Baseline, scenario 8: missed expected; one PR from the singular "a draft PR", but commits "per repo convention"; one commit per finding missed.
- Baseline, scenario 9: missed expected; issues filed, "with spec" left open, no `Next:` block and no grilling line.
- Baseline, scenario 10: missed expected; the first question asked "which of these two" for both findings, with all, pick per finding and none.
- With skill, first run: scenarios 7 to 10 met expected, but later turns lacked their finding's entry, and scenario 8 invented a `Next:` line.
- With skill, final text: scenarios 7 to 10 met expected; scenario 8 ends without a `Next:` block, and later turns open with their finding's entry.
- Left open: the choice tool hides the list written before it in some terminals (#21). Resolved by #109: questions are reply text.

## Results for scenarios 12 and 13 (2026-10-15)

- Baseline, three runs: scenarios 12 and 13 met expected; on paper the agents reload the context for agent-skills and report the PR state unprompted.
- The real failures came deep into long sessions, with the start repo's context loaded hours earlier; a paper run does not reproduce that.
- Verify in next real run: a retro whose PR goes to another repo than the start repo, and a retro report after a direct PR.

## 14. The report's `Next:` list starts with `/clear`

**Setup:** repo `acme/idle-quest`. A retro has three findings, all answered: finding 1 direct PR (draft PR `#41` opened, committed and pushed), finding 2 issue with spec (issue `#42` filed), finding 3 handed to whetstone (`cj-reviewing`). The session is long; nothing in it is needed by the next commands.
**Prompt:** (the last answer was given) write the retro report.
**Expected:** the report lists the three routes with their links, then a `Next:` list whose first line is `/clear` with its reason (session context not needed), followed by `/cj-grill-with-docs <link to #42>` and `/cj-whetstone cj-reviewing`. A report whose `Next:` list starts with `/cj-grill-with-docs` or leaves open whether to clear fails. With only a direct PR and no issue or whetstone finding, the list holds `/clear` alone.
**Origin:** sessions f4b0b38b ("bitte auch mit /clear als next steps anzeigen", 2026-10-16) and 2f7e34ad ("mit clear oder ohne ? :)", 2026-10-25). The first led only to a project memory.

## Results for scenario 14

- 2026-10-25, before the change (text of `7da78ca`): one fresh grader on paper. The `Next:` list started with `/cj-grill-with-docs` and did not say whether to `/clear`. Missed expected.
- 2026-10-25, after the change: one fresh grader on paper, two runs. With an issue and a whetstone finding the list started with `/clear · session context not needed`; with a direct PR only it held `/clear · session finished`. Met expected. Verify in next real run: the next whetstone run checks later `cj-retro` windows for a user question about clearing.
