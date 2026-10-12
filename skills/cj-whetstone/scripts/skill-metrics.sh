#!/usr/bin/env bash
# Runs the skill-metrics binary; commands and options in the usage at the top of
# ../skill-metrics/src/main.rs. Without a command it upserts one row per skill call
# from every local Claude Code and Codex transcript into ~/.claude/whetstone/skill-metrics.sqlite.
set -euo pipefail
exec cargo run --quiet --release --manifest-path "$(dirname "${BASH_SOURCE[0]}")/../skill-metrics/Cargo.toml" -- "$@"
