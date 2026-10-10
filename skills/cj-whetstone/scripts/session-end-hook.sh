#!/usr/bin/env bash
# Claude Code SessionEnd hook: extracts the ending session into the metrics database in the
# background, so the hook returns at once. Exits 0 and does nothing until ./install.sh has built
# the binary. The session id comes from the hook payload on stdin.
bin="$(dirname "${BASH_SOURCE[0]}")/../skill-metrics/target/release/skill-metrics"
[[ -x "$bin" ]] || exit 0
session="$(sed -n 's/.*"session_id" *: *"\([A-Za-z0-9-]*\)".*/\1/p' | head -n 1)"
[[ -n "$session" ]] || exit 0
# ponytail: a failed run is not logged; the next full scan of /cj-whetstone stores the session.
nohup "$bin" --session "$session" </dev/null >/dev/null 2>&1 &
exit 0
