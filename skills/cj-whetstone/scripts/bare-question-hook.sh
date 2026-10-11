#!/usr/bin/env bash
# Claude Code PreToolUse hook for the choice tool: blocks a question asked with no visible text
# since the last tool call. Exits 0 and lets every question through until ./install.sh has built
# the binary. The payload comes on stdin.
bin="$(dirname "${BASH_SOURCE[0]}")/../skill-metrics/target/release/skill-metrics"
[[ -x "$bin" ]] || exit 0
exec "$bin" hook
