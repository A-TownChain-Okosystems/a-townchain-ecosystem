#!/usr/bin/env python3
"""Exact-SHA public symbol census for the ATC organization.

This is a discovery aid, not a compiler-grade parser or proof of completeness.
Every emitted symbol is tied to the repository commit SHA and source path.
Unmapped symbols remain RESIDUAL until reviewed against the component catalog.
"""
from __future__ import annotations

import argparse
import concurrent.futures
import json
import os
import re
import sys
import time
from pathlib import Path
from urllib.error import HTTPError, URLError
from urllib.parse import quote
from urllib.request import Request, urlopen

ORG = "A-TownChain-Okosystems"
EXTENSIONS = {".rs", ".py", ".js", ".jsx", ".ts", ".tsx", ".go", ".c", ".h", ".cc", ".cpp", ".hpp", ".sol", ".move", ".atc"}
EXCLUDED_PARTS = {"vendor", "node_modules", "target", "dist", "build", ".git", ".venv", "venv", "__pycache__"}
MAX_FILE_BYTES = 1_000_000
USER_AGENT = "ATC-Source-Symbol-Census/1.0"

PATTERNS = {
    "rust": [
        ("function", re.compile(r"(?m)^\s*pub(?:\([^)]*\))?\s+(?:(?:async|unsafe|const)\s+)*fn\s+([A-Za-z_][A-Za-z0-9_]*)")),
        ("type", re.compile(r"(?m)^\s*pub(?:\([^)]*\))?\s+(?:struct|enum|trait|type)\s+([A-Za-z_][A-Za-z0-9_]*)")),
        ("constant", re.compile(r"(?m)^\s*pub(?:\([^)]*\))?\s+(?:const|static)\s+([A-Za-z_][A-Za-z0-9_]*)")),
    ],
    "python": [
        ("function", re.compile(r"(?m)^(?:async\s+)?def\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(")),
        ("class", re.compile(r"(?m)^class\s+([A-Za-z_][A-Za-z0-9_]*)\b")),
    ],
    "javascript": [
        ("export", re.compile(r"(?m)^\s*export\s+(?:default\s+)?(?:async\s+)?(?:function|class)\s+([A-Za-z_$][\w$]*)")),
        ("export", re.compile(r"(?m)^\s*export\s+(?:const|let|var)\s+([A-Za-z_$][\w$]*)")),
        ("export", re.compile(r"(?m)^\s*module\.exports(?:\.([A-Za-z_$][\w$]*))?\s*=")),
    ],
    "typescript": [
        ("export", re.compile(r"(?m)^\s*export\s+(?:default\s+)?(?:async\s+)?(?:function|class|interface|type|enum)\s+([A-Za-z_$][\w$]*)")),
        ("export", re.compile(r"(?m)^\s*export\s+(?:const|let|var)\s+([A-Za-z_$][\w$]*)")),
    ],
    "go": [
        ("function", re.compile(r"(?m)^func\s+(?:\([^)]*\)\s*)?([A-Z][A-Za-z0-9_]*)\s*\(")),
        ("type", re.compile(r"(?m)^type\s+([A-Z][A-Za-z0-9_]*)\s+(?:struct|interface|\[)")),
    ],
    "c": [
        ("declaration", re.compile(r"(?m)^\s*(?:extern\s+)?[A-Za-z_][\w\s*]+\s+([A-Za-z_][A-Za-z0-9_]*)\s*\([^;{}]*\)\s*;")),
    ],
    "solidity": [
        ("function", re.compile(r"(?m)^\s*function\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(")),
        ("entrypoint", re.compile(r"(?m)^\s*(?:constructor|fallback|receive)\s*\(")),
        ("event", re.compile(r"(?m)^\s*event\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(")),
        ("error", re.compile(r"(?m)^\s*error\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(")),
    ],
    "move": [
        ("function", re.compile(r"(?m)^\s*(?:public(?:\([^)]*\))?\s+)?(?:entry\s+)?fun\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(")),
    ],
    "atc": [
        ("directive", re.compile(r"(?m)^\s*(?:@|#)?(?:entry|export|function|fn|service|command|event|syscall|route)\s+([A-Za-z_][A-Za-z0-9_.:-]*)")),
    ],
}

def request_bytes(url: str, token: str | None = None) -> bytes:
    headers = {"User-Agent": USER_AGENT, "Accept": "application/vnd.github+json"}
    if token:
        headers["Authorization"] = "Bearer " + token
    req = Request(url, headers=headers)
    with urlopen(req, timeout=30) as response:
        return response.read()

def get_tree(repo: str, sha: str, token: str | None) -> list[dict]:
    url = f"https://api.github.com/repos/{repo}/git/trees/{sha}?recursive=1"
    data = json.loads(request_bytes(url, token))
    if data.get("truncated"):
        raise RuntimeError(f"{repo}@{sha}: GitHub returned a truncated recursive tree")
    return data.get("tree", []), data.get("sha")

def language(ext: str) -> str:
    if ext == ".rs": return "rust"
    if ext == ".py": return "python"
    if ext in {".js", ".jsx"}: return "javascript"
    if ext in {".ts", ".tsx"}: return "typescript"
    if ext == ".go": return "go"
    if ext in {".c", ".h", ".cc", ".cpp", ".hpp"}: return "c"
    if ext == ".sol": return "solidity"
    if ext == ".move": return "move"
    return "atc"

def scan_file(task: tuple[str, str, str, str, str | None]) -> tuple[list[dict], dict | None]:
    repo, sha, path, blob_sha, token = task
    ext = Path(path).suffix.lower()
    url = f"https://raw.githubusercontent.com/{repo}/{sha}/{quote(path, safe='/')}"
    try:
        raw = request_bytes(url, None)
        if len(raw) > MAX_FILE_BYTES:
            return [], {"repository": repo, "path": path, "blob_sha": blob_sha, "error": "FILE_OVER_1MB"}
        source = raw.decode("utf-8", errors="replace")
    except (HTTPError, URLError, TimeoutError, OSError) as exc:
        return [], {"repository": repo, "path": path, "blob_sha": blob_sha, "error": type(exc).__name__ + ": " + str(exc)[:180]}
    lang = language(ext)
    patterns = PATTERNS.get(lang, [])
    found: list[dict] = []
    for kind, pattern in patterns:
        for match in pattern.finditer(source):
            name = match.group(1) if match.lastindex else match.group(0).strip()
            line = source.count("\n", 0, match.start()) + 1
            start = source.rfind("\n", 0, match.start()) + 1
            end = source.find("\n", match.start())
            snippet = source[start:end if end >= 0 else len(source)].strip()[:240]
            # Keep private/internal Python identifiers out of the public API census.
            if lang == "python" and name.startswith("_"):
                continue
            found.append({
                "repository": repo, "source_sha": sha, "path": path,
                "blob_sha": blob_sha, "language": lang, "kind": kind,
                "symbol": name, "line": line, "declaration": snippet,
                "mapping_status": "UNMAPPED_RESIDUAL"
            })
    return found, None

def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--inventory", default="docs/architecture/ATC-SOURCE-INVENTORY-001.json")
    parser.add_argument("--output", default="docs/architecture/ATC-SOURCE-SYMBOLS-001.json")
    parser.add_argument("--workers", type=int, default=12)
    parser.add_argument("--max-files", type=int, default=0, help="Optional diagnostic cap; 0 scans all eligible files.")
    args = parser.parse_args()
    inventory = json.loads(Path(args.inventory).read_text(encoding="utf-8"))
    token = os.environ.get("GITHUB_TOKEN") or os.environ.get("GH_TOKEN")
    repos = [r for r in inventory["repositories"] if r.get("archive_state") == "ACTIVE"]
    tasks: list[tuple[str, str, str, str, str | None]] = []
    errors: list[dict] = []
    counts: dict[str, dict] = {}
    for repo_entry in repos:
        repo = repo_entry["full_name"]
        sha = repo_entry["head_sha"]
        try:
            tree, observed_tree_sha = get_tree(repo, sha, token)
            expected_tree_sha = repo_entry.get("tree_sha")
            if expected_tree_sha and observed_tree_sha != expected_tree_sha:
                raise RuntimeError(f"{repo}@{sha}: tree SHA mismatch: inventory={expected_tree_sha}, API={observed_tree_sha}")
        except Exception as exc:
            errors.append({"repository": repo, "source_sha": sha, "stage": "TREE_FETCH", "error": str(exc)})
            continue
        eligible = []
        for item in tree:
            path = item.get("path", "")
            if item.get("type") != "blob" or Path(path).suffix.lower() not in EXTENSIONS:
                continue
            if any(part in EXCLUDED_PARTS for part in Path(path).parts):
                continue
            eligible.append(item)
        counts[repo] = {"source_sha": sha, "tree_sha": repo_entry["tree_sha"], "eligible_source_files": len(eligible)}
        if args.max_files:
            eligible = eligible[:args.max_files]
        tasks.extend((repo, sha, item["path"], item.get("sha", ""), token) for item in eligible)

    symbols: list[dict] = []
    failures: list[dict] = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=max(1, args.workers)) as pool:
        futures = [pool.submit(scan_file, task) for task in tasks]
        for future in concurrent.futures.as_completed(futures):
            try:
                found, failure = future.result()
                symbols.extend(found)
                if failure:
                    failures.append(failure)
            except Exception as exc:
                failures.append({"error": repr(exc), "stage": "SCAN"})
    symbols.sort(key=lambda item: (item["repository"], item["path"], item["line"], item["symbol"]))
    output = {
        "schema_id": "ATC-SOURCE-SYMBOLS-001",
        "schema_version": "1.0.0",
        "status": "DISCOVERY_SCAN_RESIDUAL",
        "generated_at_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "inventory_schema": inventory.get("schema_id"),
        "inventory_observed_at": inventory.get("observed_at"),
        "active_repositories_expected": len(repos),
        "active_repositories_tree_scanned": len(counts),
        "eligible_source_files": sum(v["eligible_source_files"] for v in counts.values()),
        "files_attempted": len(tasks),
        "symbols_detected": len(symbols),
        "symbols_mapped": 0,
        "symbols_unmapped_residual": len(symbols),
        "fetch_errors": len(errors) + len(failures),
        "method_limitations": [
            "Regex-based symbol discovery is not a language compiler or AST and may miss macros, generated code, overloads, nested declarations, framework routes, runtime registration and dynamic exports.",
            "This is a candidate symbol inventory, not a claim of complete public API coverage.",
            "Every symbol is UNMAPPED_RESIDUAL until reconciled to the canonical component/function catalog or a reviewed exclusion.",
            "Tree/source refs are pinned to the inventory's recorded commit SHAs; rerun the census when any repository head changes."
        ],
        "repository_counts": counts,
        "symbols": symbols,
        "errors": errors + failures
    }
    Path(args.output).parent.mkdir(parents=True, exist_ok=True)
    Path(args.output).write_text(json.dumps(output, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(json.dumps({k: output[k] for k in ("status", "active_repositories_expected", "active_repositories_tree_scanned", "eligible_source_files", "files_attempted", "symbols_detected", "symbols_mapped", "fetch_errors")}, indent=2))
    # Fail closed when the census did not cover every active repo or any file could not be scanned.
    return 1 if len(counts) != len(repos) or errors or failures else 0

if __name__ == "__main__":
    raise SystemExit(main())
