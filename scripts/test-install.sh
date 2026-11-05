#!/usr/bin/env bash
# Tests for the cj-run part of install.sh under a temporary HOME: dry run changes nothing, a real run never overwrites.
set -uo pipefail
HERE="$(cd "$(dirname "$0")/.." && pwd)"
T="$(cd "$(mktemp -d)" && pwd -P)"; trap 'rm -rf "$T"' EXIT
FAIL=0
ok()   { echo "ok   $1"; }
fail() { echo "FAIL $1"; FAIL=1; }
has()  { grep -qF -- "$2" <<<"$1" && ok "$3" || fail "$3"; }
run()  { HOME="$T/h" CLAUDE_CONFIG_DIR= bash "$HERE/install.sh" --skill cj-run "$@" 2>&1; }

mkdir -p "$T/h"
out="$(run --dry-run)"
has "$out" "$T/h/.local/bin/cj-run" "AC-20 dry run names the cj-run link"
has "$out" "$T/h/.local/bin/cj-poll" "AC-20 dry run names the cj-poll link"
has "$out" "$T/h/.config/cj-run/base.allow" "AC-20 dry run names a config template"
has "$out" "<integer>600</integer>" "AC-20 job runs every 600 seconds"
has "$out" "<key>CLAUDE_CONFIG_DIR</key>" "AC-20 job sets CLAUDE_CONFIG_DIR"
has "$out" "$T/h/.local/bin:/opt/homebrew/bin" "AC-20 job PATH holds both folders"
has "$out" "<key>AbandonProcessGroup</key><true/>" "AC-20 job abandons the process group"
has "$out" "gh label create ready-for-agent" "AC-20 label line ready-for-agent"
has "$out" "gh label create agent-failed" "AC-20 label line agent-failed"
[[ -z "$(ls -A "$T/h")" ]] && ok "AC-20 dry run creates nothing" || fail "AC-20 dry run creates nothing"

mkdir -p "$T/h/.config/cj-run" "$T/stub"; echo mine > "$T/h/.config/cj-run/base.allow"
for c in launchctl gh; do printf '#!/bin/sh\necho "$0 $*" >> "%s/called"\n' "$T/stub" > "$T/stub/$c"; chmod +x "$T/stub/$c"; done
out="$(run --dry-run)"
has "$out" "$T/h/.config/cj-run/base.allow (kept)" "AC-20 dry run keeps an existing file"
[[ "$out" != *"+ $T/h/.config/cj-run/base.allow"* ]] && ok "AC-20 dry run does not plan to copy it" || fail "AC-20 dry run does not plan to copy it"
out="$(PATH="$T/stub:$PATH" run)"
[[ "$(cat "$T/h/.config/cj-run/base.allow")" == mine ]] && ok "AC-21 existing base.allow unchanged" || fail "AC-21 existing base.allow unchanged"
[[ -f "$T/h/.config/cj-run/base.deny" ]] && ok "AC-21 missing template copied" || fail "AC-21 missing template copied"
[[ "$(readlink "$T/h/.local/bin/cj-run")" == "$HERE/bin/cj-run" ]] && ok "links cj-run to bin/" || fail "links cj-run to bin/"
[[ ! -e "$T/stub/called" ]] && ok "AC-21 no launchctl or gh call" || fail "AC-21 no launchctl or gh call"
[[ ! -e "$T/h/Library" ]] && ok "AC-21 no job written" || fail "AC-21 no job written"
rm "$T/h/.local/bin/cj-run"; echo foreign > "$T/h/.local/bin/cj-run"; run >/dev/null
[[ "$(cat "$T/h/.local/bin/cj-run")" == foreign ]] && ok "edge: foreign cj-run not overwritten" || fail "edge: foreign cj-run not overwritten"
rm "$T/h/.local/bin/cj-run"; run >/dev/null; has "$(run)" "already linked" "second run keeps the links"
run --uninstall >/dev/null; [[ ! -e "$T/h/.local/bin/cj-run" ]] && ok "uninstall removes the link" || fail "uninstall removes the link"
exit $FAIL
