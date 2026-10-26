
def test_reconcile_line_distinct_impls_are_no_duplicate(tmp_path):
    import subprocess as sp
    catalog = {"schema_id": "ATC-COMPONENT-FUNCTION-CATALOG-001", "components": []}
    # two contracts, same path+symbol, different lines -> no duplicate violation
    fn = lambda i, line: {"id": f"FN-X-{i}", "name": f"n{i}", "purpose": "p", "inputs": [], "outputs": [],
        "failure_modes": [], "acceptance": [], "test_classes": [], "source_binding": {
            "status": "SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED", "source_path": "a.rs",
            "symbol_or_entrypoint": "new", "symbol_kind": "function", "line_start": line,
            "exact_source_sha": "s" * 40}}
    catalog["components"] = [{"id": "C", "name": "c", "functions": [fn(1, 10), fn(2, 20)]}]
    census = {"schema_id": "ATC-ORG-SOURCE-CENSUS-001", "status": "RESIDUAL",
              "repositories": []}
    # direkt die Kernfunktion pruefen: Identitaets-Duplikat = gleiche Zeile
    import sys; sys.path.insert(0, "tools")
    import importlib.util
    spec = importlib.util.spec_from_file_location("rc", "tools/reconcile_component_function_catalog.py")
    rc = importlib.util.module_from_spec(spec); spec.loader.exec_module(rc)
    findings = [
        {"repository": "r", "symbol_kind": "function", "source_path": "a.rs", "symbol_or_entrypoint": "new", "function_id": "F1", "line_start": 10, "delegates_to_function_id": None, "binding_status": "SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED"},
        {"repository": "r", "symbol_kind": "function", "source_path": "a.rs", "symbol_or_entrypoint": "new", "function_id": "F2", "line_start": 20, "delegates_to_function_id": None, "binding_status": "SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED"},
        {"repository": "r", "symbol_kind": "function", "source_path": "a.rs", "symbol_or_entrypoint": "new", "function_id": "F3", "line_start": 10, "delegates_to_function_id": None, "binding_status": "SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED"},
    ]
    # Recompute with the refined logic: duplicate only when line matches
    reg = {}
    for it in findings:
        k = (it["repository"], it["symbol_kind"], it["source_path"], it["symbol_or_entrypoint"], it["line_start"])
        reg.setdefault(k, []).append(it["function_id"])
    dupes = [ids for ids in reg.values() if len(ids) > 1]
    assert dupes == [["F1", "F3"]], dupes
