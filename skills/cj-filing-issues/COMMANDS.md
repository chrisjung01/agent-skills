# Commands

## Find

```bash
gh issue list --repo <owner/repo> --search "<keywords>" --state all --limit 50 --json number,title,state,labels,closedAt
gh pr list --repo <owner/repo> --search "<module or file name>" --state open --json number,title,files
```

Is the fix of a closed match released?

```bash
gh pr view <pr> --json mergeCommit --jq .mergeCommit.oid
git tag --contains <sha>          # earliest release tag per `releases`; compare with the reporter's version, ask when unknown
```

Open epics: `gh issue list --repo <owner/repo> --state open --search "<epic marker from issue-tracker>"`, then `gh api repos/<owner>/<repo>/issues/<epic>/sub_issues --jq '.[].number,.[].title'` to see what an epic already holds.

## File

```bash
gh issue create --repo <owner/repo> --title "<title>" --body-file <draft.md> --label "<label>" [--label ...]
```

Parent and blocked-by, one by one (both work across repos):

```bash
gh issue edit <child> --repo <owner/repo> --parent <epic-url>

# blocked-by needs the blocker's global id, not its number
BLOCKER_ID=$(gh api repos/<owner>/<repo>/issues/<blocker> --jq .id)
gh api --method POST repos/<owner>/<repo>/issues/<child>/dependencies/blocked_by -F issue_id="$BLOCKER_ID"
```

For a batch: `bash scripts/link-issues.sh` (copy it next to the plan, fill in `EPIC`, `TICKETS`, `BLOCKED_BY`).

## Read back

```bash
gh api repos/<owner>/<repo>/issues/<n>/dependencies/blocked_by --jq '.[].number'
gh api repos/<owner>/<repo>/issues/<epic>/sub_issues --jq '.[].number'
```

## Project fields

`bash scripts/set-project-fields.sh` batch-sets single-select fields (status and the like): copy it, fill in `OWNER`, `PROJECT_NUMBER` (the board from `issue-tracker`), the issue list and the field and option IDs. Needs `gh auth refresh -h github.com -s project` once.
