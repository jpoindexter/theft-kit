#!/bin/zsh
set -euo pipefail
ROOT="${1:-/Users/jasonpoindexter/Documents/GitHub/theft-kit}"
checked=0
bad=0

for d in "$ROOT"/*; do
  [ -d "$d" ] || continue
  f="$d/SKILL.md"
  [ -f "$f" ] || continue
  checked=$((checked+1))
  first=$(head -n 1 "$f" || true)
  if [ "$first" != "---" ]; then
    echo "missing-frontmatter $f"
    bad=$((bad+1))
    continue
  fi
  fm=$(awk 'NR==1 && $0=="---"{front=1;next} front&&$0=="---"{exit} front{print}' "$f")
  if ! ruby -ryaml -e 'YAML.safe_load(STDIN.read)' <<< "$fm" >/dev/null 2>&1; then
    echo "invalid-yaml $f"
    bad=$((bad+1))
  fi
done

echo "checked=$checked"
echo "bad=$bad"
[ "$bad" -eq 0 ]
