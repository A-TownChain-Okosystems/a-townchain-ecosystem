#!/usr/bin/env python3
"""Functional verification pass for the component function catalog.

Upgrade ladder for source_binding.status:

  SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED
    -> FUNCTIONALLY_VERIFIED_TEST_REFERENCE   (symbol referenced by test
       code at the bound SHA AND the component test suite passes at that
       SHA - local run and/or CI evidence)
    -> TEST_SUITE_PASS_ONLY                   (suite passes at the SHA but
       no direct symbol reference found in test code; weaker evidence,
       still recorded with the run evidence)
    -> unchanged                              (no test evidence: residual)

Usage:
  python3 tools/verify_function_bindings.py \
      --catalog docs/architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.json \
      --workspace /tmp/census-ws \
      --component COMP-LANG --repo atclang \
      --report docs/audit/functional-verification/COMP-LANG.json

Method (deterministic, fail-closed):
  1. Verify the repo working copy is exactly at the bound SHA.
  2. Locate test surfaces: Rust = files under any tests/ directory plus
     inline #[cfg(test)] regions; Python = test_*.py / *_test.py files.
  3. A test reference is a whole-word occurrence of the symbol inside a
     test surface (common-word symbols additionally require :: or . prefix
     usage context, e.g. encode/verify/read/main).
  4. --test-command may run the suite in the repo root; exit code 0 with
     a passing summary upgrades bindings to FUNCTIONALLY_VERIFIED...
     No pass evidence => bindings stay SOURCE_MAPPED (honesty rule).

Agent-ID: aurora-base44-superagent-6a1d37209d7f46ac065d39c5
Task-ID: FUNCTIONAL-VERIFICATION-001
"""
from __future__ import annotations
import argparse, json, re, subprocess, sys
from pathlib import Path

COMMON_WORDS = {
    "main", "read", "write", "verify", "encode", "decode", "files", "fail",
    "run", "execute", "check", "load", "init", "new", "count", "metadata",
}
WORD = re.compile(r"[A-Za-z0-9_]+")


def find_test_files(repo: Path, language: str) -> list[Path]:
    out = []
    if language == "rust":
        for p in repo.rglob("*.rs"):
            rel = p.relative_to(repo)
            if any(part == "tests" for part in rel.parts[:-1]):
                out.append(p)
            elif p.read_text(errors="ignore").count("#[cfg(test)]"):
                out.append(p)  # inline test module
    elif language == "python":
        for p in repo.rglob("*.py"):
            if p.name.startswith("test_") or p.name.endswith("_test.py"):
                out.append(p)
    elif language == "ts":
        for p in list(repo.rglob("*.test.ts")) + list(repo.rglob("*.test.tsx")) \
                + list(repo.rglob("*.spec.ts")) + list(repo.rglob("*.spec.tsx")):
            out.append(p)
    return out


def _rust_code_mask(text: str) -> str:
    """Rust-Quelltext ohne Strings/Char-Literale/Kommentare (als Maske gleicher Laenge).

    Braces in String- und Roh-String-Literalen sowie Kommentaren wuerden die
    naive Klammerbilanz der Test-Region-Extraktion derailen (reale Faelle:
    r#""#-Fixtures mit Schema-Brazen, Fehlermeldungen mit '{'/''). Die Maske
    ersetzt Nicht-Code-Zeichen durch Leerzeichen; die Original-Slices bleiben
    unberuehrt.
    """
    out = list(text)
    i, n = 0, len(text)
    while i < n:
        ch = text[i]
        nxt = text[i + 1] if i + 1 < n else ""
        if ch == "/" and nxt == "/":                      # Zeilenkommentar
            while i < n and text[i] != "\n":
                out[i] = " "; i += 1
        elif ch == "/" and nxt == "*":                    # Blockkommentar
            out[i] = out[i + 1] = " "; i += 2
            while i < n:
                if text[i] == "*" and i + 1 < n and text[i + 1] == "/":
                    out[i] = out[i + 1] = " "; i += 2; break
                out[i] = " "; i += 1
        elif ch == '"':                                   # normale / rohe Strings
            j = i
            hashes = 0
            if j > 0 and text[j - 1] == "r":
                k = j + 1
                while k < n and text[k] == "#":
                    hashes += 1; k += 1
                if hashes == 0:                            # r"..." gibt es nicht, aber "r unmittelbar davor
                    pass
            else:
                hashes = 0
            out[i] = " "; i += 1
            if hashes:                                     # r#...#"#### raw string
                closer = '"' + "#" * hashes
                while i < n:
                    if text.startswith(closer, i):
                        for k in range(i, i + len(closer)):
                            out[k] = " "
                        i += len(closer); break
                    out[i] = " "; i += 1
            else:                                          # "..." mit Escapes
                while i < n:
                    if text[i] == "\\" and i + 1 < n:
                        out[i] = out[i + 1] = " "; i += 2; continue
                    if text[i] == '"':
                        out[i] = " "; i += 1; break
                    out[i] = " "; i += 1
        elif ch == "'":                                   # Char-Literal 'x', '\\n' oder Lifetime
            # Lifetime ('a) bleibt unangetastet: kein Brace-Literal moeglich.
            j = i + 1
            if j < n and text[j] != "'":
                if text[j] == "\\" and j + 2 < n and text[j + 2] == "'":
                    for k in range(i, j + 3):
                        out[k] = " "
                    i = j + 3; continue
                if text[j] == "'" and text[j - 1] != "\\":
                    out[i] = out[j] = " "; i = j + 1; continue
        i += 1
    return "".join(out)


def test_regions(text: str, language: str) -> str:
    """Return only the test-relevant text (inline cfg(test) regions + tests/ files are whole)."""
    if language != "rust" or "#[cfg(test)]" not in text:
        return text
    mask = _rust_code_mask(text)
    regions, idx = [], 0
    while True:
        m = mask.find("#[cfg(test)]", idx)
        if m < 0:
            break
        brace = mask.find("{", m)
        depth, i = 0, brace
        while i < len(mask):
            if mask[i] == "{":
                depth += 1
            elif mask[i] == "}":
                depth -= 1
                if depth == 0:
                    regions.append(text[brace:i + 1]); break
            i += 1
        idx = m + 1
    return "\n".join(regions)


def symbol_referenced(symbol: str, region_text: str) -> bool:
    hits = 0
    for m in WORD.finditer(region_text):
        if m.group(0) == symbol:
            s, e = m.span()
            prefix = region_text[max(0, s - 2):s]
            if symbol in COMMON_WORDS and prefix not in ("::", ". "):
                if prefix != "." and prefix != "::":
                    continue
            hits += 1
    return hits > 0


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--catalog", required=True)
    ap.add_argument("--workspace", required=True)
    ap.add_argument("--component", required=True)
    ap.add_argument("--repo", required=True)
    ap.add_argument("--report", required=True)
    ap.add_argument("--test-command", default=None)
    ap.add_argument("--ci-evidence", default=None, help="URL of a green CI run at the SHA")
    ap.add_argument("--apply", action="store_true", help="write verification results into the catalog bindings")
    args = ap.parse_args()

    catalog = json.load(open(args.catalog))
    comp = next((c for c in catalog["components"] if c.get("id") == args.component), None)
    if not comp:
        print(f"FAIL: component {args.component} not found"); return 2
    repo = Path(args.workspace) / args.repo
    head = subprocess.run(["git", "rev-parse", "HEAD"], cwd=repo, capture_output=True, text=True).stdout.strip()

    langs = {}
    fn_lang = lambda p: "rust" if (p or "").endswith(".rs") else "python" if (p or "").endswith(".py") else "ts"
    bindings = [(f["id"], f["source_binding"]) for f in comp["functions"] if f.get("source_binding")]

    # 1. SHA guard: every binding must reference the checked-out HEAD
    sha_mismatch = [fid for fid, b in bindings if b.get("exact_source_sha") and b["exact_source_sha"] != head]
    if sha_mismatch:
        print(f"FAIL: repo HEAD {head[:8]} does not match binding SHAs for {len(sha_mismatch)} functions "
              f"(e.g. {sha_mismatch[0]}); checkout the bound SHA first"); return 2

    # 2. Test surfaces
    test_files = {lang: find_test_files(repo, lang) for lang in ("rust", "python", "ts")}
    regions = {}
    for lang, files in test_files.items():
        blob = "\n".join(test_regions(p.read_text(errors="ignore"), lang) for p in files)
        regions[lang] = blob
    print(f"test surfaces: {len(test_files['rust'])} rust files, {len(test_files['python'])} python files")

    # 3. Symbol references
    results = []
    for fid, b in bindings:
        sp = b.get("source_path")
        if not sp:
            results.append({"function_id": fid, "symbol": b["symbol_or_entrypoint"],
                            "source_path": None, "language": None,
                            "test_reference": False})
            continue
        lang = fn_lang(sp)
        region = regions.get(lang) or ""
        referenced = symbol_referenced(b["symbol_or_entrypoint"], region) if region else False
        results.append({"function_id": fid, "symbol": b["symbol_or_entrypoint"],
                        "source_path": b["source_path"], "language": lang,
                        "test_reference": referenced})

    # 4. Test run evidence
    run = None
    if args.test_command:
        proc = subprocess.run(args.test_command, shell=True, cwd=repo, capture_output=True, text=True)
        tail = (proc.stdout + proc.stderr).strip().splitlines()[-30:]
        m = re.findall(r"(\d+) passed", proc.stdout)
        run = {"command": args.test_command, "exit_code": proc.returncode,
               "passed": sum(int(x) for x in m) or None, "log_tail": tail}
        print(f"test run: exit={proc.returncode} passed={run['passed']}")

    suite_pass = bool(run and run["exit_code"] == 0)
    verified = suite_pass and all(r["test_reference"] for r in results)

    report = {"component": args.component, "repo": args.repo, "head_sha": head,
              "ci_evidence": args.ci_evidence, "test_run": run,
              "bindings": results,
              "summary": {
                  "total": len(results),
                  "test_referenced": sum(r["test_reference"] for r in results),
                  "suite_pass": suite_pass,
                  "eligible_functionally_verified": suite_pass and sum(r["test_reference"] for r in results),
              }}
    Path(args.report).parent.mkdir(parents=True, exist_ok=True)
    json.dump(report, open(args.report, "w"), indent=1)
    s = report["summary"]
    print(f"summary: {s['total']} bindings, {s['test_referenced']} test-referenced, suite_pass={s['suite_pass']}")

    if args.apply:
        if not suite_pass:
            print("FAIL: --apply requires a passing suite (fail-closed)"); return 2
        by_id = {r["function_id"]: r for r in results}
        changed = 0
        for f in comp["functions"]:
            b = f.get("source_binding")
            r = by_id.get(f["id"])
            if not b or not r:
                continue
            if r["test_reference"]:
                b["status"] = "FUNCTIONALLY_VERIFIED_TEST_REFERENCE"
                b["functional_verification"] = {
                    "method": "test_reference_at_bound_sha",
                    "test_reference": True,
                    "suite_command": run["command"],
                    "suite_exit_code": run["exit_code"],
                    "suite_passed": run["passed"],
                    "ci_evidence": args.ci_evidence,
                    "verified_against_sha": head,
                }
                changed += 1
            else:
                b.setdefault("note", "")
                if "no direct test reference" not in b["note"]:
                    b["note"] = (b["note"] + " " if b["note"] else "") + \
                        f"Suite passes at the bound SHA but no direct test reference for this symbol (verified {head[:8]})."
        catalog_path = Path(args.catalog)
        json.dump(catalog, open(catalog_path, "w"), indent=1, ensure_ascii=False)
        print(f"applied: {changed} bindings upgraded to FUNCTIONALLY_VERIFIED_TEST_REFERENCE")
    return 0


if __name__ == "__main__":
    sys.exit(main())
