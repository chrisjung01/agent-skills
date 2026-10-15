#!/usr/bin/env bash
# Template: wire up epic relationships on GitHub.
#   1. sub-issue (parent/child)  -> gh issue edit --parent
#   2. blocked-by (dependencies) -> REST /dependencies/blocked_by
# Both work cross-repo (e.g. app <-> backend).
#
# Fill in EPIC and the TICKETS and BLOCKED_BY tables below, then:
#   bash scripts/link-issues.sh

set -euo pipefail

EPIC="<owner>/<repo>:<number>"

# letter -> owner/repo:number  (letters are the stable anchors from the docs)
# one "<letter> <owner>/<repo>:<number>" per line; plain lines, not
# `declare -A`, so the script also runs on macOS's bash 3.2
TICKETS="
A <owner>/<repo>:<number>
B <owner>/<repo>:<number>
C <owner>/<repo>:<number>
"

# "<child> is blocked by <blocker,blocker>", one "<child> <blockers>" per line
BLOCKED_BY="
C A
# D A,B
"

ticket() { # letter -> owner/repo:number
  local letter ref
  while read -r letter ref; do
    if [ "$letter" = "$1" ]; then echo "$ref"; return; fi
  done <<< "$TICKETS"
  echo "unknown ticket letter: $1" >&2; return 1
}

split() { # "owner/repo:number" -> sets $OWNER $REPO $NUM
  OWNER="${1%%/*}"; local rest="${1#*/}"; REPO="${rest%%:*}"; NUM="${rest##*:}"
}

issue_id() { # global database id, needed for the dependency API
  split "$1"; gh api "repos/$OWNER/$REPO/issues/$NUM" --jq .id
}

# fd 3 feeds the loops, so a gh call that reads stdin cannot eat the table

# --- 1. sub-issues ------------------------------------------------------------
split "$EPIC"; EPIC_URL="https://github.com/$OWNER/$REPO/issues/$NUM"
while read -r letter ref <&3; do
  case "$letter" in ""|"#"*) continue ;; esac
  split "$ref"
  echo "sub-issue: $letter (#$NUM) -> epic #${EPIC_URL##*/}"
  gh issue edit "$NUM" --repo "$OWNER/$REPO" --parent "$EPIC_URL"
done 3<<< "$TICKETS"

# --- 2. blocked-by ------------------------------------------------------------
while read -r letter blocker_list <&3; do
  case "$letter" in ""|"#"*) continue ;; esac
  split "$(ticket "$letter")"; child_owner="$OWNER" child_repo="$REPO" child_num="$NUM"
  IFS=',' read -ra blockers <<< "$blocker_list"
  for b in "${blockers[@]}"; do
    bid="$(issue_id "$(ticket "$b")")"
    echo "blocked-by: $letter (#$child_num) <- $b (id $bid)"
    gh api --method POST \
      "repos/$child_owner/$child_repo/issues/$child_num/dependencies/blocked_by" \
      -F issue_id="$bid" --silent
  done
done 3<<< "$BLOCKED_BY"

echo "Done. Verify:  gh api repos/OWNER/REPO/issues/N/dependencies/blocked_by --jq '.[].number'"
