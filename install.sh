#!/usr/bin/env bash
# Symlinks every skill in ./skills into the skill directories of Claude Code and Codex, builds
# the cj-whetstone metrics binary and prints its SessionEnd and PreToolUse hook lines (settings
# stay untouched).
#
#   ./install.sh                      global: ~/.claude/skills + ~/.agents/skills
#   ./install.sh --project <path>     per project: <path>/.claude/skills + <path>/.agents/skills
#   ./install.sh --uninstall [...]    removes only symlinks that point into this repo
#   ./install.sh --dry-run [...]      prints what would happen, changes nothing
#   ./install.sh --skill <name> [...] only this one skill
#   ./install.sh --force [...]        move existing folders/foreign symlinks to <dir>.bak-<time>/ and replace them
set -euo pipefail

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SKILLS_DIR="$REPO_DIR/skills"
MODE="install"; DRY=0; FORCE=0; TARGET_ROOT="$HOME"; ONLY=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --uninstall) MODE="uninstall" ;;
    --dry-run)   DRY=1 ;;
    --force)     FORCE=1 ;;
    --project)   TARGET_ROOT="$(cd "${2:?path missing}" && pwd)"; shift ;;
    --skill)     ONLY="${2:?name missing}"; shift ;;
    -h|--help)   sed -n '2,12p' "$0"; exit 0 ;;
    *) echo "Unknown option: $1" >&2; exit 1 ;;
  esac
  shift
done

TARGETS=("$TARGET_ROOT/.claude/skills" "$TARGET_ROOT/.agents/skills")
STAMP="$(date +%Y%m%d-%H%M%S)"
run() { if [[ $DRY -eq 1 ]]; then echo "[dry-run] $*"; else "$@"; fi; }

shopt -s nullglob
# Remove dead symlinks into this repo (e.g. after a skill was renamed)
for dir in "${TARGETS[@]}"; do
  for link in "$dir"/*; do
    [[ -L "$link" && ! -e "$link" && "$(readlink "$link")" == "$SKILLS_DIR"/* ]] || continue
    run rm "$link" && echo "- $link (dead link)"
  done
done

for skill_path in "$SKILLS_DIR"/*/; do
  skill="$(basename "$skill_path")"
  [[ -n "$ONLY" && "$skill" != "$ONLY" ]] && continue
  [[ "$skill" == "example-skill" ]] && continue  # template only
  [[ -f "$skill_path/SKILL.md" ]] || { echo "⚠ $skill: no SKILL.md, skipped"; continue; }
  src="${skill_path%/}"

  for dir in "${TARGETS[@]}"; do
    dest="$dir/$skill"
    if [[ "$MODE" == "install" ]]; then
      run mkdir -p "$dir"
      if [[ -L "$dest" && "$(readlink "$dest")" == "$src" ]]; then
        echo "✓ $dest (already linked)"; continue
      elif [[ -e "$dest" || -L "$dest" ]]; then
        if [[ $FORCE -eq 0 ]]; then
          echo "⚠ $dest already exists, not overwritten (--force backs up and replaces)"; continue
        fi
        # ponytail: backup outside the skill dir, otherwise the tool loads the skill twice
        backup="$dir.bak-$STAMP"
        run mkdir -p "$backup" && run mv "$dest" "$backup/" && echo "↻ $dest → $backup/"
      fi
      run ln -s "$src" "$dest" && echo "+ $dest"
    else
      if [[ -L "$dest" && "$(readlink "$dest")" == "$src" ]]; then
        run rm "$dest" && echo "- $dest"
      fi
    fi
  done
done

# cj-whetstone's metrics binary; the hooks do nothing until it exists.
if [[ "$MODE" == "install" && ( -z "$ONLY" || "$ONLY" == "cj-whetstone" ) ]]; then
  manifest="$SKILLS_DIR/cj-whetstone/skill-metrics/Cargo.toml"
  if command -v cargo >/dev/null; then
    # --target-dir: the hook looks there, whatever CARGO_TARGET_DIR says.
    run cargo build --release --quiet --manifest-path "$manifest" --target-dir "${manifest%/*}/target"
    [[ $DRY -eq 1 ]] || echo "✓ skill-metrics built"
  else
    echo "⚠ cargo not found, skill-metrics not built (https://rustup.rs)"
  fi
  hook="$SKILLS_DIR/cj-whetstone/scripts/session-end-hook.sh"
  bare="$SKILLS_DIR/cj-whetstone/scripts/bare-question-hook.sh"
  cat <<HOOK

Add these hooks to ~/.claude/settings.json under "hooks", if they are not there yet:
  "SessionEnd": [{ "hooks": [{ "type": "command", "command": "bash \"$hook\"" }] }],
  "PreToolUse": [{ "matcher": "AskUserQuestion", "hooks": [{ "type": "command", "command": "bash \"$bare\"" }] }]
HOOK
fi
