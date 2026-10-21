#!/usr/bin/env python3
"""Build a provenance-bound per-file purpose catalog for every repository in the org manifest.

This is deterministic discovery metadata, not a semantic proof or a new capability registry.
Each tracked file is represented, including docs, configuration, assets, tests, and archives.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import subprocess
import sys
from pathlib import Path

def git(root: Path, *args: str) -> str:
    return subprocess.run(["git", "-C", str(root), *args], check=True,
                          capture_output=True, text=True).stdout.strip()

def classify(path: str) -> tuple[str, str]:
    p = path.lower()
    name = p.rsplit("/", 1)[-1]
    if "/archive/" in p or "/archiv" in p or p.startswith("archive/"):
        return "historical_archive", "Historical or archived material; not assumed to be a live implementation."
    if "/test" in p or p.startswith("tests/") or "/tests/" in p or name.startswith("test_") or ".spec." in name or ".test." in name:
        return "test", "Test or fixture material used to exercise other code."
    if p.startswith(".github/") or "/.github/" in p or "workflow" in name:
        return "ci_configuration", "CI, automation, or repository workflow configuration."
    if name in {"readme", "readme.md", "readme.rst"}:
        return "documentation_entrypoint", "Repository or directory overview and usage documentation."
    if p.endswith((".md", ".rst", ".adoc")):
        return "documentation", "Human-readable documentation, specification, or design record."
    if p.endswith((".json", ".yaml", ".yml", ".toml", ".ini", ".cfg", ".conf", ".xml")):
        if "schema" in name:
            return "schema", "Machine-readable schema or validation contract."
        if "manifest" in name:
            return "manifest", "Machine-readable inventory, package, or repository manifest."
        return "configuration_or_data", "Configuration or structured data consumed by tools or runtime components."
    if p.endswith((".rs", ".py", ".js", ".jsx", ".mjs", ".cjs", ".ts", ".tsx", ".go", ".java", ".cs", ".c", ".h", ".cc", ".cpp", ".hpp", ".swift", ".kt", ".kts", ".rb", ".php", ".sh", ".ps1")):
        return "source_code", "Executable source, library, script, or tooling implementation."
    if p.endswith((".png", ".jpg", ".jpeg", ".gif", ".svg", ".webp", ".ico", ".ttf", ".otf", ".woff", ".woff2")):
        return "media_asset", "Image, icon, font, or other media resource."
    if p.endswith((".lock", ".sum")):
        return "dependency_lock", "Pinned dependency or package resolution data."
    if p.endswith((".bin", ".dat", ".wasm", ".o", ".a", ".so", ".dll", ".exe")):
        return "binary_or_build_artifact", "Binary data or build artifact; semantic purpose requires format-specific inspection."
    return "other_tracked_file", "Tracked file not classified by the current extension/path rules."

def documented_purpose(path: str, raw: bytes, role: str, fallback: str) -> tuple[str, str, str]:
    try:
        text = raw[:200_000].decode("utf-8")
    except UnicodeDecodeError:
        return fallback, "path_extension_heuristic", "LOW"
    lines = text.splitlines()
    if path.lower().endswith((".md", ".rst", ".adoc")):
        heading = next((re.sub(r"^#+\s*", "", line).strip() for line in lines if line.startswith("#") and line.strip("# ").strip()), None)
        paragraph = []
        started = False
        for line in lines:
            line = line.strip()
            if not line or line.startswith("#"):
                if started:
                    break
                continue
            paragraph.append(line)
            started = True
            if len(" ".join(paragraph)) > 280:
                break
        if heading and paragraph:
            return (f"{heading}: {' '.join(paragraph)[:360]}", "markdown_heading_and_intro", "HIGH")
        if heading:
            return (heading, "markdown_heading", "MEDIUM")
    # Use only explicit leading comments/docstrings, never arbitrary implementation text.
    for line in lines[:18]:
        s = line.strip()
        if not s:
            continue
        if s.startswith(("#!", "//", "///", "//!")):
            value = re.sub(r"^(?:#!|///?|//!)+\s*", "", s).strip()
            if value and not value.startswith("["):
                return value[:360], "leading_source_comment", "MEDIUM"
        if s.startswith('"""') or s.startswith("'''"):
            value = s.strip('"\' ').strip()
            if len(value) > 8:
                return value[:360], "leading_docstring", "HIGH"
        if s.startswith(("/*", "*")):
            value = re.sub(r"^/\*+|^\*+|\*/$", "", s).strip()
            if len(value) > 12:
                return value[:360], "leading_source_comment", "MEDIUM"
        if not s.startswith(("#", "//", "///", "//!")):
            break
    return fallback, "path_and_extension_heuristic", "LOW"

def build_catalog(workspace: Path, manifest: dict, census: dict) -> dict:
    census_by_repo = {r.get("repository"): r for r in census.get("repositories", [])}
    repos_out = []
    total_files = 0
    blocked_repos = []
    excluded_private = []
    for item in sorted(manifest["repositories"], key=lambda x: x["name"].lower()):
        name = item["name"]
        repo_name = f'{manifest["owner"]}/{name}'
        root = workspace / name
        entry = {"repository": repo_name, "name": name, "archived": bool(item.get("archived", False)),
                 "private": bool(item.get("private", False)), "expected_role": item.get("role", "UNCLASSIFIED"),
                 "source_sha": None, "status": "BLOCKED", "tracked_file_count": 0, "files": [], "residuals": []}
        if not root.is_dir() or not (root / ".git").exists():
            if item.get("private", False):
                entry["status"] = "EXCLUDED_PRIVATE_NO_CI_ACCESS"
                entry["residuals"].append("PRIVATE_REPOSITORY_EXCLUDED_NO_CI_ACCESS")
                excluded_private.append(repo_name)
            else:
                entry["residuals"].append("CHECKOUT_MISSING")
                blocked_repos.append(repo_name)
            repos_out.append(entry)
            continue
        try:
            sha = git(root, "rev-parse", "HEAD")
            entry["source_sha"] = sha
            expected = census_by_repo.get(repo_name, {}).get("source_sha")
            if expected and expected != sha:
                entry["residuals"].append("CENSUS_SOURCE_SHA_MISMATCH")
                blocked_repos.append(repo_name)
                repos_out.append(entry)
                continue
            if git(root, "status", "--porcelain", "--untracked-files=all"):
                entry["residuals"].append("WORKTREE_DIRTY")
                blocked_repos.append(repo_name)
                repos_out.append(entry)
                continue
            index_entries = []
            for record in git(root, "ls-files", "--stage", "-z").split("\0"):
                if not record:
                    continue
                metadata, rel = record.split("\t", 1)
                mode, blob_sha, _stage = metadata.split(" ", 2)
                index_entries.append((rel, mode, blob_sha))
            symbol_files = {}
            report = (census_by_repo.get(repo_name) or {}).get("report") or {}
            for scanned in report.get("files", []):
                symbol_files[scanned.get("path", "")] = scanned.get("candidate_public_symbols", [])
            for rel, mode, blob_sha in sorted(index_entries):
                file_path = root / rel
                try:
                    if mode == "160000":
                        raw = b""
                    elif mode == "120000" and file_path.is_symlink():
                        raw = os.readlink(file_path).encode("utf-8", errors="surrogateescape")
                    else:
                        raw = file_path.read_bytes()
                except OSError:
                    entry["residuals"].append(f"FILE_READ_FAILED:{rel}")
                    continue
                role, fallback = classify(rel)
                if mode == "160000":
                    role, fallback = "git_submodule_reference", "Git submodule pointer; referenced repository content is inventoried separately if present in the manifest."
                purpose, basis, confidence = documented_purpose(rel, raw, role, fallback)
                symbols = symbol_files.get(rel, [])
                function_summary = (
                    "Candidate declarations: " + ", ".join(sorted({s.get("name", "") for s in symbols if s.get("name")}))[:500]
                    if symbols else "No candidate public symbols were emitted by the lexical scanner; this does not prove the file has no functions."
                )
                entry["files"].append({
                    "path": rel,
                    "git_blob_sha": blob_sha,
                    "git_mode": mode,
                    "file_sha256": hashlib.sha256(raw).hexdigest(),
                    "size_bytes": len(raw),
                    "file_role": role,
                    "purpose": purpose,
                    "purpose_basis": basis,
                    "purpose_confidence": confidence,
                    "candidate_function_summary": function_summary,
                    "candidate_public_symbols": symbols,
                    "functional_verification": "NOT_PERFORMED",
                    "status": "DISCOVERY_ONLY",
                    "canonical_implementation_claim": "NOT_ASSERTED",
                })
            entry["tracked_file_count"] = len(paths)
            entry["status"] = "RESIDUAL" if not entry["residuals"] else "BLOCKED"
            if entry["status"] == "BLOCKED":
                blocked_repos.append(repo_name)
            total_files += len(entry["files"])
        except (OSError, subprocess.CalledProcessError, ValueError) as exc:
            entry["residuals"].append(f"REPOSITORY_AUDIT_ERROR:{type(exc).__name__}")
            blocked_repos.append(repo_name)
        repos_out.append(entry)
    return {
        "schema_id": "ATC-ORG-FILE-FUNCTION-CATALOG-001",
        "schema_version": "1.0.0",
        "owner": manifest["owner"],
        "catalog_source_commit": git(workspace / "a-townchain-ecosystem", "rev-parse", "HEAD") if (workspace / "a-townchain-ecosystem" / ".git").exists() else None,
        "status": "BLOCKED" if blocked_repos else "RESIDUAL",
        "functional_verification": "NOT_PERFORMED",
        "summary": {"repositories_expected": len(manifest["repositories"]), "repositories_scanned": sum(r["status"] == "RESIDUAL" for r in repos_out),
                    "repositories_blocked": len(set(blocked_repos)), "repositories_excluded_private": len(excluded_private),
                    "tracked_files_cataloged": total_files, "files_by_role": {
                        role: sum(1 for r in repos_out for f in r["files"] if f["file_role"] == role)
                        for role in sorted({f["file_role"] for r in repos_out for f in r["files"]})}},
        "blocked_repositories": sorted(set(blocked_repos)),
        "excluded_private_repositories": excluded_private,
        "policy": ["Every tracked file from each clean checkout is listed; no extension-based file exclusion.",
                   "Purpose is extracted from explicit documentation/comments when available, otherwise marked as a low-confidence path/extension inference.",
                   "Lexical candidate symbols are discovery data, not function contracts or functional verification.",
                   "Mirrored umbrella-repository files and archived repositories are inventoried but do not become canonical implementation owners by implication.",
                   "Missing checkout, dirty worktree, SHA mismatch, or unreadable file is recorded and keeps the result BLOCKED."],
        "repositories": repos_out,
    }

def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--workspace", required=True, type=Path)
    parser.add_argument("--manifest", required=True, type=Path)
    parser.add_argument("--census", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    manifest = json.loads(args.manifest.read_text(encoding="utf-8"))
    census = json.loads(args.census.read_text(encoding="utf-8"))
    if not isinstance(manifest.get("repositories"), list) or not manifest.get("owner"):
        parser.error("manifest requires owner and repositories array")
    result = build_catalog(args.workspace.resolve(), manifest, census)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({"status": result["status"], **result["summary"], "output": str(args.output)}, sort_keys=True))
    return 0 if result["status"] == "RESIDUAL" else 2

if __name__ == "__main__":
    sys.exit(main())
