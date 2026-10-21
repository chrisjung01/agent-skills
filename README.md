# agent-skills

My personal agent skills for **Claude Code** and **OpenAI Codex**, in the open [Agent Skills](https://agentskills.io/specification) format: one folder per skill with a `SKILL.md`.

Every skill from this repo carries the `cj-` prefix (some are adapted, see [Sources](#sources)), so it is obvious in a skill list which ones come from here and which come from plugins or other repos.

This is a personal repo. The skills follow how I work and change when that changes. Issues and ideas are welcome. For a pull request, open an issue first so we can talk it through briefly.

## The spec-driven flow

```
/cj-grill-with-docs <issue>  →  /cj-to-spec <issue>                          →  /cj-implement <issue>  →  cj-reviewing  →  /cj-accept <issue>  →  /cj-retro
  tier, challenge, decisions     "## Spec" comment, sub-issues (optional)         small PR per ticket  (inside implement)   "## Acceptance" by hand   environment
```

- The issue stays as its author wrote it. The spec is a comment starting with `## Spec`; tickets are its sub-issues; technical deviations are comments below it.
- Steps 1 and 2 run in one session; `cj-to-spec` slices a spec that needs more than one ticket in the same run. `cj-implement` starts fresh after `/clear` and can be resumed: linked branches and open PRs are its state.
- One ticket, one branch, one small PR (draft, unless small and auto-merge is allowed), stacked on its blocker's PR while that is still open. `/cj-implement <issue> one-pr` puts the whole spec on one branch and PR instead.
- `cj-accept` runs at most once per spec, on its last open PR before it merges, when the spec's Acceptance line says by hand: the user checks every row by hand, because green tests can still hide a broken feature. A failed row is fixed in that PR through `cj-implement` or becomes a follow-up issue.
- `cj-retro` improves the repo the agent worked in; `cj-whetstone` improves the skills from the skill calls its metrics binary stores.
- `cj-grill-with-docs` opens by proposing the tier. In the big tier it first runs the challenge: whether it should be built at all, blocking what has no documented goal, no evidence or a smaller lever.

The chain above is the big tier. A change runs only the steps its tier names.

### Tiers by stakes

Pick the tier before the first command, from the stakes the issue describes; `/cj-grill-with-docs` proposes it as its first question.

- **Small**: a typo or copy change, a fix that restores behaviour the issue states as intended (length alone does not disqualify it), or a refactor whose issue states its scope and unchanged behaviour, none with an open product decision. Only `/cj-implement <issue>`, which takes the issue as spec. No grill, no spec comment, no acceptance. Anything else is at least normal. Real effort and more than one ticket win over small. Money, data, security or an irreversible step beats small unless the user confirmed small: `/cj-implement` asks once and posts a small answer on the issue as `Tier: small, confirmed`.
- **Normal**: a feature that fits one ticket, including a new rule the issue calls a fix. `/cj-grill-with-docs`, `/cj-to-spec`, `/cj-implement`. The grill asks no challenge question unless money, data, security or an irreversible step turns up while grilling. `/cj-accept` when the spec's Acceptance line says by hand. A spec that turns out to need more than one ticket moves to the big tier: `/cj-to-spec` slices it after posting.
- **Big or risky**: real effort, more than one ticket, money, data, security or an irreversible step. The whole chain: `/cj-grill-with-docs` runs the challenge first, `/cj-to-spec` with its slice into sub-issues, `/cj-accept` before the last merge when the spec's Acceptance line says by hand. An item a stakeholder already decided with a documented goal link gets the short challenge (steps 1, 3 and 5).
- **Any tier**: `/cj-retro` after a session with friction.

The last `Acceptance:` line of every spec, `Acceptance: by hand` or `Acceptance: verified is enough`, decides acceptance in every tier; `cj-to-spec` proposes it from the triggers in [SPEC.md](skills/cj-to-spec/SPEC.md#acceptance-line). With verified is enough, `/cj-accept` only writes the kill issue and deviations, when there are any.

## Skills

**Flow and tools you start** (`/name`; their descriptions cost no context):

| Skill | What it does |
|---|---|
| [`cj-grill-with-docs`](skills/cj-grill-with-docs/SKILL.md) | **Flow 1.** Proposes the tier first. In the big tier it runs the challenge: a sceptical project lead checks goal link, documented evidence, the smaller lever and a pre-mortem, blocks on gaps and posts a verdict with a kill criterion. Then grills the decisions one question at a time; glossary terms and ADRs are written as they settle; questions only the issue author can answer go into one comment. |
| [`cj-to-spec`](skills/cj-to-spec/SKILL.md) | **Flow 2.** Turns the grilled conversation into a spec (AC-1… as Given/When/Then, edge cases, test seams, out of scope), has a fresh reader look for criteria a wrong implementation would pass, and posts it as a `## Spec` comment, or as a new issue. Slices a spec that needs more than one ticket into small vertical sub-issues with blocked-by edges. |
| [`cj-implement`](skills/cj-implement/SKILL.md) | **Flow 4.** Implements the spec's tickets as small PRs (draft unless auto-merge applies), one per ticket, stacked while a blocker is unmerged: targeted tests, full checks once per PR, independent review, a PR body of five blocks in at most 15 lines with one Verified line per AC. Each logic test is shown red by one inverted condition; a fresh subagent writes a bug's reproduction test. `one-pr` for one PR per spec. Resumable after `/clear`. |
| [`cj-accept`](skills/cj-accept/SKILL.md) | **Flow 5.** The user checks every acceptance criterion and edge case of the spec by hand on its last open PR, before it merges. Failed rows become a fix in that PR or a follow-up issue; the result is an `## Acceptance` comment on the spec's issue. Once nothing is left to fix, it files the kill-criterion issue and turns deviations into ADRs or glossary terms. A spec that says verified is enough gets only these records. |
| [`cj-retro`](skills/cj-retro/SKILL.md) | **Flow 6.** Turns a session's friction into checks, standards or agent-doc pointers in the worked-in repo; skill findings go to `cj-whetstone`. |
| [`cj-whetstone`](skills/cj-whetstone/SKILL.md) | Improves a skill from the skill calls the whetstone metrics store, after reading their windows. |
| [`cj-architecture-scan`](skills/cj-architecture-scan/SKILL.md) | You suspect the architecture of a repo hurts but cannot name a decision yet. Scans the hot spots (or the area you name), shows up to five refactoring candidates as cards on a page, and files the one you pick as an issue for `/cj-grill-with-docs`. |
| [`cj-teach`](skills/cj-teach/SKILL.md) | Teaches a topic over several sessions: mission, sources, short HTML lessons with quizzes, reference sheets and learning records in the current directory. Copied from mattpocock/skills. |

**Day to day** (the model picks them up from your words, or you type `/name`):

| Skill | Use it when |
|---|---|
| [`cj-bug-triage`](skills/cj-bug-triage/SKILL.md) | An error-tracker issue or a human bug report (chat, screenshot, support ticket) needs a verdict: real bug, noise, expected behaviour, UX finding or duplicate. Drafts the issue and the chat reply. |
| [`cj-ci-triage`](skills/cj-ci-triage/SKILL.md) | A CI run is red. Real failure, flaky, infrastructure or unrelated, before anyone clicks rerun. |
| [`cj-review-loop`](skills/cj-review-loop/SKILL.md) | Your own branch needs a review: one pass, or rounds of review and fix until no high or medium findings remain. |
| [`cj-review-teammate-pr`](skills/cj-review-teammate-pr/SKILL.md) | You review a colleague's PR against its issue and the team agreements. Drafts the comments, never approves for you. |
| [`cj-address-pr-feedback`](skills/cj-address-pr-feedback/SKILL.md) | Your own PR has open review threads. Verifies each claim, shows a ledger before touching code, replies on every thread. |
| [`cj-design-options`](skills/cj-design-options/SKILL.md) | An architecture decision is due: where code or data belongs, how parts talk, a technology with lock-in, a new abstraction. Finds the forces in your code, checks two or three candidates, explains them with change scenarios and recommends one; you decide. |
| [`cj-filing-issues`](skills/cj-filing-issues/SKILL.md) | An issue is about to be created, or you want to know whether it already exists. Finds duplicates and dependencies, writes the body by kind, wires sub-issues and blocked-by. |

**Building blocks** (called by the skills above; hidden from the `/` menu):

| Skill | Role |
|---|---|
| [`cj-asking`](skills/cj-asking/SKILL.md) | How every skill asks you: plain reply text, one question per message, numbered options for every question, `1) Yes` `2) No` for a confirm. |
| [`cj-grilling`](skills/cj-grilling/SKILL.md) | One question at a time, in the shape `cj-asking` gives, the recommended option first; keeps the domain language precise and writes the glossary and ADRs. |
| [`cj-reviewing`](skills/cj-reviewing/SKILL.md) | The reviewer brief (CORRECTNESS, STANDARDS, SPEC) and the one severity scale. |
| [`cj-posting`](skills/cj-posting/SKILL.md) | The one gate for anything others see: draft, clean with `unslop`, show, ask, then act. |
| [`cj-lint-from-findings`](skills/cj-lint-from-findings/SKILL.md) | A review found something a linter could catch: drafts the rule, proves it fires on the faulty code and stays quiet on the fix, then asks where it goes. |
| [`cj-static-analysis`](skills/cj-static-analysis/SKILL.md) | Who calls a symbol, which symbols a diff leaves orphaned, and which findings the linter already blocks. Asks a code index, then the analyzer, then `grep`, and names the source. |
| [`cj-project-context`](skills/cj-project-context/SKILL.md) | Finds project and organisation facts at runtime instead of hard-coding them. |
| [`cj-html-report`](skills/cj-html-report/SKILL.md) | Renders a result as a visual page next to the chat summary. |

Parked on 2026-10-07 (no use in 45 days; restore from git history when needed): `cj-feature-check`, `cj-documenting-knowledge`, `cj-architecture-review` (succeeded by `cj-architecture-scan`, text not in history).

Removed on 2026-10-16: `cj-challenge`. Its successor is `cj-grill-with-docs`, which runs the challenge with [`CHALLENGE.md`](skills/cj-grill-with-docs/CHALLENGE.md) in the big tier.

Removed on 2026-10-15: `cj-to-tickets`. Its successor is `cj-to-spec`, which slices a spec that needs more than one ticket with [`TICKETS.md`](skills/cj-to-spec/TICKETS.md).

No skill contains facts about a specific organisation or repo. They ask `cj-project-context` for what they need (labels, channels, commands, goals), which reads it from the repo and your instructions or asks you once. To set up a repo, let the first run record its answers in `docs/agents/facts.md`, or point your own `CLAUDE.md` at a personal facts file.

## Rules all skills follow

- **Text for humans goes through `unslop`.** Anything a person reads is cleaned of AI tells before it is shown. `unslop` is not part of this repo, so install it separately.
- **Tool-neutral wording.** Skills describe what to do, not which Claude or Codex tool to call, so both agents can use them.

## Sources

Each skill that takes something from a source names the files and what it took in its own `CREDITS.md`.

- [mattpocock/skills](https://github.com/mattpocock/skills): the spec-driven flow (grill, spec, tickets, implement, retro), the split into user- and model-invoked skills, and shared building blocks called through the Skill tool. `cj-teach` is copied unchanged apart from its name. `cj-design-options` takes the deletion test, the one-adapter rule, the dependency categories and parallel candidate designs from `codebase-design`. `cj-architecture-scan` is adapted from `improve-codebase-architecture`: the scan, the candidate cards and the closing recommendation; the pick becomes an issue instead of an inline grill.
- [addyosmani/agent-skills](https://github.com/addyosmani/agent-skills): single checks and review rules.
- [superpowers](https://github.com/obra/superpowers): testing a skill change with scenarios that fail before the edit and pass after it (`docs/tests/`), and the `writing-skills` method `cj-whetstone` uses for its edits.
- [unslop](https://github.com/backnotprop/pstack/tree/main/skills/unslop) from pstack: the writing rules every text for humans goes through. Install it separately.
- [Agent Skills specification](https://agentskills.io/specification): the folder format, one `SKILL.md` per skill with `name` and `description` in its frontmatter.

## Installation

```bash
git clone git@github.com:chrisjung01/agent-skills.git ~/code/agent-skills
cd ~/code/agent-skills
./install.sh --dry-run   # see what would happen
./install.sh             # link all skills globally
```

`install.sh` symlinks each skill into both tool directories:

| Tool | Global | Per project (`--project <path>`) |
|---|---|---|
| Claude Code | `~/.claude/skills/` | `<path>/.claude/skills/` |
| Codex and other agents | `~/.agents/skills/` | `<path>/.agents/skills/` |

Because they are symlinks, `git pull` updates every installed skill immediately. Run `./install.sh` again after adding or renaming a skill, and after a pull that changes `skills/cj-whetstone/skill-metrics`.

With `cargo` installed, `install.sh` also builds the cj-whetstone metrics binary and prints one hook line. Add it to `~/.claude/settings.json` yourself; the script never edits it. The SessionEnd hook stores each ending session's skill calls in the background. If `settings.json` still runs the removed `bare-question-hook.sh`, `install.sh` says so; remove that `PreToolUse` entry.

For Codex, `install.sh` adds the SessionEnd hook itself, to `$CODEX_HOME/hooks.json` (default `~/.codex/hooks.json`). It appends one entry, leaves other hooks alone, copies an existing file to `hooks.json.bak-<number>` before changing it, and refuses to touch a file it cannot parse. Running it again adds nothing; `--dry-run` only reports; `--uninstall` removes that one entry. With `--project`, the global hook stays as it is. Codex runs a new hook only after you trust it: start `codex`, which reports hooks that need review, and approve it under `/hooks`. Codex trusts the exact command, so running `install.sh` from another checkout or worktree registers a new command that needs approval again. The hook fires when Codex closes the session, which can be 40 to 60 seconds after `/exit`, so a session can show up in Whetstone a minute late. If hooks are off in Codex, the script says so. Without the hook, the full scan of `/cj-whetstone` still collects Codex sessions.

Moving from the old Python hook, once:

0. Update this checkout (`git pull`) and run `./install.sh`. `~/.claude/skills/cj-whetstone` links to this checkout, so both `skill-metrics.sh` and the hook run its code. An outdated checkout builds an old binary that doesn't know `import-notes`.
1. Replace its SessionEnd line in `~/.claude/settings.json` (the one running `~/.claude/whetstone/hooks/collect_notes.py`) with the new line, then delete that file.
2. Run `bash skills/cj-whetstone/scripts/skill-metrics.sh` once, a full scan of all transcripts. The import only matches sessions already in the database.
3. Run `bash skills/cj-whetstone/scripts/skill-metrics.sh import-notes` and check its output: unmatched entries, forks, and the backup it names. It runs once; with many unmatched entries, restore that backup before trying again.
4. Then delete `~/.claude/whetstone/notes/`. The import is the only copy of its statuses.

### Options

| Option | What it does |
|---|---|
| `--dry-run` | Prints every action without changing anything. Combine it with any other option. |
| `--project <path>` | Installs into a single project instead of your home directory. |
| `--skill <name>` | Installs or uninstalls only this one skill. |
| `--force` | If a folder or foreign symlink already sits at the target, moves it to `<dir>.bak-<timestamp>/` and links the repo version instead. Without `--force` the script only warns. |
| `--uninstall` | Removes only symlinks that point into this repo. Anything else stays. |

Each run also deletes dead symlinks that point into this repo, for example after a skill was renamed. `example-skill` is a template and never gets installed.

Backups go next to the skill folder, not inside it (`~/.claude/skills.bak-…/`), so the agent does not load a skill twice.

## Adding a skill

1. Copy the template: `cp -r skills/example-skill skills/cj-my-skill`
2. In `SKILL.md`, set `name: cj-my-skill`. It must match the folder name and may only contain lowercase letters, digits and hyphens.
3. Write the `description` as "Use when ..." and list only the triggers. If it summarises the workflow, agents tend to follow the description and skip the body.
4. Put long reference material in `references/` and helper scripts in `scripts/`, and link them relative to `SKILL.md`. Call scripts through their interpreter (`bash scripts/x.sh`), because executable bits can get lost.
5. Add the skill to the table above and to [`AGENTS.md`](AGENTS.md).
6. Check and install:

   ```bash
   bash scripts/validate.sh
   ./install.sh --skill cj-my-skill
   ```

## Repository layout

```
skills/
└── <skill-name>/
    ├── SKILL.md        # frontmatter (name, description), then instructions
    ├── CREDITS.md      # only in adapted skills: upstream skill, commit, licence
    ├── references/     # optional: details, examples
    └── scripts/        # optional: helper scripts
AGENTS.md               # skill overview and editing rules for agents
LICENSE                 # MIT
install.sh              # links skills for Claude Code and Codex
scripts/validate.sh     # format check, also runs in GitHub Actions
.githooks/pre-commit    # runs validate.sh before each commit
```

## Validation

`scripts/validate.sh` checks every skill for a `SKILL.md`, frontmatter with `name` and `description`, and a `name` that matches the folder. The GitHub Action [`validate-skills`](.github/workflows/validate.yml) runs it on every push and pull request.

It also scans every tracked file for the words in `.org-terms`, a local, gitignored list with one term per line. Two pieces of fixed text don't count as a hit: the MIT licence's warranty line and GitHub personal access tokens, such as the ones in the redaction tests. Without that file the scan is skipped with a notice, so CI checks only the format. To run the check before each commit, set up the hook once per checkout:

```bash
git config core.hooksPath .githooks
```

## Do not commit

Secrets, tokens, or absolute paths like `/Users/...`.
