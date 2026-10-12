#!/usr/bin/env bash
# Checks every skill: SKILL.md exists, frontmatter has name + description, name == folder name,
# the description matches the invocation kind, no skill calls a user-invoked one, no organisation term in any tracked file,
# every copy of the scannable sentence matches the one in AGENTS.md, the PR body template holds its five blocks in order.
set -uo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
errors=0
user_invoked=()
for dir in skills/*/; do
  skill="$(basename "$dir")"; file="$dir/SKILL.md"; before=$errors
  if [[ ! -f "$file" ]]; then echo "✗ $skill: SKILL.md missing"; errors=$((errors+1)); continue; fi
  if [[ "$(head -n1 "$file")" != "---" ]]; then echo "✗ $skill: frontmatter missing"; errors=$((errors+1)); continue; fi
  fm="$(awk 'NR==1{next} /^---$/{exit} {print}' "$file")"
  name="$(grep -E '^name:' <<<"$fm" | head -n1 | sed -E 's/^name:[[:space:]]*//')"
  desc="$(grep -E '^description:' <<<"$fm" | head -n1 | sed -E 's/^description:[[:space:]]*//')"
  [[ -z "$name" ]] && { echo "✗ $skill: 'name' missing"; errors=$((errors+1)); }
  [[ -n "$name" && "$name" != "$skill" ]] && { echo "✗ $skill: name '$name' != folder name"; errors=$((errors+1)); }
  [[ -n "$name" && ! "$name" =~ ^[a-z0-9]+(-[a-z0-9]+)*$ ]] && { echo "✗ $skill: name may only contain lowercase letters, digits, hyphens"; errors=$((errors+1)); }
  [[ -z "$desc" ]] && { echo "✗ $skill: 'description' missing"; errors=$((errors+1)); }
  # Invocation: user-invoked skills have a one-line human description; model-invoked ones list triggers.
  plain="${desc#[\"\']}"
  if grep -qE '^disable-model-invocation:[[:space:]]*true' <<<"$fm"; then
    user_invoked+=("$skill")
    [[ "$plain" == "Use when"* ]] && { echo "✗ $skill: user-invoked, so the description is a one-line summary, not 'Use when ...' triggers"; errors=$((errors+1)); }
  else
    [[ -n "$plain" && "$plain" != "Use when"* ]] && { echo "✗ $skill: model-invoked, so the description starts with 'Use when'"; errors=$((errors+1)); }
  fi
  [[ $errors -eq $before ]] && echo "✓ $skill"
done
# No skill calls a user-invoked skill through the Skill tool.
for u in "${user_invoked[@]+"${user_invoked[@]}"}"; do
  hits="$(grep -rn --include=SKILL.md -e "Call the Skill tool" skills/ | grep -F "\"$u\"" || true)"
  [[ -n "$hits" ]] && { echo "✗ '$u' is user-invoked; no skill may call it:"; echo "$hits"; errors=$((errors+1)); }
done
# Organisation facts: words listed one per line in a local, gitignored .org-terms file must not appear in any
# tracked file. Scans the index, so the pre-commit hook sees exactly what is staged. A linked worktree has no
# copy of the file, so it falls back to the main checkout's.
terms=.org-terms
[[ -f $terms ]] || terms="$(git rev-parse --path-format=absolute --git-common-dir 2>/dev/null)/../.org-terms"
if [[ -f $terms ]]; then
  while IFS= read -r term; do
    term="${term%$'\r'}"
    [[ -z "$term" || "$term" == \#* ]] && continue
    hits="$(git grep --cached -niF -e "$term")"; rc=$?
    (( rc > 1 )) && { echo "✗ org term scan failed (git grep exit $rc)"; errors=$((errors+1)); break; }
    # Fixed text that may contain a term as part of a word: the MIT licence's warranty line and the GitHub
    # token prefix. A line still counts when the term appears outside them. Binary hits have no content and
    # always count. gh[p] keeps this line itself from matching.
    # ponytail: tolower folds ASCII only, enough for the list today.
    hits="$(T="$term" awk '/^Binary file .* matches$/ { print; next }
      { s = $0; sub(/^[^:]*:[0-9]+:/, "", s); gsub(/WARRANTIES OF MERCHANTABILITY|gh[p]_[A-Za-z0-9]/, "", s)
        if (index(tolower(s), tolower(ENVIRON["T"]))) print }' <<<"$hits")"
    [[ -n "$hits" ]] && { echo "✗ org term '$term':"; echo "$hits"; errors=$((errors+1)); }
  done < "$terms"
else
  echo "! .org-terms missing: org term scan skipped"
fi
# Scannable sentence: AGENTS.md holds the source, quoted. Every tracked line that holds one of two fragments counts
# as a copy and must hold the whole sentence; the second fragment catches a copy whose opening words changed.
# Scans the index like the org term scan. This file names the fragments, so it is excluded.
# ponytail: scoped to this one sentence; generalize once a second sentence has to be copied word for word.
sentence="$(git show :AGENTS.md 2>/dev/null | grep -oE '"Write it scannable[^"]*"' | head -n1)"
sentence="${sentence//\"/}"
if [[ -z "$sentence" ]]; then
  echo "✗ scannable sentence: source missing in AGENTS.md"; errors=$((errors+1))
else
  hits="$(git grep --cached -nF -e "Write it scannable" -e "in the reader's terms" -- ':!scripts/validate.sh')"; rc=$?
  (( rc > 1 )) && { echo "✗ scannable sentence scan failed (git grep exit $rc)"; errors=$((errors+1)); }
  hits="$(grep -vF -e "$sentence" <<<"$hits")"
  [[ -n "$hits" ]] && { echo "✗ scannable sentence drifted from AGENTS.md:"; cut -d: -f1,2 <<<"$hits"; errors=$((errors+1)); }
fi
# PR body template: the five blocks, each a bold list item, present and in this order (#46).
body=skills/cj-implement/PR-BODY.md
want="Why,What,How to test,Verified,Risk"
have="$(grep -oE '^- \*\*(Why|What|How to test|Verified|Risk)\*\*' "$body" 2>/dev/null | sed -E 's/^- \*\*(.*)\*\*$/\1/' | paste -sd, -)"
[[ "$have" == "$want" ]] || { echo "✗ $body: blocks are '$have', need '$want'"; errors=$((errors+1)); }
[[ $errors -eq 0 ]]
