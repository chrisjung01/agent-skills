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

**Expected with skill:** The turn that asks shows the numbered findings list as reply text first, then calls the choice tool; options name findings in full words with their one consequence.

**Status:** verify in next real run. A paper exercise does not reproduce it: an agent describing its plan lists the findings, the failure shows only when the harness's choice tool is at hand.

## 4. Readable findings followed by a clickable choice

**Setup:** A project's retro has two small documentation findings: reduce planning overhead for reversible playtesting changes, and include log checking in the local smoke-check instructions. A choice tool is available in the active harness. The user wants a quick, readable decision.

**Prompt:** "/cj-retro"

**Baseline (real Codex run, session `01a12c9e-6dca-7c50-8ce4-4ec7373a8846`, 2026-10-11):** Each finding put a bold title and a file path on the same line, followed by a dense explanation. The agent ended with all three answer options embedded in one sentence instead of calling the available choice tool. The user approved both findings but explicitly criticised the list's readability and the missing selectable question.

**Expected with skill:** Show a numbered list with a short bold title per finding, one plain explanation sentence beneath it, and a blank line between findings. Include a linked destination in the explanation only when it helps the decision. After the list, use the available choice tool, with one question and selectable options. When the harness has no available choice tool, put the question and each answer option on separate lines.

**Status:** verify in next real run. The baseline failed in the live harness. The revised template can be inspected, but a paper exercise cannot verify the rendered list or actual choice-tool use. The current scanner does not store this Codex call, so no metrics call UUID is available.

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
