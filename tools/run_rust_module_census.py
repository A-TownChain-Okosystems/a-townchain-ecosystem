#!/usr/bin/env python3
"""Exact-HEAD Rust module/package/dependency census; lexical discovery only."""
from __future__ import annotations
import argparse, hashlib, json, re, subprocess, tomllib
from pathlib import Path

SKIP = {".git", "target", "vendor", "node_modules", "dist", "build"}
MOD_RE = re.compile(r"(?m)^\s*(?:(?:pub(?:\([^)]*\))?)\s+)?mod\s+([A-Za-z_]\w*)\s*(\{|;)")
PUB_RE = re.compile(r"(?m)^\s*pub(?:\([^)]*\))?\s+(?:(?:async|unsafe|const|extern\s+\"[^\"]+\"|default)\s+)*(fn|struct|enum|trait|type|union|static|const|mod)\s+([A-Za-z_]\w*)")
CFG_RE = re.compile(r"#\s*\[\s*cfg\s*\((.*?)\)\s*\]", re.S)
USE_RE = re.compile(r"(?m)^\s*(?:pub\s+)?use\s+([^;]+);")
DEP_SECTIONS = {"dependencies", "dev-dependencies", "build-dependencies"}

def git(root: Path, *args: str) -> str:
    return subprocess.run(["git","-C",str(root),*args],check=True,capture_output=True,text=True).stdout.strip()

def file_sha(path: Path) -> str:
    h=hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda:f.read(1024*1024),b""): h.update(chunk)
    return h.hexdigest()

def owner(repo: str, rel: str) -> tuple[str,str]:
    # Canonical ownership is explicit; all other paths remain unresolved.
    p=rel.replace("\\","/")
    if repo.endswith("/atclang") or "/atclang/" in repo:
        return ("atclang","SSOT") if p.startswith("crates/") else ("UNRESOLVED","RESIDUAL")
    if repo.endswith("/a-townchain") or "/a-townchain/" in repo:
        if p.startswith("components/vm/"): return ("components/vm","SSOT")
        if p.startswith("components/algorithm/"): return ("components/algorithm","SSOT")
        if p.startswith("components/wallet/"): return ("components/wallet","SSOT")
        if p.startswith("modules/atc-blockchain/"): return ("a-townchain L1","CANDIDATE_REQUIRES_RECONCILIATION")
    if repo.endswith("/globus-os") or "/globus-os/" in repo:
        if p.startswith("modules/atc-shivacore/kernel/"): return ("ShivaCore Kernel","SSOT")
        if p.startswith("modules/atc-shivacore/"): return ("ShivaCore specs/boot/service-space","CANDIDATE_REQUIRES_RECONCILIATION")
    if repo.endswith("/aurora-ai") or "/aurora-ai/" in repo:
        if p.startswith("modules/atc-aurora-"): return ("Aurora AI","SSOT_CANDIDATE")
    return ("UNRESOLVED","RESIDUAL")

def cfg_for(source: str, pos: int) -> list[str]:
    prefix=source[max(0,pos-500):pos]
    # Only contiguous attribute/declaration prefix is considered; stop at blank line or code.
    lines=prefix.splitlines()
    tail=[]
    for line in reversed(lines):
        s=line.strip()
        if not s:
            if tail: break
            continue
        if s.startswith("#[") or s.startswith("///") or s.startswith("//!"):
            tail.append(s); continue
        break
    return [m.group(1).strip() for line in reversed(tail) for m in CFG_RE.finditer(line)]

def cargo_metadata(path: Path, repo: str, head: str) -> list[dict]:
    try: data=tomllib.loads(path.read_text(encoding="utf-8"))
    except Exception as e:
        return [{"kind":"cargo_manifest","repository":repo,"head_sha":head,"path":path.as_posix(),"blob_sha":git(path.parents[0],"hash-object",str(path)) if False else None,"status":"BLOCKED","reason":type(e).__name__}]
    rel=path.as_posix()
    package=data.get("package",{})
    records=[]
    if package:
        records.append({"kind":"cargo_package","repository":repo,"head_sha":head,"path":rel,"package_name":package.get("name"),"package_version":package.get("version"),"edition":package.get("edition"),"status":"RESIDUAL","blob_sha":None})
    ws=data.get("workspace",{})
    for member in ws.get("members",[]) if isinstance(ws,dict) else []:
        records.append({"kind":"cargo_workspace_member","repository":repo,"head_sha":head,"path":rel,"member":member,"status":"RESIDUAL"})
    for section, entries in data.items():
        if section in DEP_SECTIONS or section.startswith("target."):
            if not isinstance(entries,dict): continue
            for sub, deps in entries.items():
                if sub not in DEP_SECTIONS or not isinstance(deps,dict): continue
                for name, spec in deps.items():
                    records.append({"kind":"cargo_dependency","repository":repo,"head_sha":head,"path":rel,"section":section+"."+sub,"dependency":name,"spec":spec if isinstance(spec,(str,int,float,bool)) else {k:v for k,v in spec.items() if k in {"version","path","git","package","optional","default-features","features"} },"status":"RESIDUAL"})
    return records

def scan_repo(root: Path, repo: str) -> dict:
    head=git(root,"rev-parse","HEAD")
    tree=git(root,"rev-parse","HEAD^{tree}")
    modules=[]; symbols=[]; packages=[]; directories=[]; dependencies=[]; errors=[]
    rust_files=sorted(p for p in root.rglob("*.rs") if not any(x in SKIP for x in p.parts))
    manifests=sorted(p for p in root.rglob("Cargo.toml") if not any(x in SKIP for x in p.parts))
    dirs=sorted(p for p in root.rglob("*") if p.is_dir() and not any(x in SKIP for x in p.parts))
    for d in dirs:
        if any((d/n).exists() for n in ("Cargo.toml","src","package.json")):
            rel=d.relative_to(root).as_posix() or "."
            own,status=owner(repo,rel)
            directories.append({"kind":"directory_candidate","repository":repo,"head_sha":head,"path":rel,"owner_ssot":own,"status":status})
    for path in manifests:
        rel=path.relative_to(root).as_posix()
        blob=git(root,"rev-parse",f"HEAD:{rel}")
        try: data=tomllib.loads(path.read_text(encoding="utf-8"))
        except Exception as e:
            errors.append({"path":rel,"reason":"CARGO_TOML_PARSE_ERROR","detail":type(e).__name__}); continue
        pkg=data.get("package",{})
        if pkg:
            packages.append({"kind":"cargo_package","repository":repo,"head_sha":head,"path":rel,"blob_sha":blob,"package_name":pkg.get("name"),"version":pkg.get("version"),"edition":pkg.get("edition"),"status":"RESIDUAL"})
        for section, entries in data.items():
            candidates=[]
            if section in DEP_SECTIONS and isinstance(entries,dict): candidates.append((section,entries))
            if section.startswith("target.") and isinstance(entries,dict):
                candidates.extend((section+"."+s,entries[s]) for s in DEP_SECTIONS if isinstance(entries.get(s),dict))
            for sec, deps in candidates:
                for name,spec in deps.items():
                    dependencies.append({"kind":"cargo_dependency","repository":repo,"head_sha":head,"path":rel,"blob_sha":blob,"section":sec,"dependency":name,"spec":spec if isinstance(spec,str) else {k:v for k,v in spec.items() if k in {"version","path","git","package","optional","default-features","features"}} if isinstance(spec,dict) else str(spec),"status":"RESIDUAL"})
    for path in rust_files:
        rel=path.relative_to(root).as_posix()
        blob=git(root,"rev-parse",f"HEAD:{rel}")
        source=path.read_text(encoding="utf-8",errors="replace")
        own, own_status=owner(repo,rel)
        common={"repository":repo,"head_sha":head,"path":rel,"blob_sha":blob,"owner_ssot":own}
        for match in MOD_RE.finditer(source):
            attrs=cfg_for(source,match.start())
            gate="TEST_ONLY" if any(re.search(r"\btest\b",a) for a in attrs) else "FEATURE_GATED" if any("feature" in a for a in attrs) else "CONDITIONAL" if attrs else "UNCONDITIONAL_OR_UNANNOTATED"
            modules.append({**common,"kind":"rust_module_declaration","name":match.group(1),"declaration":"inline" if match.group(2)=="{" else "external","line":source.count("\n",0,match.start())+1,"cfg_attributes":attrs,"gate_class":gate,"status":"RESIDUAL" if own_status!="SSOT" else "CANDIDATE_REQUIRES_RECONCILIATION"})
        for match in PUB_RE.finditer(source):
            attrs=cfg_for(source,match.start())
            symbols.append({**common,"kind":"rust_public_symbol","symbol_kind":match.group(1),"name":match.group(2),"line":source.count("\n",0,match.start())+1,"cfg_attributes":attrs,"gate_class":"FEATURE_GATED" if any("feature" in a for a in attrs) else "TEST_ONLY" if any(re.search(r"\btest\b",a) for a in attrs) else "CONDITIONAL" if attrs else "UNCONDITIONAL_OR_UNANNOTATED","status":"RESIDUAL"})
    return {"repository":repo,"head_sha":head,"root_tree_sha":tree,"working_tree_clean":not bool(git(root,"status","--porcelain","--untracked-files=all")),"summary":{"rust_files":len(rust_files),"cargo_manifests":len(manifests),"modules":len(modules),"public_symbols":len(symbols),"packages":len(packages),"dependencies":len(dependencies),"directory_candidates":len(directories)},"modules":modules,"public_symbols":symbols,"packages":packages,"dependencies":dependencies,"directory_candidates":directories,"errors":errors,"status":"RESIDUAL","limitations":["Regex-based lexical discovery, not rust-analyzer or compiler AST.","External module resolution and macro-generated declarations require compiler-aware follow-up.","Dependency versions and cfg predicates do not establish runtime reachability.","SSOT labels are candidate ownership rules; unresolved paths remain RESIDUAL.","Functional verification was not performed."]}

def main():
    ap=argparse.ArgumentParser()
    ap.add_argument("--workspace",required=True,type=Path)
    ap.add_argument("--repositories",required=True,help="semicolon-separated repository names as workspace directory names")
    ap.add_argument("--output",required=True,type=Path)
    a=ap.parse_args()
    reports=[]; blocked=[]
    for name in [x for x in a.repositories.split(";") if x]:
        root=a.workspace/name
        if not (root/".git").exists():
            blocked.append({"repository":name,"status":"BLOCKED","reason":"CHECKOUT_MISSING"}); continue
        try: reports.append(scan_repo(root,"A-TownChain-Okosystems/"+name))
        except Exception as e: blocked.append({"repository":name,"status":"BLOCKED","reason":type(e).__name__,"detail":str(e)})
    summary={k:sum(r["summary"][k] for r in reports) for k in ("rust_files","cargo_manifests","modules","public_symbols","packages","dependencies","directory_candidates")}
    out={"schema_id":"ATC-RUST-MODULE-CENSUS-001","schema_version":"1.0.0","status":"BLOCKED" if blocked else "RESIDUAL","functional_verification":"NOT_PERFORMED","summary":summary,"repositories":reports,"blocked_repositories":blocked,"limitations":["Counts are lexical discovery candidates, not verified module/package/function contracts.","Each record binds repository HEAD, source path and Git blob SHA where file-backed.","Unknown ownership, module resolution, and cfg semantics remain RESIDUAL/BLOCKED."]}
    a.output.parent.mkdir(parents=True,exist_ok=True); a.output.write_text(json.dumps(out,indent=2,sort_keys=True)+"\n",encoding="utf-8")
    print(json.dumps({"status":out["status"],**summary,"repositories_scanned":len(reports),"repositories_blocked":len(blocked),"output":str(a.output)},sort_keys=True))
    return 2 if blocked else 0
if __name__=="__main__": raise SystemExit(main())
