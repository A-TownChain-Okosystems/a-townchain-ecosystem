#!/usr/bin/env python3
import json, subprocess, sys

registry=json.load(open("evidence/markdown-registry.json",encoding="utf-8"))
tracked=subprocess.check_output(["git","ls-files","*.md"],text=True).splitlines()
registered=[e["path"] for e in registry["entries"]]
if sorted(tracked)!=sorted(registered):
    missing=sorted(set(tracked)-set(registered))
    stale=sorted(set(registered)-set(tracked))
    print(f"MD-001 FAIL: missing={missing[:20]} stale={stale[:20]}")
    sys.exit(1)
if len(registered)!=registry["markdown_count"] or len(set(registered))!=len(registered):
    print("MD-001 FAIL: count/uniqueness mismatch")
    sys.exit(1)
print(f"MD-001 PASS: {len(tracked)} Markdown files represented exactly once")
