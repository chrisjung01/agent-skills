# AGENTS.md

This repo holds personal agent skills for Claude Code and Codex. Each skill is a folder under `skills/` with a `SKILL.md`. See [README.md](README.md) for installation and details.
Domain terms (skill call, window, nested skill, …) are defined in [GLOSSARY.md](GLOSSARY.md).

## Skills

| Skill | Purpose |
|---|---|
| `cj-challenge` | Flow 0: question whether to build at all; block on missing goal link, evidence or a smaller lever |
| `cj-grill-with-docs` | Flow 1: grill an issue or idea until every decision is made, glossary and ADRs inline |
| `cj-to-spec` | Flow 2: post the spec as a `## Spec` comment on the issue (or a new issue) |
| `cj-to-tickets` | Flow 3: slice the spec into small vertical sub-issues with blocked-by edges |
| `cj-implement` | Flow 4: one small reviewed draft PR per ticket, stacked while a blocker is open; `one-pr` mode for one PR per spec |
| `cj-retro` | Flow 5: propose changes to the worked-in repo's environment after a session |
| `cj-bug-triage` | Verdict and issue for an error-tracker issue or a human bug report |
| `cj-review-loop` | Review your own branch: one pass, or fix in rounds until no high or medium findings remain |
| `cj-reviewing` | The reviewer brief (CORRECTNESS, STANDARDS, SPEC) and the one severity scale every review uses |
| `cj-review-teammate-pr` | Review a colleague's PR against issue and agreements |
| `cj-address-pr-feedback` | Work through review threads on your own PR |
| `cj-ci-triage` | Classify a red CI run: real, flaky, infrastructure, unrelated |
| `cj-whetstone` | Improve a skill from the skill calls the whetstone metrics store, after reading their windows |
| `cj-grilling` | Interview the user one question at a time until every decision is made; sharpen domain terms, write glossary and ADRs |
| `cj-html-report` | Render a result as a visual page (cards, diagrams) next to the chat summary |
| `cj-filing-issues` | Find duplicates and dependencies, then create issues with good bodies and wire parent and blocked-by edges |
| `cj-posting` | The one gate for anything others see or that changes shared state: draft, clean, show, ask, then act |
| `cj-project-context` | Find project and organisation facts at runtime (commands, labels, channels, goals) instead of hard-coding them |
| `example-skill` | Template for new skills, never installed |

## When something becomes a skill

- A **flow** the user starts on purpose (`/cj-...`): its own skill, user-invoked.
- A **discipline** that two or more skills need (grilling, the page format, project facts): its own skill, model-invoked, called with `Call the Skill tool with "<name>"`.
- Everything else is a section of the skill that uses it, or a reference file beside its `SKILL.md`, linked where the step needs it. One user of a thing means no skill for it.

## Invocation

- **User-invoked**: `disable-model-invocation: true`; the description is one line for a human browsing commands. Use it for flows with side effects or long runs.
- **Model-invoked** (default): the description starts with "Use when ..." and lists only triggers.
- No skill calls a user-invoked skill.
- **Building blocks**: model-invoked with `user-invocable: false`, so they stay out of the `/` menu. For skills only other skills call.
- A skill nobody used for 45 days is parked: removed, with a line in the README naming it, restorable from git history.

## Rules when editing

- A skill asks the user one question per turn, as a choice with the recommended option first (the harness's choice tool where it offers one). Never a batch of questions.

- Own skills carry the `cj-` prefix. `name` in the frontmatter must match the folder name (lowercase letters, digits, hyphens).
- A model-invoked skill's `description` starts with "Use when ..." and lists only triggers. It decides whether the skill fires; a workflow summary there makes agents skip the body.
- Text meant for humans goes through the `unslop` skill.
- A skill that shows a summary in chat (decision list, report, ledger) carries this sentence inline at that step, word for word, because `AGENTS.md` is not loaded when the skill runs in another repo: "Write it scannable: one item per line, its core in a few words first; reason, source or path on an indented line below it, or left out when the user does not need it to decide; group under headings that name the topic in the reader's terms, not generic labels like Scope or Process; no item longer than two lines." Tables, code or log blocks and the "Next:" block keep their own format. When a question then refers to the items, the skill also says: number them in the reply text, never only inside the question, and name an item in each option by its number and a few words.
- Write instructions tool-neutrally and reference files relative to `SKILL.md`. Call scripts via `bash scripts/x.sh`.
- Write skills in English.
- No secrets, tokens or absolute paths (`/Users/...`).
- No facts about one organisation or repo: no repo names, channel names or ids, team names, label names, framework commands. A skill names the fact it needs and gets it through `cj-project-context`. Values live in the target repo (`AGENTS.md`, `docs/agents/`) or in the user's own instructions.
- Add new skills to the table above and to the README.

## Checks

```bash
bash scripts/validate.sh     # format of all skills (also runs in CI): invocation rules, no calls to user-invoked skills; locally also no word from the gitignored .org-terms file in any tracked file
git config core.hooksPath .githooks   # once per checkout: runs validate.sh, and cargo fmt --check when the crate changes, before each commit
./install.sh --dry-run       # shows what would be linked
# in skills/cj-whetstone/skill-metrics/ when the crate changes (also in CI):
cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test --locked
cargo test -- --ignored      # manual, not in CI, after a report layout change: writes target/tmp/report-check.html from sample data; look at it as the test's doc comment says
```

Skills are installed as symlinks. Edits here take effect immediately, no reinstall needed.

## Conventions

- Commit: `<area>: <what changed> (#<issue>)`, e.g. `skill-metrics: report version view per skill (#52)`.
- Branch: `feat/<issue>-<slug>` or `fix/<issue>-<slug>`; without an issue, `fix/<slug>`.
- Worktree: a sibling directory of the checkout, `../agent-skills-<issue>-<slug>`.
- Language of issues, PRs, commits and review replies: English.
