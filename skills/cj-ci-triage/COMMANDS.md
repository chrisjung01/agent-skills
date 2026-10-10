# CI triage commands

Read with `gh`; none of these needs a yes. `<main workflow>` and `<gate job>` come from the `ci` fact.

## Run and PR for each other

```
gh pr view --json number,headRefOid                   # current branch -> PR and head
gh pr checks <n>                                      # PR -> checks with run links
gh run list --branch <branch> --workflow <main workflow> --limit 5 --json databaseId,headSha,conclusion,url
gh api repos/{owner}/{repo}/actions/runs/<id> --jq '{sha:.head_sha,pr:.pull_requests[0].number}'   # run -> PR
```

## Failing jobs and steps, gate job excluded

```
gh run view <id> --json jobs --jq '.jobs[]|select(.conclusion=="failure" and .name!="<gate job>")|{name,id:.databaseId,steps:[.steps[]|select(.conclusion=="failure")|.name]}'
```

Exclude notification-only jobs from the result as well.

## Error lines of one job

```
gh run view <id> --job <jobid> --log-failed | grep -n '##\[error\]'
```

The full log is unstructured (`UNKNOWN STEP` prefixes) and starts with runner boilerplate: grep first, then read around the first `##[error]`.

## Comparison runs

```
gh run list --branch main --workflow <main workflow> --limit 3                                  # unrelated check
gh run list --workflow <main workflow> --status failure --limit 10 --json databaseId,headBranch  # flaky evidence, cap 10
gh pr diff <n> --name-only                                                                      # real-failure overlap
```

## Actions after a yes

```
gh run rerun <id> --failed
gh pr comment <n> ...                                  # why a red check does not block
```
