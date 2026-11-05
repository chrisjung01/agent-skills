#!/usr/bin/env bash
# Tests for bin/cj-run against a temporary config folder, throwaway git repos and stubs of claude, gh and osascript. No network.
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

# --- real runs against stubs of claude, gh and osascript on the PATH ---
export CJ_RUN_STATE_DIR="$T/state" STUB="$T/stub"
mkdir -p "$T/bin" "$STUB"
cat > "$T/bin/claude" <<'EOS'
#!/usr/bin/env bash
pwd > "$STUB/claude-cwd"
ls "$CJ_RUN_STATE_DIR/running" > "$STUB/markers-during"
[[ -z "${STUB_CHILD:-}" ]] || { sleep 300 & echo $! > "$STUB/child.pid"; }
[[ -z "${STUB_COST:-}" ]] || echo "{\"type\":\"result\",\"total_cost_usd\":$STUB_COST}"
[[ -z "${STUB_SLEEP:-}" ]] || sleep "$STUB_SLEEP"
exit "${STUB_RC:-0}"
EOS
cat > "$T/bin/gh" <<'EOS'
#!/usr/bin/env bash
echo "$*" >> "$STUB/gh.log"
for ((i=1; i<=$#; i++)); do [[ "${!i}" != --body-file ]] || { j=$((i+1)); cp "${!j}" "$STUB/comment-body"; }; done
[[ "$1 $2" == "pr list" ]] || exit 0
[[ -z "${STUB_GH_FAIL:-}" ]] || { echo "$STUB_GH_FAIL" >&2; exit 1; }
limit="$(sed -n 's/.*--limit \([0-9]*\).*/\1/p' <<<"$*")"
head -n "${limit:-30}" "$STUB/prs"
EOS
cat > "$T/bin/osascript" <<'EOS'
#!/usr/bin/env bash
printf '%s\n' "$@" >> "$STUB/osa.log"
EOS
chmod +x "$T/bin"/*
RUN_ARGS=; OLD_PATH="$PATH"; PATH="$T/bin:$PATH"
FRESH=2999-01-01T00:00:00Z; STALE=2000-01-01T00:00:00Z
PR196="https://github.com/acme/Widget/pull/9"
prs() { printf 'agent/196-fix\t%s\t%s\n' "$PR196" "$1" > "$STUB/prs"; }
run() { # run [env...]: one cj-run for issue 196, fresh logs
  rm -rf "$T/state/logs" "$STUB/gh.log" "$STUB/osa.log" "$STUB/child.pid" "$STUB/comment-body"
  OUT="$(env "$@" "$CJ" $URL $RUN_ARGS 2>&1)"; RC=$?
  LOGF="$(ls "$T/state/logs"/* 2>/dev/null | head -n 1)"; LOGT="$(cat "$LOGF" 2>/dev/null)"
}

# AC-6: success needs claude exit 0 and an open agent/196-* PR updated after the start
prs $FRESH; run STUB_COST=0.5
check "AC-6 done exits 0" test $RC -eq 0
check "AC-6 log names the PR" has "$LOGT" "pr: $PR196"
check "AC-6 log says done" has "$LOGT" "result: done"
check "AC-6 claude ran in the worktree" eq "$(cat "$STUB/claude-cwd")" "$T/code/Widget-a"
prs $STALE; run
check "AC-6 PR not updated since start fails" test $RC -ne 0
check "AC-6 stale: failed in log" has "$LOGT" "result: failed"
check "AC-6 stale: no PR link in log" eq "$(count "$LOGT" "$PR196")" 0
printf 'agent/1960-x\t%s\t%s\n' "$PR196" $FRESH > "$STUB/prs"; run
check "AC-6 other branch prefix fails" test $RC -ne 0
prs $FRESH; run STUB_RC=3
check "AC-6 claude non-zero fails" test $RC -ne 0
check "AC-6 claude non-zero: no PR link" eq "$(count "$LOGT" "$PR196")" 0
for i in $(seq 1 40); do printf 'agent/5-x\thttps://x/%s\t%s\n' $i $FRESH; done > "$STUB/prs"; printf 'agent/196-fix\t%s\t%s\n' "$PR196" $FRESH >> "$STUB/prs"; run
check "AC-6 PR beyond the 30th is found" test $RC -eq 0

# AC-7: cost and duration in the log
prs $FRESH; run STUB_COST=0.0123
check "AC-7 cost in log" has "$LOGT" "cost_usd: 0.0123"
check "AC-7 duration in log" grep -qE '^duration_s: [0-9]+$' <<<"$LOGT"
check "AC-7 one log per run" eq "$(ls "$T/state/logs" | wc -l | tr -d ' ')" 1
"$CJ" $URL >/dev/null 2>&1
check "AC-7 second run, second log" eq "$(ls "$T/state/logs" | wc -l | tr -d ' ')" 2

# AC-8: time limit ends the stub and its child; AC-7: cost unknown after a timeout
printf 'minutes=0.03\n' > "$T/cfg/config"
run STUB_COST=1.5 STUB_CHILD=1 STUB_SLEEP=300
check "AC-8 timeout exits non-zero" test $RC -ne 0
check "AC-8 log says timeout" has "$LOGT" "result: timeout"
check "AC-7 cost unknown after timeout" has "$LOGT" "cost_usd: unknown"
check "AC-10 marker gone after timeout" eq "$(ls "$T/state/running" | wc -l | tr -d ' ')" 0
sleep 0.3
check "AC-8 child is gone" bash -c '! kill -0 "$(cat "$1")" 2>/dev/null' _ "$STUB/child.pid"
rm "$T/cfg/config"

# AC-9: only <repo>-196-verify goes, also with uncommitted files
for w in 196-verify 1960-verify 197-verify; do git -C "$MAIN" worktree add -q --detach "$T/code/Widget-$w" HEAD; done
echo dirty > "$T/code/Widget-196-verify/untracked"
prs $FRESH; run
wl="$(git -C "$MAIN" worktree list)"
check "AC-9 own verify worktree removed" bash -c '! grep -q "Widget-196-verify" <<<"$1"' _ "$wl"
check "AC-9 other verify worktrees stay" has "$wl" "Widget-1960-verify"
check "AC-9 issue 197 verify stays" has "$wl" "Widget-197-verify"
check "AC-9 issue worktree stays" has "$wl" "Widget-a"

# AC-10: marker while running, gone after done, failed and timeout
prs $FRESH; run
check "AC-10 marker during run" eq "$(cat "$STUB/markers-during")" "acme__Widget__196"
check "AC-10 marker gone after done" eq "$(ls "$T/state/running" | wc -l | tr -d ' ')" 0
run STUB_RC=1
check "AC-10 marker gone after failed" eq "$(ls "$T/state/running" | wc -l | tr -d ' ')" 0

# AC-11: notification on every end; a double quote in the reason stays intact
prs $FRESH; run
check "AC-11 done: repo, issue and PR link" has "$(cat "$STUB/osa.log")" "acme/Widget#196: done $PR196"
run STUB_GH_FAIL='HTTP 500: "boom"'
check "AC-11 failed: reason with quote" has "$(cat "$STUB/osa.log")" 'gh pr list failed: HTTP 500: "boom"'
printf 'minutes=0.03\n' > "$T/cfg/config"; run STUB_SLEEP=300
check "AC-11 timeout notified" has "$(cat "$STUB/osa.log")" "acme/Widget#196: timeout"
rm "$T/cfg/config"

# AC-12: --report on a failed run labels and comments; without it no write call on the issue
prs $STALE; RUN_ARGS=; run STUB_COST=0.25
check "AC-12 no report: no issue write" bash -c '! grep -qE "^issue (edit|comment)" "$1"' _ "$STUB/gh.log"
RUN_ARGS=--report; run STUB_COST=0.25
check "AC-12 failed run still exits non-zero" test $RC -ne 0
check "AC-12 label set" has "$(cat "$STUB/gh.log")" "issue edit 196 --repo acme/Widget --add-label agent-failed"
check "AC-12 comment posted" has "$(cat "$STUB/gh.log")" "issue comment 196 --repo acme/Widget"
body="$(cat "$STUB/comment-body")"
check "AC-12 comment: duration" grep -qE 'Duration: [0-9]+s' <<<"$body"
check "AC-12 comment: cost" has "$body" "Cost: 0.25 USD"
check "AC-12 comment: log path" has "$body" "$LOGF"
check "AC-12 comment: log tail" has "$body" "result: failed"
prs $FRESH; run STUB_COST=0.25
check "AC-12 done run with --report: no issue write" bash -c '! grep -qE "^issue (edit|comment)" "$1"' _ "$STUB/gh.log"
RUN_ARGS=
PATH="$OLD_PATH"

# AC-13: the seven named commands are denied; no allow rule is also a deny rule
for c in "gh pr merge 5" "git push --force origin x" "git push -f origin x" "git push --delete origin x" "gh api -X DELETE repos/o/r" "gh issue delete 5" "gh repo delete o/r"; do
  check "AC-13 denies: $c" denied "$c"
done
check "AC-13 no allow rule in deny" eq "$(grep -hv -e '^#' -e '^$' "$HERE/config/cj-run/base.allow" | grep -cxFf - "$HERE/config/cj-run/base.deny")" 0

exit $FAIL
