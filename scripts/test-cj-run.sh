#!/usr/bin/env bash
# Tests for bin/cj-run against a temporary config folder and throwaway git repos. No claude, no network.
set -uo pipefail
HERE="$(cd "$(dirname "$0")/.." && pwd)"
CJ="$HERE/bin/cj-run"
T="$(cd "$(mktemp -d)" && pwd -P)"; trap 'rm -rf "$T"' EXIT
export CJ_RUN_CONFIG_DIR="$T/cfg"
FAIL=0
ok()    { echo "ok   $1"; }
fail()  { echo "FAIL $1"; FAIL=1; }
check() { local name="$1"; shift; if "$@"; then ok "$name"; else fail "$name"; fi; }
has()   { grep -qF -- "$2" <<<"$1"; }
line()  { grep -qFx -- "$2" <<<"$1"; }
count() { grep -oF -- "$2" <<<"$1" | wc -l | tr -d ' '; }
eq()    { [[ "$1" == "$2" ]]; }

mkdir -p "$T/cfg" "$T/code" && cd "$T/code" || exit 1
git init -q Widget && git -C Widget -c user.email=t@t -c user.name=t commit -q --allow-empty -m init
MAIN="$T/code/Widget"
printf '# comment\n\nBash(gh issue *)\nRead\n' > "$T/cfg/base.allow"
printf 'Bash(gh pr merge*)\nBash(git push --force*)\n' > "$T/cfg/base.deny"
printf 'Read\nBash(make *)\n' > "$T/cfg/Widget.allow"
printf 'acme/Widget=%s auto\n' "$MAIN" > "$T/cfg/repos"
URL=https://github.com/acme/Widget/issues/196

# AC-1: one worktree on agent/196-* wins
git -C "$MAIN" worktree add -q "$T/code/Widget-196" -b agent/196-fix
out="$("$CJ" --dry-run $URL)"
check "AC-1 worktree named" line "$out" "Working directory: $T/code/Widget-196"

# AC-2: no such worktree -> main checkout (agent/1960-* must not match agent/196-)
git -C "$MAIN" worktree remove --force "$T/code/Widget-196"
git -C "$MAIN" worktree add -q "$T/code/Widget-1960" -b agent/1960-other
out="$("$CJ" --dry-run $URL)"
check "AC-2 main checkout named" line "$out" "Working directory: $MAIN"

# AC-3: unknown repo, bad URL, two matching worktrees
out="$("$CJ" --dry-run https://github.com/acme/Other/issues/1 2>&1)"; rc=$?
check "AC-3 unknown repo exits non-zero" test $rc -ne 0
check "AC-3 unknown repo reason" has "$out" "not in"
"$CJ" --dry-run https://github.com/acme/Widget/pull/5 >/dev/null 2>&1; rc=$?
check "AC-3 not an issue URL" test $rc -ne 0
git -C "$MAIN" worktree add -q "$T/code/Widget-a" -b agent/196-a
git -C "$MAIN" worktree add -q "$T/code/Widget-b" -b agent/196-b
out="$("$CJ" --dry-run $URL 2>&1)"; rc=$?
check "AC-3 two worktrees exits non-zero" test $rc -ne 0
check "AC-3 two worktrees reason" has "$out" "more than one worktree"
git -C "$MAIN" worktree remove --force "$T/code/Widget-b"

# AC-4: rules of both allow files once, deny under --disallowedTools, add-dir, write rule
out="$("$CJ" --dry-run $URL)"
check "AC-4 shared rule once" eq "$(count "$out" "'Read'")" 1
check "AC-4 base rule once" eq "$(count "$out" "'Bash(gh issue *)'")" 1
check "AC-4 repo rule once" eq "$(count "$out" "'Bash(make *)'")" 1
check "AC-4 comment is no rule" eq "$(count "$out" '# comment')" 0
check "AC-4 deny after flag" has "$out" "'--disallowedTools' 'Bash(gh pr merge*)' 'Bash(git push --force*)'"
check "AC-4 add-dir parent" has "$out" "'--add-dir' '$T/code'"
check "AC-4 write rule" has "$out" "'Edit(/$T/code/Widget-*/**)'"

# AC-5: defaults without config, values from config
check "AC-5 default budget" has "$out" "'--max-budget-usd' '20'"
check "AC-5 default minutes" has "$out" "45 minutes"
printf 'budget_usd=7\nminutes=12\n' > "$T/cfg/config"
out="$("$CJ" --dry-run $URL)"
check "AC-5 configured budget" has "$out" "'--max-budget-usd' '7'"
check "AC-5 configured minutes" has "$out" "12 minutes"

# the shipped templates work as a config
mkdir -p "$T/cfg2" && cp "$HERE"/config/cj-run/{base.allow,base.deny,config} "$T/cfg2/"
printf 'acme/Widget=%s\n' "$MAIN" > "$T/cfg2/repos"
out="$(CJ_RUN_CONFIG_DIR="$T/cfg2" "$CJ" --dry-run $URL)"
while IFS= read -r r; do
  check "templates: $r once" eq "$(count "$out" "'$r'")" 1
done <<<"$(grep -hv -e '^#' -e '^$' "$HERE/config/cj-run/base.allow" "$HERE/config/cj-run/base.deny")"
# shipped deny list vs concrete command lines (rule glob == command)
denied() { local r; while IFS= read -r r; do r="${r#Bash(}"; r="${r%)}"; [[ "$1" == $r ]] && return 0; done <<<"$(grep '^Bash(' "$HERE/config/cj-run/base.deny")"; return 1; }
for c in "gh api repos/o/r/pulls/1/merge -X PUT" "gh api graphql -f query=mutation{mergePullRequest}" "git push -uf origin x" "git push origin :b" "git push --mirror" "git branch --delete --force x" "git branch -d x"; do
  check "template denies: $c" denied "$c"
done

# review fixes: "auto" in a path, regex characters in the URL, apostrophe in a rule, bad config value, gone worktree
git init -q "$T/code/auto" && git -C "$T/code/auto" -c user.email=t@t -c user.name=t commit -q --allow-empty -m init
printf 'acme/Widget=%s\nacme/a.b=%s\n' "$T/code/auto" "$MAIN" > "$T/cfg/repos"
out="$("$CJ" --dry-run $URL)"
check "path ending in auto kept" line "$out" "Working directory: $T/code/auto"
printf 'acme/Widget=%s auto\n' "$T/code/auto" > "$T/cfg/repos"
out="$("$CJ" --dry-run $URL)"
check "auto marker stripped" line "$out" "Working directory: $T/code/auto"
printf 'acme/a.b=%s\n' "$MAIN" > "$T/cfg/repos"
"$CJ" --dry-run https://github.com/acme/aXb/issues/1 >/dev/null 2>&1; rc=$?
check "dot in repo name is no wildcard" test $rc -ne 0
printf 'acme/Widget=%s\n' "$MAIN" > "$T/cfg/repos"
printf "Bash(echo 'x'*)\n" >> "$T/cfg/base.allow"
out="$("$CJ" --dry-run $URL)"
check "apostrophe quoted" has "$out" "'Bash(echo '\\''x'\\''*)'"
printf 'budget_usd=abc\n' > "$T/cfg/config"
"$CJ" --dry-run $URL >/dev/null 2>&1; rc=$?
check "non-numeric budget refused" test $rc -ne 0
printf 'acme/Widget=%s\n' "$MAIN" > "$T/cfg/repos"; rm "$T/cfg/config"
git -C "$MAIN" worktree add -q "$T/code/Widget-197" -b agent/197-x && rm -rf "$T/code/Widget-197"
out="$("$CJ" --dry-run https://github.com/acme/Widget/issues/197)"
check "gone worktree ignored" line "$out" "Working directory: $MAIN"

exit $FAIL
