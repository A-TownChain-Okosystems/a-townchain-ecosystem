#!/usr/bin/env bash
# A-TownChain-Okosystems — Org-Snapshot-Refresh
#
# Zweck: lokale Arbeitskopie aktuell halten und JEDEN Lauf mit Zeitstempel und
# allen HEAD-SHAs dokumentieren. Kein Push, keine Schreiboperation auf Remotes.
#
# Disziplin:
#   * GitHub ist die Wahrheit, diese Kopie nur Arbeitsbestand.
#   * Keine lokalen Aenderungen ungeprueft pushen.
#   * Jeder Lauf erzeugt eine append-only Zeile in SNAPSHOTS.jsonl.
#
# Usage:
#   ./refresh.sh              # fetch + Manifest-Zeile
#   ./refresh.sh --quiet      # nur JSON-Zeile
#   ./refresh.sh --check      # Exit 1, wenn ein Remote voraus ist
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MANIFEST="$ROOT/SNAPSHOTS.jsonl"
LATEST="$ROOT/ORG_MANIFEST.md"
TS="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
QUIET=0; CHECK=0
for a in "$@"; do
  case "$a" in
    --quiet) QUIET=1 ;;
    --check) CHECK=1 ;;
  esac
done

shopt -s nullglob
TMP="$(mktemp)"
for d in "$ROOT"/*/ "$ROOT"/.[!.]*/; do
  [ -d "$d/.git" ] || continue
  name="$(basename "$d")"
  git -C "$d" fetch --quiet --prune origin 2>/dev/null || true
  branch="$(git -C "$d" rev-parse --abbrev-ref HEAD 2>/dev/null || echo unknown)"
  head="$(git -C "$d" rev-parse HEAD 2>/dev/null || echo '')"
  remote="$(git -C "$d" rev-parse "origin/$branch" 2>/dev/null || echo '')"
  ab="$(git -C "$d" rev-list --left-right --count "origin/$branch...HEAD" 2>/dev/null || echo '0 0')"
  behind="${ab%%[[:space:]]*}"; ahead="${ab##*[[:space:]]}"
  case "$behind" in ''|*[!0-9]*) behind=0 ;; esac
  case "$ahead"  in ''|*[!0-9]*) ahead=0  ;; esac
  dirty="$(git -C "$d" status --porcelain 2>/dev/null | wc -l | tr -d ' ')"
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$name" "$branch" "$head" "$remote" "$behind" "$ahead" "$dirty" >> "$TMP"
done

RC=0
python3 - "$TMP" "$TS" "$MANIFEST" "$LATEST" <<'PY' || RC=$?
import json, sys
tmp, ts, manifest, latest = sys.argv[1:5]
rows = []
for line in open(tmp, encoding="utf-8"):
    p = line.rstrip("\n").split("\t")
    if len(p) != 7:
        continue
    name, branch, head, remote, behind, ahead, dirty = p
    rows.append({
        "repo": name, "branch": branch, "head": head, "remote_head": remote,
        "behind": int(behind or 0), "ahead": int(ahead or 0),
        "dirty_files": int(dirty or 0),
    })
rows.sort(key=lambda r: r["repo"])
rec = {
    "timestamp": ts,
    "repositories": len(rows),
    "drift": [r["repo"] for r in rows if r["behind"] > 0],
    "unpushed": [r["repo"] for r in rows if r["ahead"] > 0],
    "dirty": [r["repo"] for r in rows if r["dirty_files"] > 0],
    "repos": {r["repo"]: r for r in rows},
}
with open(manifest, "a", encoding="utf-8") as f:
    f.write(json.dumps(rec, sort_keys=True) + "\n")

with open(latest, "w", encoding="utf-8") as f:
    f.write("# Org-Snapshot A-TownChain-Okosystems\n\n")
    f.write(f"Zeitpunkt (UTC): {ts}\n")
    f.write(f"Repositories: {len(rows)}\n")
    f.write(f"Drift (Remote voraus): {', '.join(rec['drift']) or 'nein'}\n")
    f.write(f"Unpushed: {', '.join(rec['unpushed']) or 'nein'}\n")
    f.write(f"Uncommitted: {', '.join(rec['dirty']) or 'nein'}\n\n")
    f.write("| Repo | Branch | HEAD | Remote-HEAD |\n|---|---|---|---|\n")
    for r in rows:
        f.write(f"| {r['repo']} | {r['branch']} | `{r['head'][:12]}` | `{r['remote_head'][:12]}` |\n")
    f.write("\nArbeitsnachweis, kein Ersatz fuer die GitHub-Remote-Wahrheit.\n")

print(f"repos={len(rows)} drift={len(rec['drift'])} unpushed={len(rec['unpushed'])} dirty={len(rec['dirty'])}")
sys.exit(1 if (rec["drift"]) else 0)
PY
rm -f "$TMP"
[ "$QUIET" -eq 0 ] && echo "→ $MANIFEST, $LATEST"
if [ "$CHECK" -eq 1 ]; then exit $RC; fi
exit 0
