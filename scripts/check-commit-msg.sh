#!/usr/bin/env bash
# Checks a commit message file against AGENTS.md "Conventions": no attribution trailer; on a feat/<n>-* or fix/<n>-* branch the subject ends in (#<n>).
# Used by .githooks/commit-msg. Merge, revert, fixup! and squash! subjects are skipped.
msg="$1"; branch="${2:-$(git rev-parse --abbrev-ref HEAD 2>/dev/null)}"
subject="$(head -n1 "$msg")"
case "$subject" in Merge\ *|Revert\ *|fixup!\ *|squash!\ *) exit 0 ;; esac
if grep -qiE '^(Co-Authored-By|Generated-By|Signed-off-by):' "$msg"; then
  echo "commit-msg: attribution trailer found; AGENTS.md says messages end with their last content line" >&2; exit 1
fi
if [[ "$branch" =~ ^(feat|fix)/([0-9]+)- ]] && [[ "$subject" != *"(#${BASH_REMATCH[2]})" ]]; then
  echo "commit-msg: subject must end with (#${BASH_REMATCH[2]})" >&2; exit 1
fi
