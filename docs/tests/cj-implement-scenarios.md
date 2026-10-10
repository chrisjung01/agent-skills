# Scenarios: cj-implement

Paper exercises built from real sessions. The agent under test gets the skill text and the tool output below and describes its next commands; it runs nothing. The flow-wide scenarios are in [spec-driven-flow-scenarios.md](spec-driven-flow-scenarios.md).

## 1. Resume: the ticket's branch link is gone, its PR is open

**Setup:** spec #18, tickets #26 → #22 → #23 (blocked-by chain), mode `per-ticket`. An earlier run created `feat/26-skill-metrics-skeleton` with `gh issue develop 26` and opened draft PR #28 (`Closes #26`, CI green). Since then GitHub dropped the branch link under "Development". Tool output in this run:

```
$ for n in 26 22 23; do echo "#$n: $(gh issue develop --list $n)"; done
#26:
#22:
#23:
```

**Prompt:** "/cj-implement #18 per-ticket". The agent names only the commands step 2 asks for and says what each returns in this setup.

**Expected:** #26 counts as done through PR #28, found by the skill's own lookup and not by luck; no second `gh issue develop 26`. The next unit is #22, stacked on PR #28 (base `feat/26-skill-metrics-skeleton`).

**Source:** sessions `765a26e3`, `27597af8`, `557480b2`, `40075e28` (2026-10-08): `gh issue develop --list 26` was empty in all four while PR #28 was open; each agent found the PR only through an extra `gh pr list` the skill did not ask for.

## Results

- 2026-10-08, scenario 1, two runs each. Without the change: both found no branch to pass to `gh pr list --head`, called #26 ready and named a second `gh issue develop 26`. With it: both found PR #28 through `closingIssuesReferences`, marked #26 done and took #22 as the unit, stacked on `feat/26-skill-metrics-skeleton`.
