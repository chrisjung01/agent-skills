# Reports

Use the escalation report when a finding is still open after 3 attempts, the final report for every other stop.

## Escalation report

```
Review loop stopped: <id> unresolved after 3 attempts

Finding: <section>, <severity>, <title> (<location>)
Problem per reviewer: <...>

Attempt 1: <what changed> -> <why reviewer or checks did not accept it>
Attempt 2: ...
Attempt 3: ...

Suspected cause: <conflicting goals, missing info, a finding that contradicts another>
Decision needed: <one concrete question for the user>
```

## Final report

```
Review loop: <done | stopped (reason)> after <n> rounds
Checks: <commands> -> <green/red>

| ID | Section | Sev | Location | Title | Status |
|----|---------|-----|----------|-------|--------|
| F1 | SPEC | high | spec: criterion 2 | ... | fixed (round 1) |
| F2 | CORRECTNESS | medium | b.ts:40 | ... | rejected: <reason> |
| F3 | STANDARDS | low | c.ts:7 | ... | open (low) |
```
