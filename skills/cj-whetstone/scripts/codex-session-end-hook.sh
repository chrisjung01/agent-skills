#!/usr/bin/env bash
# Codex SessionEnd hook: collects the ending session into the metrics database. Exits 0 and does
# nothing until ./install.sh has built the binary. The payload on stdin names the transcript;
# input the binary cannot collect is reported on stderr with exit 1, and the next full scan of
# /cj-whetstone stores the session if its rollout exists.
bin="$(dirname "${BASH_SOURCE[0]}")/../skill-metrics/target/release/skill-metrics"
[[ -x "$bin" ]] || exit 0
exec "$bin" codex-hook
