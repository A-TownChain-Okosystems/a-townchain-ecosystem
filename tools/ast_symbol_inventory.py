#!/usr/bin/env python3
"""Repository-wide AST symbol inventory."""
from __future__ import annotations
import json, sys
from pathlib import Path
from tree_sitter import Language, Parser
import tree_sitter_rust as ts_rust
import tree_sitter_python as ts_python
import tree_sitter_javascript as ts_js
import tree_sitter_typescript as ts_ts
import tree_sitter_go as ts_go
GRAMMARS={".rs":Language(ts_rust.language()),".py":Language(ts_python.language()),".js":Language(ts_js.language()),".jsx":Language(ts_js.language()),".ts":Language(ts_ts.language_typescript()),".tsx":Language(ts_ts.language_tsx()),".go":Language(ts_go.language())}
FUNCTION_NODES={"function_item","function_definition","method_definition","function_declaration","method_declaration"}
TYPE_NODES={"struct_item","enum_item","trait_item","type_item","class_definition","class_declaration","interface_declaration","type_alias_declaration","type_declaration"}
EXCLUDED_DIRS={".git",".github","target","node_modules","dist","build",".venv","venv"}
def node_name(node,source):
    for child in node.children:
        if child.type in {"identifier","type_identifier","field_identifier","name"}:
            return source[child.start_byte:child.end_byte].decode("utf-8","replace")
    return None
def walk(node,source,path,out):
    if node.type in FUNCTION_NODES|TYPE_NODES:
        name=node_name(node,source)
        if name:
            out.append({"path":str(path),"line":node.start_point[0]+1,"kind":"function" if node.type in FUNCTION_NODES else "type","symbol":name,"ast_node":node.type})
    for child in node.children: walk(child,source,path,out)
def main(root):
    root=Path(root); rows=[]
    for path in sorted(root.rglob("*")):
        if not path.is_file() or any(p in EXCLUDED_DIRS for p in path.parts): continue
        lang=GRAMMARS.get(path.suffix.lower())
        if not lang: continue
        try:
            source=path.read_bytes(); tree=Parser(lang).parse(source); walk(tree.root_node,source,path.relative_to(root),rows)
        except Exception as exc:
            rows.append({"path":str(path.relative_to(root)),"kind":"parse_error","error":str(exc)})
    rows.sort(key=lambda r:(r["path"],r.get("line",0),r["kind"],r.get("symbol","")))
    for row in rows: print(json.dumps(row,ensure_ascii=False,sort_keys=True))
if __name__=="__main__":
    if len(sys.argv)!=2: raise SystemExit("usage: ast_symbol_inventory.py <repo-root>")
    main(sys.argv[1])
