# Commands

`<owner>` and `<name>` come from `gh repo view --json owner,name`. `<n>` is the PR number.

## Find the PR (phase 0)

```
gh pr view --json number,url,author,headRefName,baseRefName   # PR of the current branch
gh pr list --author @me --state all --limit 10                 # no PR on this branch
gh api user --jq .login
```

## Worktree of the PR branch (phase 0)

```
git worktree list                                              # reuse the one on <headRefName>
git fetch origin <headRefName> <baseRefName>
git worktree add <path> --track -b <headRefName> origin/<headRefName>   # branch exists locally: git worktree add <path> <headRefName>
cd <path> && git pull
```

## Threads and review bodies (phase 0)

Threads with resolved and outdated state, plus `diffHunk` to find moved code:

```
gh api graphql -F n=<n> -f o=<owner> -f r=<name> -f query='query($o:String!,$r:String!,$n:Int!){repository(owner:$o,name:$r){pullRequest(number:$n){reviewThreads(first:100){nodes{id isResolved isOutdated path line originalLine comments(first:50){nodes{author{login} body url diffHunk}}}}}}}'
```

More than 100 threads or 50 comments in one thread: page with `after:` before trusting "last comment".

Review bodies:

```
gh pr view <n> --json reviews --jq '.reviews[]|select(.body!="")|{author:.author.login,state,body}'
```

## Show the diff (phase 3)

```
git diff origin/<baseRefName>...HEAD
```

## Post, in this order (phase 3)

1. Push: `git push`
2. Reply to a thread (body from a file, see cj-posting):
   ```
   gh api graphql -F id=<thread id> -F body=@<file> -f query='mutation($id:ID!,$body:String!){addPullRequestReviewThreadReply(input:{pullRequestReviewThreadId:$id,body:$body}){comment{url}}}'
   ```
   Review-body feedback: `gh pr comment <n> --body-file <file>`
3. Resolve:
   ```
   gh api graphql -F id=<thread id> -f query='mutation($id:ID!){resolveReviewThread(input:{threadId:$id}){thread{isResolved}}}'
   ```
4. Re-request review: `gh pr edit <n> --add-reviewer <login>`
