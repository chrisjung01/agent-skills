# AGENTS.md

This repo holds personal agent skills for Claude Code and Codex. Each skill is a folder under `skills/` with a `SKILL.md`. See [README.md](README.md) for installation and details.
Domain terms (skill call, window, nested skill, …) are defined in [GLOSSARY.md](GLOSSARY.md).

## Skills

| Skill | Purpose |
|---|---|
| `cj-grill-with-docs` | Flow 1: propose the tier; in the big tier first challenge whether to build at all, blocking on a missing goal link, evidence or a smaller lever; then grill an issue or idea until every decision is made, glossary and ADRs inline |
| `cj-to-spec` | Flow 2: post the spec as a `## Spec` comment on the issue (or a new issue); slice a multi-ticket spec into sub-issues with blocked-by edges |
| `cj-implement` | Flow 4: one small reviewed PR per ticket (draft, unless small and auto-merge is allowed), stacked while a blocker is open; `one-pr` mode for one PR per spec |
| `cj-accept` | Flow 5: the user checks every AC and edge case of a spec by hand on its last open PR; posts an `## Acceptance` comment, files the kill-criterion issue, records deviations as ADR or glossary; only the records when the spec says verified is enough |
| `cj-retro` | Flow 6: propose changes to the worked-in repo's environment after a session |
| `cj-teach` | Teach a topic over several sessions in a workspace of lessons, references and learning records |
| `cj-bug-triage` | Verdict and issue for an error-tracker issue or a human bug report |
| `cj-review-loop` | Review your own branch: one pass, or fix in rounds until no high or medium findings remain |
| `cj-reviewing` | The reviewer brief (CORRECTNESS, STANDARDS, SPEC) and the one severity scale every review uses |
| `cj-review-teammate-pr` | Review a colleague's PR against issue and agreements |
| `cj-address-pr-feedback` | Work through review threads on your own PR |
| `cj-ci-triage` | Classify a red CI run: real, flaky, infrastructure, unrelated |
| `cj-whetstone` | Improve a skill from the skill calls the whetstone metrics store, after reading their windows |
| `cj-asking` | Ask the user as plain reply text: one question per message, numbered options for every question, `1) Yes` `2) No` for a confirm |
| `cj-grilling` | Interview the user one question at a time until every decision is made; sharpen domain terms, write glossary and ADRs |
| `cj-html-report` | Render a result as a visual page (cards, diagrams) next to the chat summary |
| `cj-design-options` | Help the user make an architecture decision: forces from the code, two or three checked candidates, a recommendation with its flip condition; the user decides |
| `cj-filing-issues` | Find duplicates and dependencies, then create issues with good bodies and wire parent and blocked-by edges |
| `cj-posting` | The one gate for anything others see or that changes shared state: draft, clean, show, ask, then act |
| `cj-lint-from-findings` | Turn a mechanical review finding into a lint rule, pattern rule or grep check, proven to fire, placed where the user picks |
| `cj-static-analysis` | Who calls a symbol, which symbols a diff orphans, which findings the linter already blocks; code index, then analyzer, then `grep`, each answer naming its source |
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

- A skill asks the user one question per message, as plain reply text in the shape `cj-asking` gives, the recommended option first. Never a batch of questions.

- Own skills carry the `cj-` prefix. `name` in the frontmatter must match the folder name (lowercase letters, digits, hyphens).
- A model-invoked skill's `description` starts with "Use when ..." and lists only triggers. It decides whether the skill fires; a workflow summary there makes agents skip the body.
- Text meant for humans goes through the `unslop` skill.
- A skill that writes text a person reads in chat (summary, report, question, `Next:` block) carries this rule block inline at that step, word for word, because `AGENTS.md` is not loaded when the skill runs in another repo: "Write it scannable: the first line is the result or the decision needed. Be extremely concise. Sacrifice grammar for the sake of concision. Text others read keeps full sentences. One sentence per bullet, at most 25 words. A list has 3 to 5 items, never more than 7; a longer one splits under two headings or goes onto a page. Three or more items go one per line, never inside a sentence. No nested lists, and no tables unless this skill names one; code, log and command blocks and the `Next:` block keep their own shape. Bold only as a label at the start of a line, code font for paths and commands. Headings name the topic in the reader's terms, not generic labels like Scope or Process. No made-up abbreviations: write glossary terms out unless the glossary defines the abbreviation as the term; code identifiers, paths and issue or PR numbers stay. Before the first question to the user, call the Skill tool with `cj-asking` and ask as it says." `validate.sh` fails when a copy drifts, when a skill asks a choice question, writes a `Next:` block or a chat summary without the block, or when a skill other than `cj-asking` names the picker; `cj-teach`, an upstream copy kept unchanged, is exempt.
- Write instructions tool-neutrally and reference files relative to `SKILL.md`. Call scripts via `bash scripts/x.sh`.
- Write skills in English.
- No secrets, tokens or absolute paths (`/Users/...`).
- No facts about one organisation or repo: no repo names, channel names or ids, team names, label names, framework commands. A skill names the fact it needs and gets it through `cj-project-context`. Values live in the target repo (`AGENTS.md`, `docs/agents/`) or in the user's own instructions.
- Add new skills to the table above and to the README.

## Checks

```bash
bash scripts/validate.sh     # format of all skills (also runs in CI): invocation rules, no calls to user-invoked skills, copies of the rule block match AGENTS.md and every skill that writes chat text carries it, no skill names the picker, PR-BODY.md holds its five blocks in order, a description has at most 1024 characters and a body at most 500 lines, warns when all descriptions pass 6000 characters; locally also no word from the gitignored .org-terms file in any tracked file
git config core.hooksPath .githooks   # once per checkout: runs validate.sh, and cargo fmt --check when the crate changes, before each commit
# proving a new validate.sh check fires: commit first, then break a file, git add it (the checks read the index), run, git reset --hard; reverting before the commit loses the edit
./install.sh --dry-run       # shows what would be linked
# trying install.sh under a temporary HOME: also set RUSTUP_HOME and CARGO_HOME to the real ones, or cargo fails
# in skills/cj-whetstone/skill-metrics/ when the crate changes (also in CI):
cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test --locked
cargo test -- --ignored      # manual, not in CI, after a report layout change: writes target/tmp/report-check.html from sample data; look at it as the test's doc comment says
```

Skills are installed as symlinks. Edits here take effect immediately, no reinstall needed.
A skill change is tried with the scenarios in `docs/tests/` (rules in its README): a fresh agent runs `<skill>-scenarios.md` against the text before and after the change.

## Conventions

- Commit: `<area>: <what changed> (#<issue>)`, e.g. `skill-metrics: report version view per skill (#52)`.
- Commit messages end with their last content line: no `Co-Authored-By` or other attribution trailer, whatever the harness suggests.
- Branch: `feat/<issue>-<slug>` or `fix/<issue>-<slug>`; without an issue, `fix/<slug>`.
- Worktree: a sibling directory of the checkout, `../agent-skills-<issue>-<slug>`.
- Language of issues, PRs, commits and review replies: English.
