# Routing

Findings that belong elsewhere leave the skill unchanged and go here:

| Finding | Goes to |
|---|---|
| Project-specific convention | That repo's AGENTS.md / CLAUDE.md, or memory |
| Personal preference across all work | Memory or `~/.claude/CLAUDE.md`, suggested, not written |
| Correction regex misreads a phrase | Proposed regex fix in `skill-metrics/src/user_text.rs` |
| Skill not in this repo (plugin, other repo) | Report only, with a suggested upstream change |
| Skill removed from this repo | Its successor's ledger (named in the removal commit), else `verworfen` with finding `skill removed` |
| Environment or harness behaviour (a tool the agent called twice, a permission prompt) | Report it as an environment finding; the skill changes only when its own text caused it |
