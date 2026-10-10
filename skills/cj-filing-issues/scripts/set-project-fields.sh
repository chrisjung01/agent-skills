#!/usr/bin/env bash
# Template: set GitHub Project (v2) single-select fields on a batch of issues
# to match an epic (e.g. status, work package, team).
#
# Copy this into your feature folder, then fill in:
#   - OWNER, PROJECT_NUMBER, PROJECT_ID and the FIELD_* / OPTION_* IDs (see "Finding IDs" below)
#   - the ISSUES list (owner/repo:number)
#
# Prerequisite (once, interactive — gh needs the project scope):
#   gh auth refresh -h github.com -s project
#
# Usage:
#   bash scripts/set-project-fields.sh

set -euo pipefail

# --- Fill these in (Finding IDs: see bottom of file) -------------------------
OWNER="<owner>"          # org or user that owns the project board
PROJECT_NUMBER="<number>" # from: gh project list --owner "$OWNER"
PROJECT_ID="PVT_xxx"
FIELD_STATUS="PVTSSF_xxx"
OPTION_STATUS_NEXT="xxxxxxxx"
FIELD_WORK_PACKAGE="PVTSSF_xxx"
OPTION_WP_VALUE="xxxxxxxx"
FIELD_PROJECTS="PVTSSF_xxx"
OPTION_TEAM_VALUE="xxxxxxxx"

declare -a ISSUES=(
  "<owner>/<repo>:<number>"
  "<owner>/<repo>:<number>"
)
# -----------------------------------------------------------------------------

get_project_item_id() {
  local owner="$1" repo="$2" number="$3"
  gh api graphql -f query='
    query($owner: String!, $repo: String!, $number: Int!) {
      repository(owner: $owner, name: $repo) {
        issue(number: $number) {
          projectItems(first: 5) { nodes { id project { id } } }
        }
      }
    }
  ' -f owner="$owner" -f repo="$repo" -F number="$number" \
    --jq ".data.repository.issue.projectItems.nodes[]
      | select(.project.id==\"$PROJECT_ID\") | .id"
}

set_select_field() {
  gh project item-edit \
    --id "$1" --project-id "$PROJECT_ID" \
    --field-id "$2" --single-select-option-id "$3"
}

for entry in "${ISSUES[@]}"; do
  owner="${entry%%/*}"; rest="${entry#*/}"
  repo="${rest%%:*}"; number="${rest##*:}"

  item_id="$(get_project_item_id "$owner" "$repo" "$number")"
  if [[ -z "$item_id" ]]; then
    echo "#${number} (${owner}/${repo}) not on project yet — adding."
    gh project item-add "$PROJECT_NUMBER" --owner "$OWNER" \
      --url "https://github.com/${owner}/${repo}/issues/${number}"
    item_id="$(get_project_item_id "$owner" "$repo" "$number")"
  fi

  echo "Updating #${number} (${owner}/${repo}) …"
  set_select_field "$item_id" "$FIELD_STATUS" "$OPTION_STATUS_NEXT"
  set_select_field "$item_id" "$FIELD_WORK_PACKAGE" "$OPTION_WP_VALUE"
  set_select_field "$item_id" "$FIELD_PROJECTS" "$OPTION_TEAM_VALUE"
done

echo "Done."

# --- Finding IDs -------------------------------------------------------------
# List a project's fields + single-select option IDs:
#   gh project field-list "$PROJECT_NUMBER" --owner "$OWNER" --format json
# Get PROJECT_ID (node id) from:
#   gh project list --owner "$OWNER" --format json
# Read an existing issue's current field values to copy from the epic:
#   gh project item-list "$PROJECT_NUMBER" --owner "$OWNER" --format json
