#!/usr/bin/env python3
"""sync_modules.py - Mirror-Synchronisation fuer a-townchain-ecosystem.

Ersetzt die frueher referenzierte, im operativen Repo fehlende Fassung.
Regeln (Beschluss 09.11.2026, siehe forensics/entscheidungen-2026-11-09.md):
  D1  Der Integrations-Mirror enthaelt buildrelevanten Code. Doku, Normen,
      Wiki-Inhalte und Evidenzdateien werden NICHT gespiegelt.
  D7  Metadaten (README/STATUS/CHANGELOG/FILE_REGISTER) folgen dem Kanon.
  Kein Loeschen: Dateien, die nur im Mirror liegen, werden nur gemeldet,
      niemals automatisch entfernt (Orphan-Politik D5).

Aufruf:
  python3 scripts/sync_modules.py --check            # nur Bericht, Exit 1 bei Drift
  python3 scripts/sync_modules.py --check --component atc-vm
  python3 scripts/sync_modules.py --sync             # Kanon -> Mirror kopieren
  python3 scripts/sync_modules.py --sync --dry-run   # zeigen, was passieren wuerde

Der Kanon liegt in den Produkt-Repos (AD-017). Der Pfad dorthin wird ueber
--products gesetzt (Standard: ../  neben dem Mirror bzw. $ATC_PRODUCTS_DIR).
"""

from __future__ import annotations

import argparse
import os
import subprocess
import sys

MIRROR_PREFIX = 'components'
# Doku/Normen/Wiki/Evidenz gehoeren nicht in den Code-Mirror (D1).
# Ausnahme: der Docs-Hub spiegelt seine eigenen docs/ und wiki/ Inhalte.
SCOPE_EXCLUDE = ('docs/', 'standards/', 'wiki/', '.atc/evidence/')
HUB_COMPONENT = 'a-townchain-os-docs'


def git(repo: str, *args: str) -> str:
    cmd = ['git', '-C', repo, '-c', 'core.quotePath=false'] + list(args)
    res = subprocess.run(cmd, capture_output=True, text=True)
    if res.returncode != 0:
        raise SystemExit('git %s fehlgeschlagen in %s: %s' % (' '.join(args), repo, res.stderr.strip()))
    return res.stdout


def blob_map(repo: str, prefix: str = '') -> dict:
    """Pfad -> Blob-SHA aus dem committeten HEAD (keine Arbeitsdateien)."""
    out = git(repo, 'ls-tree', '-r', 'HEAD', '--format=%(objectname) %(path)')
    result = {}
    for line in out.splitlines():
        sha, _, path = line.partition(' ')
        if prefix:
            if not path.startswith(prefix + '/'):
                continue
            path = path[len(prefix) + 1:]
        result[path] = sha
    return result


def in_scope(component: str, path: str) -> bool:
    if component == HUB_COMPONENT:
        return True
    return not any(path.startswith(rule) for rule in SCOPE_EXCLUDE)


def compare(component: str, products_dir: str, mirror_dir: str) -> dict:
    canon_repo = os.path.join(products_dir, component)
    if not os.path.isdir(os.path.join(canon_repo, '.git')):
        return {'state': 'NO_CANON', 'detail': 'Produkt-Klon fehlt: %s' % canon_repo}
    canon = {p: s for p, s in blob_map(canon_repo).items() if in_scope(component, p)}
    mirror = blob_map(mirror_dir, '%s/%s' % (MIRROR_PREFIX, component))
    mirror_scoped = {p: s for p, s in mirror.items() if in_scope(component, p)}
    differing = sorted(p for p in set(canon) & set(mirror_scoped) if canon[p] != mirror_scoped[p])
    only_canon = sorted(set(canon) - set(mirror_scoped))
    only_mirror = sorted(set(mirror_scoped) - set(canon))
    state = 'IN_SYNC' if not (differing or only_canon or only_mirror) else 'DRIFT'
    return {
        'state': state,
        'differing': differing,
        'only_canon': only_canon,
        'only_mirror': only_mirror,
        'out_of_scope_in_mirror': sorted(p for p in mirror if not in_scope(component, p)),
    }


def sync(component: str, products_dir: str, mirror_dir: str, dry_run: bool) -> dict:
    report = compare(component, products_dir, mirror_dir)
    if report['state'] != 'DRIFT':
        return report
    canon_repo = os.path.join(products_dir, component)
    target_root = os.path.join(mirror_dir, MIRROR_PREFIX, component)
    copied = []
    for path in report['differing'] + report['only_canon']:
        src = os.path.join(canon_repo, path)
        dst = os.path.join(target_root, path)
        with open(src, 'rb') as fh:
            data = fh.read()
        if dry_run:
            copied.append(path)
            continue
        os.makedirs(os.path.dirname(dst) or target_root, exist_ok=True)
        with open(dst, 'wb') as fh:
            fh.write(data)
        copied.append(path)
    report['copied'] = copied
    return report


def main() -> int:
    ap = argparse.ArgumentParser(description='Kanon-Produkt-Repos -> Integrations-Mirror')
    ap.add_argument('--check', action='store_true', help='nur Bericht (Exit 1 bei Drift)')
    ap.add_argument('--sync', action='store_true', help='Kanon in den Mirror kopieren (kein Loeschen)')
    ap.add_argument('--dry-run', action='store_true', help='mit --sync: nur anzeigen')
    ap.add_argument('--component', action='append', help='nur diese Komponente(n)')
    ap.add_argument('--products', default=os.environ.get('ATC_PRODUCTS_DIR', os.path.join('..', '')),
                    help='Verzeichnis mit den Produkt-Klonen (Standard: ../)')
    ap.add_argument('--mirror', default='.', help='Mirror-Repo (Standard: aktuelles Verzeichnis)')
    args = ap.parse_args()
    if not (args.check or args.sync):
        ap.error('Bitte --check oder --sync angeben.')

    comp_root = os.path.join(args.mirror, MIRROR_PREFIX)
    components = sorted(d for d in os.listdir(comp_root) if os.path.isdir(os.path.join(comp_root, d)))
    if args.component:
        components = [c for c in components if c in set(args.component)]
    if not components:
        print('Keine Komponenten gefunden.')
        return 2

    drift = []
    for comp in components:
        rep = sync(comp, args.products, args.mirror, args.dry_run) if args.sync else compare(comp, args.products, args.mirror)
        if rep['state'] == 'NO_CANON':
            print('%-24s %-8s %s' % (comp, 'NO_CANON', rep['detail']))
            continue
        if rep['state'] == 'IN_SYNC':
            print('%-24s IN_SYNC' % comp)
            continue
        drift.append(comp)
        print('%-24s DRIFT    diff=%d fehlt=%d orphan=%d%s' % (
            comp, len(rep['differing']), len(rep['only_canon']), len(rep['only_mirror']),
            ' kopiert=%d' % len(rep.get('copied', [])) if args.sync else ''))
        for p in rep['only_mirror'][:3]:
            print('    orphan (bleibt, D5): %s' % p)

    print('\n%d/%d Komponenten in Scope identisch.' % (len(components) - len(drift), len(components)))
    if drift:
        print('Drift: %s' % ', '.join(drift))
    return 1 if drift and args.check else 0


if __name__ == '__main__':
    sys.exit(main())
