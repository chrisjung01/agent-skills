#!/usr/bin/env bash
# Template: wire up epic relationships on GitHub.
#   1. sub-issue (parent/child)  -> gh issue edit --parent
#   2. blocked-by (dependencies) -> REST /dependencies/blocked_by
# Both work cross-repo (e.g. app <-> backend).
#
# Fill in EPIC and the BLOCKS table below, then:
#   bash scripts/link-issues.sh

set -euo pipefail

EPIC="<owner>/<repo>:<number>"

# letter -> owner/repo:number  (letters are the stable anchors from the docs)
declare -A TICKET=(
  [A]="<owner>/<repo>:<number>"
  [B]="<owner>/<repo>:<number>"
  [C]="<owner>/<repo>:<number>"
)

# "<child> is blocked by <blocker,blocker>"
declare -A BLOCKED_BY=(
  [C]="A"
  # [D]="A,B"
)

split() { # "owner/repo:number" -> sets $OWNER $REPO $NUM
  OWNER="${1%%/*}"; local rest="${1#*/}"; REPO="${rest%%:*}"; NUM="${rest##*:}"
}

issue_id() { # global database id, needed for the dependency API
  split "$1"; gh api "repos/$OWNER/$REPO/issues/$NUM" --jq .id
}

# --- 1. sub-issues ------------------------------------------------------------
split "$EPIC"; EPIC_URL="https://github.com/$OWNER/$REPO/issues/$NUM"
for letter in "${!TICKET[@]}"; do
  split "${TICKET[$letter]}"
  echo "sub-issue: $letter (#$NUM) -> epic #${EPIC_URL##*/}"
  gh issue edit "$NUM" --repo "$OWNER/$REPO" --parent "$EPIC_URL"
done

# --- 2. blocked-by ------------------------------------------------------------
for letter in "${!BLOCKED_BY[@]}"; do
  split "${TICKET[$letter]}"; child_owner="$OWNER" child_repo="$REPO" child_num="$NUM"
  IFS=',' read -ra blockers <<< "${BLOCKED_BY[$letter]}"
  for b in "${blockers[@]}"; do
    bid="$(issue_id "${TICKET[$b]}")"
    echo "blocked-by: $letter (#$child_num) <- $b (id $bid)"
    gh api --method POST \
      "repos/$child_owner/$child_repo/issues/$child_num/dependencies/blocked_by" \
      -F issue_id="$bid" --silent
  done
done

echo "Done. Verify:  gh api repos/OWNER/REPO/issues/N/dependencies/blocked_by --jq '.[].number'"
