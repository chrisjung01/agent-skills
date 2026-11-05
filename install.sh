#!/usr/bin/env bash
# Symlinks every skill in ./skills into the skill directories of Claude Code and Codex, builds
# the cj-whetstone metrics binary, registers its Codex SessionEnd hook in $CODEX_HOME/hooks.json
# (default ~/.codex, backed up first) and prints its Claude Code hook line (settings.json stays
# untouched). A global install also links bin/cj-run and bin/cj-poll into ~/.local/bin, copies config/cj-run/*
# to ~/.config/cj-run/ where no file exists, and prints a launchd job and two `gh label create` lines
# (it never loads the job or creates a label).
#
#   ./install.sh                      global: ~/.claude/skills + ~/.agents/skills
#   ./install.sh --project <path>     per project: <path>/.claude/skills + <path>/.agents/skills
#   ./install.sh --uninstall [...]    removes only symlinks that point into this repo and, globally, its Codex hook
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
    -h|--help)   sed -n '2,14p' "$0"; exit 0 ;;
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
if [[ -z "$ONLY" || "$ONLY" == "cj-whetstone" ]]; then
  manifest="$SKILLS_DIR/cj-whetstone/skill-metrics/Cargo.toml"
  bin="${manifest%/*}/target/release/skill-metrics"
  codex_hook="bash \"$SKILLS_DIR/cj-whetstone/scripts/codex-session-end-hook.sh\""
  codex_home="${CODEX_HOME:-$HOME/.codex}"
  dry=""; [[ $DRY -eq 1 ]] && dry="--dry-run"
  if [[ "$MODE" == "install" ]]; then
    if command -v cargo >/dev/null; then
      # --target-dir: the hook looks there, whatever CARGO_TARGET_DIR says.
      run cargo build --release --quiet --manifest-path "$manifest" --target-dir "${manifest%/*}/target"
      [[ $DRY -eq 1 ]] || echo "✓ skill-metrics built"
    else
      echo "⚠ cargo not found, skill-metrics not built (https://rustup.rs)"
    fi
    if [[ "$TARGET_ROOT" != "$HOME" ]]; then
      echo "Codex hook: only the global install registers it (./install.sh without --project)"
    elif [[ -x "$bin" ]]; then
      # shellcheck disable=SC2086 # $dry is empty or one flag
      "$bin" codex-hooks install "$codex_hook" $dry || echo "⚠ Codex hook not registered"
    elif [[ $DRY -eq 1 ]]; then
      echo "[dry-run] register the Codex SessionEnd hook in $codex_home/hooks.json"
    else
      echo "⚠ skill-metrics not built, Codex hook not registered"
    fi
    if [[ -d "$codex_home" ]]; then
      echo "Codex runs the hook once you trust it: start codex, it reports hooks that need review; approve it under /hooks."
      if command -v codex >/dev/null && [[ "$(codex features list 2>/dev/null | awk '$1 == "hooks" { print $NF }')" == "false" ]]; then
        echo "⚠ Hooks are off in Codex: set hooks = true under [features] in $codex_home/config.toml."
      fi
      echo "Without the hook, /cj-whetstone's full scan still collects Codex sessions."
    fi
    hook="$SKILLS_DIR/cj-whetstone/scripts/session-end-hook.sh"
    cat <<HOOK

Add this hook to ~/.claude/settings.json under "hooks", if it is not there yet:
  "SessionEnd": [{ "hooks": [{ "type": "command", "command": "bash \"$hook\"" }] }]
HOOK
  elif [[ "$TARGET_ROOT" != "$HOME" ]]; then
    :  # the global Codex hook stays for the global install
  elif [[ -x "$bin" ]]; then
    # shellcheck disable=SC2086
    "$bin" codex-hooks uninstall "$codex_hook" $dry || echo "⚠ Codex hook not removed"
  elif [[ -f "$codex_home/hooks.json" ]]; then
    echo "⚠ skill-metrics not built: remove the SessionEnd hook running $codex_hook from $codex_home/hooks.json yourself"
  fi
fi

# cj-run and cj-poll for unattended runs (global install only): links, missing config templates, printed job and labels.
if [[ ( -z "$ONLY" || "$ONLY" == "cj-run" ) && "$TARGET_ROOT" == "$HOME" ]]; then
  localbin="$HOME/.local/bin"; cfg="$HOME/.config/cj-run"
  for tool in cj-run cj-poll; do
    if [[ "$MODE" == "install" ]]; then
      if [[ -L "$localbin/$tool" && "$(readlink "$localbin/$tool")" == "$REPO_DIR/bin/$tool" ]]; then
        echo "✓ $localbin/$tool (already linked)"
      elif [[ -e "$localbin/$tool" || -L "$localbin/$tool" ]]; then
        echo "⚠ $localbin/$tool already exists, not overwritten"
      else
        run mkdir -p "$localbin"; run ln -s "$REPO_DIR/bin/$tool" "$localbin/$tool" && echo "+ $localbin/$tool"
      fi
    elif [[ -L "$localbin/$tool" && "$(readlink "$localbin/$tool")" == "$REPO_DIR/bin/$tool" ]]; then
      run rm "$localbin/$tool" && echo "- $localbin/$tool"
    fi
  done
  if [[ "$MODE" == "install" ]]; then
    for tpl in "$REPO_DIR"/config/cj-run/*; do
      [[ -e "$cfg/${tpl##*/}" ]] && { echo "✓ $cfg/${tpl##*/} (kept)"; continue; }
      run mkdir -p "$cfg"; run cp "$tpl" "$cfg/" && echo "+ $cfg/${tpl##*/}"
    done
    cat <<JOB

Timer for cj-poll: save as ~/Library/LaunchAgents/com.cj-run.poll.plist, then run
  launchctl bootstrap gui/\$(id -u) ~/Library/LaunchAgents/com.cj-run.poll.plist
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
  <key>Label</key><string>com.cj-run.poll</string>
  <key>ProgramArguments</key><array><string>$localbin/cj-poll</string></array>
  <key>StartInterval</key><integer>600</integer>
  <key>AbandonProcessGroup</key><true/>
  <key>EnvironmentVariables</key><dict>
    <key>CLAUDE_CONFIG_DIR</key><string>${CLAUDE_CONFIG_DIR:-$HOME/.claude}</string>
    <key>PATH</key><string>$localbin:/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin</string>
  </dict>
</dict></plist>

Labels, once per repo marked auto in $cfg/repos (run after your yes):
  gh label create ready-for-agent --repo <owner>/<repo>
  gh label create agent-failed --repo <owner>/<repo>
JOB
  fi
fi

# The bare-question hook is gone (#107); an old entry for it in settings.json is dead weight.
if grep -qs "bare-question-hook.sh" "$HOME/.claude/settings.json"; then
  echo "⚠ ~/.claude/settings.json still runs bare-question-hook.sh: remove the hook entry that runs it, the script no longer exists"
fi
