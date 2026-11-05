#!/usr/bin/env bash
# Tests for bin/cj-poll against a temporary config and state folder, a gh stub that answers from fixture files
# (filtered through the real jq) and a cj-run stub. No network.
set -uo pipefail
HERE="$(cd "$(dirname "$0")/.." && pwd)"
CP="$HERE/bin/cj-poll"
command -v jq >/dev/null || { echo "skip: jq is not installed"; exit 0; }
T="$(cd "$(mktemp -d)" && pwd -P)"; trap 'kill $(cat "$T/stub/run.pid" 2>/dev/null) 2>/dev/null; rm -rf "$T"' EXIT
export CJ_RUN_CONFIG_DIR="$T/cfg" CJ_RUN_STATE_DIR="$T/state" STUB="$T/stub" CJ_POLL_RUN="$T/bin/cj-run"
FAIL=0
ok()    { echo "ok   $1"; }
fail()  { echo "FAIL $1"; FAIL=1; }
check() { local name="$1"; shift; if "$@"; then ok "$name"; else fail "$name"; fi; }
has()   { grep -qF -- "$2" <<<"$1"; }
hasnt() { ! grep -qF -- "$2" <<<"$1"; }
eq()    { [[ "$1" == "$2" ]]; }
n_of()  { grep -cF -- "$2" <<<"$1" || true; }

mkdir -p "$T/cfg" "$T/bin" "$STUB"/{vis,prs,issues} "$T/state/running"
cat > "$T/bin/gh" <<'EOS'
#!/usr/bin/env bash
repo_key() { while [[ $# -gt 1 ]]; do [[ "$1" != --repo ]] || { echo "${2//\//__}"; return; }; shift; done; }
case "$1 $2" in
  "repo view") cat "$STUB/vis/${3//\//__}" ;;
  "pr list"|"issue list")
    key="$(repo_key "$@")"
    [[ "$1" != pr || -z "${STUB_PR_FAIL:-}" ]] || { echo "$STUB_PR_FAIL" >&2; exit 1; }
    if [[ "$1" == issue ]]; then  # like gh: only issues that carry the label
      jq 'map(select(.labels | any(.name == "ready-for-agent")))' "$STUB/issues/$key.json" | jq -r "${@: -1}"
    else jq -r "${@: -1}" "$STUB/prs/$key.json"; fi ;;
  "issue edit")
    echo "gh $*" >> "$STUB/log"
    [[ -z "${STUB_EDIT_FAIL:-}" ]] || exit 1
    f="$STUB/issues/$(repo_key "$@").json"
    jq --argjson n "$3" '(.[] | select(.number == $n) | .labels) |= map(select(.name != "ready-for-agent" and .name != "agent-failed"))' "$f" > "$f.new" && mv "$f.new" "$f" ;;
  "issue comment") echo "gh $*" >> "$STUB/log" ;;
esac
EOS
cat > "$T/bin/cj-run" <<'EOS'
#!/usr/bin/env bash
echo "cj-run $*" >> "$STUB/log"
if [[ -n "${STUB_RUN_SLEEP:-}" ]]; then echo $$ > "$STUB/run.pid"; exec sleep "$STUB_RUN_SLEEP"; fi
EOS
chmod +x "$T/bin"/*; PATH="$T/bin:$PATH"

repos() { : > "$T/cfg/repos"; local r; for r in "$@"; do echo "$r" >> "$T/cfg/repos"; done; }
vis()   { echo "$2" > "$STUB/vis/${1//\//__}"; }
prs()   { local r="$1"; shift; local b; { echo '['; local s=; for b in "$@"; do printf '%s{"headRefName":"%s"}' "$s" "$b"; s=,; done; echo ']'; } > "$STUB/prs/${r//\//__}.json"; }
# issues <repo> <json array>
issues() { echo "$2" > "$STUB/issues/${1//\//__}.json"; }
spec='{"body":"## Spec\n\ntext"}'
issue() { # issue <n> <created> <comment json> [extra label]
  local lab='{"name":"ready-for-agent"}'; [[ -z "${4:-}" ]] || lab="$lab,{\"name\":\"$4\"}"
  printf '{"number":%s,"createdAt":"%s","labels":[%s],"comments":[%s]}' "$1" "$2" "$lab" "$3"
}
reset() { rm -f "$STUB/log" "$T/state/running"/*; : > "$STUB/log"; }
A=acme/a
fresh_setup() { repos "$A=/x auto"; vis $A PRIVATE; prs $A; reset; }

# AC-14: private auto repo, spec comment, room -> startable
fresh_setup; issues $A "[$(issue 5 2026-01-01T00:00:00Z "$spec")]"
out="$("$CP" --dry-run)"
check "AC-14 startable listed" has "$out" "startable acme/a#5"
check "AC-14 dry run starts nothing" eq "$(cat "$STUB/log")" ""

# AC-15: one fixture per guard
fresh_setup; repos "$A=/x"; issues $A "[$(issue 5 2026-01-01T00:00:00Z "$spec")]"
out="$("$CP" 2>&1)"
check "AC-15 not auto: reason" has "$out" "skip acme/a: not marked auto"
check "AC-15 not auto: nothing started" eq "$(cat "$STUB/log")" ""
fresh_setup; vis $A PUBLIC; issues $A "[$(issue 5 2026-01-01T00:00:00Z "$spec")]"
out="$("$CP" 2>&1)"
check "AC-15 public: reason" has "$out" "only private repos run"
check "AC-15 public: nothing started" eq "$(cat "$STUB/log")" ""
fresh_setup; issues $A "[$(issue 5 2026-01-01T00:00:00Z '{"body":"see ## Spec above"},{"body":"intro\n## Spec"}')]"
out="$("$CP" 2>&1)"
check "AC-15 spec text elsewhere: reason" has "$out" "skip acme/a#5: no comment starting with ## Spec"
check "AC-15 spec text elsewhere: nothing started" eq "$(cat "$STUB/log")" ""
fresh_setup; prs $A agent/1-x agent/2-y; issues $A "[$(issue 5 2026-01-01T00:00:00Z "$spec")]"
out="$("$CP" 2>&1)"
check "AC-15 cap: reason" has "$out" "skip acme/a#5: cap reached"
check "AC-15 cap: nothing started" eq "$(cat "$STUB/log")" ""
check "AC-15 cap: label kept" has "$(cat "$STUB/issues/acme__a.json")" ready-for-agent

# AC-16: a stale marker is ignored; a marker plus a PR of one issue counts once
fresh_setup; prs $A agent/7-x; touch -t 200001010000 "$T/state/running/acme__a__8"
issues $A "[$(issue 5 2026-01-01T00:00:00Z "$spec"),$(issue 6 2026-01-02T00:00:00Z "$spec")]"
out="$("$CP" --dry-run)"
check "AC-16 stale marker ignored, one room left" eq "$(n_of "$out" startable)" 1
fresh_setup; prs $A agent/7-x; touch "$T/state/running/acme__a__7"
out="$("$CP" --dry-run)"
check "AC-16 marker and PR of one issue count once" eq "$(n_of "$out" startable)" 1
touch "$T/state/running/acme__a__9"
out="$("$CP" --dry-run)"
check "AC-16 two different issues fill the cap" eq "$(n_of "$out" startable)" 0

# AC-17: 3 startable, room for 2 -> exactly 2 started, oldest first
fresh_setup; issues $A "[$(issue 9 2026-01-03T00:00:00Z "$spec"),$(issue 5 2026-01-01T00:00:00Z "$spec"),$(issue 6 2026-01-02T00:00:00Z "$spec")]"
out="$("$CP")"; sleep 0.5
check "AC-17 exactly two started" eq "$(n_of "$out" started)" 2
check "AC-17 oldest two" bash -c 'grep -q "started acme/a#5" <<<"$1" && grep -q "started acme/a#6" <<<"$1"' _ "$out"
check "AC-17 cj-run called twice" eq "$(n_of "$(cat "$STUB/log")" "cj-run --report")" 2
check "AC-17 third issue keeps its label" has "$(jq -c '.[]|select(.number==9)' "$STUB/issues/acme__a.json")" ready-for-agent

# two repos share the cap in one pass
fresh_setup; repos "$A=/x auto" "acme/b=/y auto"; vis acme/b PRIVATE; prs acme/b
issues $A "[$(issue 5 2026-01-01T00:00:00Z "$spec"),$(issue 6 2026-01-02T00:00:00Z "$spec")]"
issues acme/b "[$(issue 7 2026-01-01T00:00:00Z "$spec")]"
out="$("$CP")"; sleep 0.5
check "cap counts across repos in one pass" eq "$(n_of "$out" started)" 2

# AC-18: labels removed (agent-failed too) and comment posted before cj-run; failed removal -> no run; second pass lists nothing
fresh_setup; issues $A "[$(issue 5 2026-01-01T00:00:00Z "$spec" agent-failed)]"
"$CP" >/dev/null; sleep 0.5; log="$(cat "$STUB/log")"
check "AC-18 edit names both labels" has "$log" "issue edit 5 --repo acme/a --remove-label ready-for-agent --remove-label agent-failed"
check "AC-18 order: edit, comment, run" bash -c 'e=$(grep -n "issue edit" <<<"$1" | cut -d: -f1); c=$(grep -n "issue comment" <<<"$1" | cut -d: -f1); r=$(grep -n "^cj-run" <<<"$1" | cut -d: -f1); [[ $e -lt $c && $c -lt $r ]]' _ "$log"
check "AC-18 run gets --report and the URL" has "$log" "cj-run --report https://github.com/acme/a/issues/5"
check "AC-18 comment names the log folder" has "$log" "$T/state/logs/"
out="$("$CP" --dry-run)"
check "AC-18 second pass lists nothing" hasnt "$out" "acme/a#5"
fresh_setup; issues $A "[$(issue 5 2026-01-01T00:00:00Z "$spec")]"
out="$(STUB_EDIT_FAIL=1 "$CP" 2>&1)"; sleep 0.3
check "AC-18 failed removal: cj-run not called" hasnt "$(cat "$STUB/log")" "cj-run"
check "AC-18 failed removal: reason" has "$out" "could not remove the labels"

# AC-19: poll returns fast while the started run keeps running
fresh_setup; issues $A "[$(issue 5 2026-01-01T00:00:00Z "$spec")]"
t0=$SECONDS; STUB_RUN_SLEEP=60 "$CP" >/dev/null; dt=$((SECONDS - t0)); sleep 0.5
check "AC-19 returns within 10 seconds" test $dt -lt 10
check "AC-19 the run is still running" bash -c 'kill -0 "$(cat "$1")"' _ "$STUB/run.pid"

# review fixes: no state folder yet (fresh install), minutes from config, failing PR listing, odd branch names, odd inputs
fresh_setup; rm -rf "$T/state"; issues $A "[$(issue 5 2026-01-01T00:00:00Z "$spec")]"
out="$("$CP" --dry-run 2>&1)"
check "fresh install: no state folder still lists" has "$out" "startable acme/a#5"
mkdir -p "$T/state/running"
fresh_setup; prs $A; touch -t "$(date -v-30M +%Y%m%d%H%M 2>/dev/null || date -d '30 minutes ago' +%Y%m%d%H%M)" "$T/state/running/acme__a__8" "$T/state/running/acme__a__9"
issues $A "[$(issue 5 2026-01-01T00:00:00Z "$spec")]"
out="$("$CP" --dry-run)"
check "minutes default 45: 30-minute-old markers are fresh, cap reached" has "$out" "cap reached"
mkdir -p "$T/cfg"; echo "minutes=20" > "$T/cfg/config"
out="$("$CP" --dry-run)"
check "minutes from config: 30-minute-old markers are dead" has "$out" "startable acme/a#5"
rm "$T/cfg/config"
fresh_setup; issues $A "[$(issue 5 2026-01-01T00:00:00Z "$spec")]"
out="$(STUB_PR_FAIL='HTTP 401' "$CP" --dry-run 2>&1)"; rc=$?
check "failing PR listing: non-zero exit" test $rc -ne 0
check "failing PR listing: reason, nothing listed" bash -c 'grep -q "gh pr list failed for acme/a: HTTP 401" <<<"$1" && ! grep -q startable <<<"$1"' _ "$out"
fresh_setup; prs $A agent/foo agent/1-x; issues $A "[$(issue 5 2026-01-01T00:00:00Z "$spec"),$(issue 6 2026-01-02T00:00:00Z "$spec")]"
out="$("$CP" --dry-run)"
check "branch without number is no running agent" eq "$(n_of "$out" startable)" 1
fresh_setup; printf 'acme/a auto\n%s\n' "$A=/x auto" > "$T/cfg/repos"; issues $A "[$(issue 5 2026-01-01T00:00:00Z "$spec")]"
out="$("$CP" --dry-run)"
check "repos line without = is skipped" eq "$out" "startable acme/a#5"
fresh_setup; issues $A "[$(issue 5 2026-01-01T00:00:00Z "$spec" agent-failed)]"
out="$(CJ_POLL_RUN=$T/bin/missing "$CP" 2>&1)"; rc=$?
check "cj-run not executable: stops before any label change" bash -c '[[ $1 -eq 2 ]] && grep -q "not executable" <<<"$2" && [[ ! -s "$3" ]]' _ $rc "$out" "$STUB/log"
check "bad argument: usage, exit 2" bash -c '"$1" --bogus >/dev/null 2>&1; [[ $? -eq 2 ]]' _ "$CP"

exit $FAIL
