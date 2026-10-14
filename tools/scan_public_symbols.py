#!/usr/bin/env python3
"""Deterministic discovery scan for source symbols.

This is a candidate inventory generator, not a compiler/parser and not proof of
functional correctness. It records exact source SHA supplied by the caller and
per-file SHA-256. Every result remains DISCOVERY_ONLY until reconciled with
normative contracts and tests.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
import sys
from collections import Counter
from pathlib import Path

SKIP_DIRS = {
    ".git", ".hg", ".svn", "target", "node_modules", "vendor", "dist",
    "build", ".venv", "venv", "__pycache__", "coverage", "generated",
}
EXTENSIONS = {
    ".rs": "rust", ".py": "python", ".js": "javascript", ".jsx": "javascript",
    ".mjs": "javascript", ".cjs": "javascript", ".ts": "typescript",
    ".tsx": "typescript", ".go": "go", ".java": "java", ".cs": "csharp",
    ".c": "c", ".h": "c", ".cc": "cpp", ".hh": "cpp", ".cpp": "cpp",
    ".hpp": "cpp", ".swift": "swift", ".kt": "kotlin", ".kts": "kotlin",
    ".rb": "ruby", ".php": "php",
}
PATTERNS = {
    "rust": [
        ("function", re.compile(r"^[ \t]*pub(?:\([^)]*\))?\s+(?:async\s+)?(?:unsafe\s+)?(?:extern\s+\"[^\"]+\"\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)", re.M)),
        ("type", re.compile(r"^[ \t]*pub(?:\([^)]*\))?\s+(?:struct|enum|trait|type|union)\s+([A-Za-z_][A-Za-z0-9_]*)", re.M)),
        ("macro", re.compile(r"^[ \t]*\#\[macro_export\]\s*\n\s*macro_rules!\s+([A-Za-z_][A-Za-z0-9_]*)", re.M)),
    ],
    "python": [
        ("function", re.compile(r"^[ \t]*(?:async\s+)?def\s+([A-Za-z_][A-Za-z0-9_]*)", re.M)),
        ("type", re.compile(r"^[ \t]*class\s+([A-Za-z_][A-Za-z0-9_]*)", re.M)),
    ],
    "javascript": [
        ("function", re.compile(r"^[ \t]*export\s+(?:default\s+)?(?:async\s+)?function\*?\s+([A-Za-z_$][\w$]*)", re.M)),
        ("type", re.compile(r"^[ \t]*export\s+(?:default\s+)?(?:abstract\s+)?class\s+([A-Za-z_$][\w$]*)", re.M)),
        ("value", re.compile(r"^[ \t]*export\s+(?:const|let|var)\s+([A-Za-z_$][\w$]*)", re.M)),
    ],
    "typescript": [
        ("function", re.compile(r"^[ \t]*export\s+(?:default\s+)?(?:declare\s+)?(?:async\s+)?function\*?\s+([A-Za-z_$][\w$]*)", re.M)),
        ("function", re.compile(r"^[ \t]*(?:(?:public|static|async|override|readonly|abstract)\s+)*([A-Za-z_$][\w$]*)\s*\([^;\n]*\)\s*:\s*.*\{\s*$", re.M)),
        ("type", re.compile(r"^[ \t]*export\s+(?:default\s+)?(?:abstract\s+)?(?:class|interface|type|enum)\s+([A-Za-z_$][\w$]*)", re.M)),
        ("value", re.compile(r"^[ \t]*export\s+(?:const|let|var)\s+([A-Za-z_$][\w$]*)", re.M)),
    ],
    "go": [
        ("function", re.compile(r"^func\s+(?:\([^\n)]*\)\s*)?([A-Z][A-Za-z0-9_]*)\s*\(", re.M)),
        ("type", re.compile(r"^type\s+([A-Z][A-Za-z0-9_]*)\s+(?:struct|interface|\[|[A-Za-z_*])", re.M)),
    ],
    "java": [
        ("type", re.compile(r"^[ \t]*(?:(?:public|protected)\s+)?(?:(?:abstract|final|sealed|static)\s+)*(?:class|interface|enum|record)\s+([A-Za-z_$][\w$]*)", re.M)),
        ("function", re.compile(r"^[ \t]*(?:public|protected)\s+(?:(?:static|final|abstract|synchronized|native|default)\s+)*[\w<>?,.\[\] ]+\s+([A-Za-z_$][\w$]*)\s*\(", re.M)),
    ],
    "csharp": [
        ("type", re.compile(r"^[ \t]*public\s+(?:(?:abstract|sealed|static|partial)\s+)*(?:class|interface|struct|enum|record)\s+([A-Za-z_][\w]*)", re.M)),
        ("function", re.compile(r"^[ \t]*public\s+(?:(?:static|async|virtual|override|abstract|sealed|extern|unsafe|new)\s+)*[\w<>?,.\[\] ]+\s+([A-Za-z_][\w]*)\s*\(", re.M)),
    ],
    "c": [
        ("function", re.compile(r"^[ \t]*(?:extern\s+)?(?:const\s+)?[A-Za-z_][\w\s*]*\s+([A-Za-z_][\w]*)\s*\([^;{}]*\)\s*\{", re.M)),
    ],
    "cpp": [
        ("function", re.compile(r"^[ \t]*(?:extern\s+\"C\"\s+)?(?:inline\s+|static\s+|virtual\s+|constexpr\s+|friend\s+|explicit\s+)*[\w:<>~,\s*&]+\s+([A-Za-z_][\w:]*)\s*\([^;{}]*\)\s*(?:const\s*)?(?:noexcept\s*)?\{", re.M)),
        ("type", re.compile(r"^[ \t]*(?:class|struct|enum\s+class|enum)\s+([A-Za-z_][\w]*)", re.M)),
    ],
    "swift": [
        ("function", re.compile(r"^[ \t]*(?:public|open)\s+(?:static\s+|class\s+|override\s+|mutating\s+|async\s+|throws\s+)*func\s+([A-Za-z_][\w]*)", re.M)),
        ("type", re.compile(r"^[ \t]*(?:public|open)\s+(?:final\s+)?(?:class|struct|enum|protocol|actor)\s+([A-Za-z_][\w]*)", re.M)),
    ],
    "kotlin": [
        ("function", re.compile(r"^[ \t]*(?:public\s+)?(?:suspend\s+|inline\s+|operator\s+|override\s+|open\s+|final\s+)*fun\s+(?:<[^>]+>\s*)?([A-Za-z_][\w]*)", re.M)),
        ("type", re.compile(r"^[ \t]*(?:public\s+)?(?:data\s+|sealed\s+|abstract\s+|open\s+)?(?:class|interface|object|enum\s+class)\s+([A-Za-z_][\w]*)", re.M)),
    ],
    "ruby": [
        ("function", re.compile(r"^[ \t]*def\s+([A-Za-z_][\w!?=]*)", re.M)),
        ("type", re.compile(r"^[ \t]*class\s+([A-Z][A-Za-z0-9_:]*)", re.M)),
    ],
    "php": [
        ("function", re.compile(r"^[ \t]*(?:(?:public|protected)\s+)?(?:static\s+|final\s+|abstract\s+)?function\s+&?([A-Za-z_][\w]*)", re.M)),
        ("type", re.compile(r"^[ \t]*(?:abstract\s+|final\s+)?(?:class|interface|trait|enum)\s+([A-Za-z_][\w]*)", re.M)),
    ],
}


# Rust private helpers are recorded separately from public API candidates.
RUST_INTERNAL_FUNCTION_PATTERN = re.compile(
    r"^[ \t]*(?:async\s+)?(?:unsafe\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)", re.M
)
TYPESCRIPT_INTERNAL_FUNCTION_PATTERNS = {
    "typescript": [
        re.compile(r"^[ \t]*(?:async\s+)?function\s+([A-Za-z_$][\w$]*)\s*\(", re.M),
        re.compile(r"^[ \t]*(?:(?:private|protected)\s+)+([A-Za-z_$][\w$]*)\s*\([^;\n]*\)\s*:\s*.*\{\s*$", re.M),
    ],
    "javascript": [
        re.compile(r"^[ \t]*(?:async\s+)?function\s+([A-Za-z_$][\w$]*)\s*\(", re.M),
    ],
}


def _find_matching_brace(source: str, opening: int) -> int | None:
    """Find a matching Rust block brace while skipping comments and strings."""
    depth = 0
    i = opening
    state = "normal"
    block_depth = 0
    raw_end: str | None = None
    while i < len(source):
        char = source[i]
        nxt = source[i + 1] if i + 1 < len(source) else ""
        if state == "line_comment":
            if char == "\n":
                state = "normal"
            i += 1
            continue
        if state == "block_comment":
            if char == "/" and nxt == "*":
                block_depth += 1
                i += 2
            elif char == "*" and nxt == "/":
                block_depth -= 1
                i += 2
                if block_depth == 0:
                    state = "normal"
            else:
                i += 1
            continue
        if state == "string":
            if char == "\\":
                i += 2
            elif char == '"':
                state = "normal"
                i += 1
            else:
                i += 1
            continue
        if state == "raw_string":
            if raw_end and source.startswith(raw_end, i):
                i += len(raw_end)
                state = "normal"
                raw_end = None
            else:
                i += 1
            continue
        if char == "/" and nxt == "/":
            state = "line_comment"
            i += 2
            continue
        if char == "/" and nxt == "*":
            state = "block_comment"
            block_depth = 1
            i += 2
            continue
        if char == '"':
            state = "string"
            i += 1
            continue
        # Raw Rust strings: r"...", r#"..."#, and r###"..."###.
        if char == "r":
            raw = re.match(r'r(#+)?\"', source[i:])
            if raw:
                hashes = raw.group(1) or ""
                raw_end = '"' + hashes
                state = "raw_string"
                i += len(raw.group(0))
                continue
        # Character literals are short; do not treat Rust lifetimes as chars.
        if char == "'" and i + 2 < len(source):
            if source[i + 1] == "\\":
                close = source.find("'", i + 2, min(len(source), i + 6))
                if close != -1:
                    i = close + 1
                    continue
            elif source[i + 2] == "'":
                i += 3
                continue
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return i
        i += 1
    return None


def _mask_rust_test_only_source(source: str) -> tuple[str, list[str]]:
    """Remove inline cfg(test) modules and test functions from runtime scan.

    Test-only function names are returned separately for traceability. This is
    still lexical discovery, not a Rust AST or proof of complete visibility.
    """
    ranges: list[tuple[int, int]] = []
    test_names: set[str] = set()
    module_pattern = re.compile(
        r"#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+[A-Za-z_][A-Za-z0-9_]*\s*\{",
        re.M,
    )
    for match in module_pattern.finditer(source):
        opening = match.end() - 1
        closing = _find_matching_brace(source, opening)
        if closing is None:
            # Malformed/unbalanced source stays visible and is caught as residual.
            continue
        ranges.append((match.start(), closing + 1))
        region = source[opening:closing + 1]
        for pattern in (PATTERNS["rust"][0][1], RUST_INTERNAL_FUNCTION_PATTERN):
            for symbol_match in pattern.finditer(region):
                test_names.add(symbol_match.group(1))
    # Also identify direct #[test], #[tokio::test], and cfg(test) functions
    # that are not enclosed in a cfg(test) module.
    attr_pattern = re.compile(r"#\s*\[\s*(?:tokio::\s*test|test|cfg\s*\(\s*test\s*\))\s*\]")
    for attr in attr_pattern.finditer(source):
        tail = source[attr.end():]
        fn_match = re.search(r"\b(?:async\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)", tail)
        if not fn_match:
            continue
        name = fn_match.group(1)
        test_names.add(name)
        fn_start = attr.start()
        body_open = source.find("{", attr.end() + fn_match.end())
        if body_open < 0:
            continue
        body_close = _find_matching_brace(source, body_open)
        if body_close is not None:
            ranges.append((fn_start, body_close + 1))
    # Merge nested/overlapping ranges before masking.
    merged: list[list[int]] = []
    for start, end in sorted(ranges):
        if merged and start <= merged[-1][1]:
            merged[-1][1] = max(merged[-1][1], end)
        else:
            merged.append([start, end])
    masked = list(source)
    for start, end in merged:
        for pos in range(start, end):
            if masked[pos] != "\\n":
                masked[pos] = " "
    return "".join(masked), sorted(test_names)


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def should_skip(path: Path) -> bool:
    return any(part in SKIP_DIRS for part in path.parts)


def scan(root: Path, repository: str, source_sha: str, scope_exclusions: list[dict] | None = None) -> dict:
    files = []
    symbols_total = 0
    internal_symbols_total = 0
    test_symbols_total = 0
    scope_exclusions = scope_exclusions or []
    exclusion_counts = {str(item.get("path", "")).strip("/"): 0 for item in scope_exclusions}
    language_counts: Counter[str] = Counter()
    unclassified = []
    for path in sorted(root.rglob("*")):
        if not path.is_file():
            continue
        relative = path.relative_to(root).as_posix()
        excluded_prefix = next((prefix for prefix in exclusion_counts if prefix and (relative == prefix or relative.startswith(prefix + "/"))), None)
        if excluded_prefix is not None:
            exclusion_counts[excluded_prefix] += 1
            continue
        if should_skip(path.relative_to(root)):
            continue
        language = EXTENSIONS.get(path.suffix.lower())
        if language is None:
            continue
        language_counts[language] += 1
        try:
            source = path.read_text(encoding="utf-8")
        except (UnicodeDecodeError, OSError) as exc:
            unclassified.append({"path": path.relative_to(root).as_posix(), "reason": type(exc).__name__})
            continue
        found: dict[tuple[str, str, int], str] = {}
        internal_found: dict[tuple[str, str, int], str] = {}
        test_names: list[str] = []
        scan_source = source
        if language == "rust":
            scan_source, test_names = _mask_rust_test_only_source(source)
        source_lines = scan_source.splitlines()
        normalized_parts = [part.lower() for part in path.relative_to(root).as_posix().split("/")]
        filename = normalized_parts[-1] if normalized_parts else ""
        is_test_path = (
            any(part in {"test", "tests", "__tests__"} for part in normalized_parts)
            or filename.startswith("test_")
            or filename.endswith(("_test.rs", "_test.py", ".test.ts", ".test.tsx", ".spec.ts", ".spec.tsx", ".test.js", ".spec.js"))
        )
        for kind, pattern in PATTERNS.get(language, []):
            for match in pattern.finditer(scan_source):
                name = match.group(1)
                # Exclude common test-only/private symbols from the public API candidate set.
                if language == "python" and name.startswith("_"):
                    continue
                line_start = scan_source.count("\n", 0, match.start()) + 1
                signature = source_lines[line_start - 1].strip()[:300] if line_start <= len(source_lines) else name
                found[(kind, name, line_start)] = signature
        if language == "rust":
            for match in RUST_INTERNAL_FUNCTION_PATTERN.finditer(scan_source):
                name = match.group(1)
                line_start = scan_source.count("\n", 0, match.start()) + 1
                signature = source_lines[line_start - 1].strip()[:300] if line_start <= len(source_lines) else name
                key = ("function", name, line_start)
                if key not in found:
                    internal_found[key] = signature
        elif language in TYPESCRIPT_INTERNAL_FUNCTION_PATTERNS:
            for pattern in TYPESCRIPT_INTERNAL_FUNCTION_PATTERNS[language]:
                for match in pattern.finditer(scan_source):
                    name = match.group(1)
                    line_start = scan_source.count("\n", 0, match.start()) + 1
                    signature = source_lines[line_start - 1].strip()[:300] if line_start <= len(source_lines) else name
                    key = ("function", name, line_start)
                    if key not in found:
                        internal_found[key] = signature
        if is_test_path:
            test_names.extend(name for kind, name, _ in found if kind == "function")
            test_names.extend(name for kind, name, _ in internal_found if kind == "function")
            found = {key: value for key, value in found.items() if key[0] != "function"}
            internal_found = {key: value for key, value in internal_found.items() if key[0] != "function"}
            test_names = sorted(set(test_names))
        symbols = [{"kind": kind, "name": name, "line_start": line, "signature": signature} for (kind, name, line), signature in sorted(found.items())]
        internal_symbols = [{"kind": kind, "name": name, "line_start": line, "signature": signature} for (kind, name, line), signature in sorted(internal_found.items())]
        test_symbols = [{"kind": "function", "name": name} for name in test_names]
        symbols_total += len(symbols)
        internal_symbols_total += len(internal_symbols)
        test_symbols_total += len(test_symbols)
        files.append({
            "path": path.relative_to(root).as_posix(),
            "language": language,
            "file_sha256": sha256_file(path),
            "candidate_public_symbols": symbols,
            "candidate_internal_symbols": internal_symbols,
            "candidate_test_symbols": test_symbols,
            "candidate_symbol_count": len(symbols),
            "candidate_internal_symbol_count": len(internal_symbols),
            "candidate_test_symbol_count": len(test_symbols),
        })
    return {
        "schema_id": "ATC-SOURCE-SYMBOL-INVENTORY-001",
        "schema_version": "1.0.0",
        "repository": repository,
        "source_sha": source_sha,
        "scanner": "tools/scan_public_symbols.py",
        "scan_mode": "LEXICAL_CANDIDATE_DISCOVERY",
        "completeness_status": "RESIDUAL",
        "functional_verification": "NOT_PERFORMED",
        "limitations": [
            "Regex/lexical discovery is not a compiler AST and can miss macro-generated, dynamically registered, conditional, or indirect entrypoints.",
            "Candidate symbols are not automatically equated to catalog function contracts.",
            "Generated/vendor/build paths are excluded by policy; excluded path policy must be reviewed per repository.",
            "File hashes bind results to content, but source_sha must be supplied from the exact checked-out repository HEAD."
        ],
        "summary": {
            "source_files_scanned": len(files),
            "candidate_public_symbols": symbols_total,
            "candidate_internal_symbols": internal_symbols_total,
            "candidate_test_symbols": test_symbols_total,
            "files_excluded_by_scope": sum(exclusion_counts.values()),
            "files_by_language": dict(sorted(language_counts.items())),
            "files_with_read_errors": len(unclassified),
        },
        "scope_exclusions": [
            {**item, "path": str(item.get("path", "")).strip("/"), "files_excluded": exclusion_counts.get(str(item.get("path", "")).strip("/"), 0)}
            for item in scope_exclusions
        ],
        "files": files,
        "unclassified_files": unclassified,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", required=True, type=Path, help="Root of one checked-out repository")
    parser.add_argument("--repository", required=True, help="Canonical owner/repository name")
    parser.add_argument("--source-sha", required=True, help="Exact git HEAD SHA of the checked-out repository")
    parser.add_argument("--output", required=True, type=Path, help="Output JSON path")
    args = parser.parse_args()
    if not re.fullmatch(r"[0-9a-fA-F]{40}", args.source_sha):
        parser.error("--source-sha must be a full 40-character Git commit SHA")
    root = args.root.resolve()
    if not root.is_dir():
        parser.error("--root must be an existing directory")
    try:
        actual_sha = subprocess.run(
            ["git", "-C", str(root), "rev-parse", "HEAD"],
            check=True, capture_output=True, text=True,
        ).stdout.strip().lower()
    except (OSError, subprocess.CalledProcessError):
        parser.error("--root must be a Git checkout with a readable HEAD")
    if actual_sha != args.source_sha.lower():
        parser.error(
            f"--source-sha mismatch: supplied {args.source_sha.lower()}, "
            f"actual HEAD {actual_sha}"
        )
    result = scan(root, args.repository, actual_sha)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({"repository": args.repository, "source_sha": args.source_sha.lower(), **result["summary"], "completeness_status": "RESIDUAL", "output": str(args.output)}, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
