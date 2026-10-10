# Commands

`<owner>` and `<name>` come from `gh repo view --json owner,name`. `<n>` is the PR number.

## PR metadata (phase 0)

```
gh pr view <n> --json state,isDraft,author,title,body,baseRefName,headRefName,headRefOid,labels,files,reviews,reviewDecision,statusCheckRollup,closingIssuesReferences
gh api user --jq .login
```

## Existing review threads (phase 0)

`reviews` and `comments` in `gh pr view` do not contain inline threads; this does, with resolved state:

```
gh api graphql -f query='query($o:String!,$r:String!,$n:Int!){repository(owner:$o,name:$r){pullRequest(number:$n){reviewThreads(first:100){nodes{isResolved path line comments(first:10){nodes{author{login} body}}}}}}}' -F n=<n> -f o=<owner> -f r=<name>
```

## Diff and file context (phase 0)

```
gh pr diff <n>
gh api "repos/<owner>/<name>/contents/<path>?ref=<headRefName>" --jq .content | base64 -d
```

Keep the quotes: `?` is a glob in zsh.

## Detached worktree for local checks (phase 2)

Leaves the user's working tree alone and creates no branch:

```
git fetch origin pull/<n>/head && git worktree add --detach <scratchpad>/pr-<n> FETCH_HEAD && cd <scratchpad>/pr-<n>
# run checks
git worktree remove --force <scratchpad>/pr-<n>
```

## Posting (phase 4)

Review with the summary as body:

```
gh pr review <n> --comment --body-file <file>
```

Inline comment per finding, anchored to the reviewed commit:

```
gh api repos/<owner>/<name>/pulls/<n>/comments -F body=@<file> -f path='<file>' -F line=<line> -f side=RIGHT -f commit_id=<headRefOid>
```

## Re-review delta

```
git diff <old oid> <new oid>   # in the worktree, or gh pr diff <n> again
```
