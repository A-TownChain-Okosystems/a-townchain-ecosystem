import unittest

from tools.reconcile_runtime_registry_candidates import reconcile


def census():
    return {
        "schema_id": "ATC-ORG-SOURCE-CENSUS-001",
        "repositories": [{
            "repository": "example/sample",
            "name": "sample",
            "source_sha": "a" * 40,
            "status": "RESIDUAL",
            "report": {"files": [
                {"path": "src/lib.rs", "file_sha256": "f" * 64,
                 "candidate_public_symbols": [
                     {"kind": "function", "name": "execute"},
                     {"kind": "function", "name": "test_helper"},
                 ]},
                {"path": "tests/integration.rs", "file_sha256": "e" * 64,
                 "candidate_public_symbols": [{"kind": "function", "name": "integration"}]},
            ]},
        }],
    }


def catalog():
    return {
        "schema_id": "ATC-COMPONENT-FUNCTION-CATALOG-001",
        "components": [{
            "id": "COMP-SAMPLE",
            "name": "sample",
            "canonical_owner": "example/sample",
            "source_inventory": {"repository": "example/sample"},
            "functions": [{
                "id": "FN-EXECUTE",
                "source_binding": {"source_path": "src/lib.rs", "symbol_or_entrypoint": "execute"},
            }],
        }],
    }


def registry():
    return {
        "schema_id": "ATC-RUNTIME-SYMBOL-REGISTRY-001",
        "summary": {"per_repository": {"example/sample": {
            "BOUND_TO_CATALOG_FUNCTION": 1,
            "RUNTIME_SYMBOL": 1,
            "EXCLUDED_TEST": 1,
        }}},
        "entries": [{
            "id": "RTS-example0000",
            "repository": "example/sample",
            "source_path": "src/lib.rs",
            "kind": "function",
            "symbol": "test_helper",
            "file_sha256": "f" * 64,
            "census_source_sha": "a" * 40,
            "status": "RUNTIME_FUNCTION_PENDING_FUNCTIONAL_REVIEW",
        }],
    }


class RuntimeRegistryCandidateReconciliationTests(unittest.TestCase):
    def test_exact_runtime_candidate_maps_and_policy_exclusion_is_explicit(self):
        result = reconcile(census(), catalog(), registry())
        by_symbol = {row.get("symbol"): row for row in result["candidates"] if row.get("symbol")}
        self.assertEqual(by_symbol["execute"]["status"], "MAPPED")
        self.assertEqual(by_symbol["test_helper"]["status"], "MAPPED")
        self.assertEqual(by_symbol["integration"]["status"], "EXCLUDED")
        self.assertEqual(result["summary"]["unresolved"], 0)
        self.assertEqual(result["status"], "PASS")

    def test_missing_runtime_entry_is_unresolved(self):
        reg = registry()
        reg["entries"] = []
        result = reconcile(census(), catalog(), reg)
        by_symbol = {row.get("symbol"): row for row in result["candidates"] if row.get("symbol")}
        self.assertEqual(by_symbol["test_helper"]["status"], "UNRESOLVED")
        self.assertEqual(result["status"], "BLOCKED")
        self.assertGreater(result["summary"]["unresolved"], 0)

    def test_source_sha_mismatch_is_unresolved(self):
        reg = registry()
        reg["entries"][0]["census_source_sha"] = "b" * 40
        result = reconcile(census(), catalog(), reg)
        by_symbol = {row.get("symbol"): row for row in result["candidates"] if row.get("symbol")}
        self.assertEqual(by_symbol["test_helper"]["status"], "UNRESOLVED")

    def test_registry_entry_absent_from_census_is_reported(self):
        reg = registry()
        reg["entries"].append({
            "id": "RTS-extra000000",
            "repository": "example/sample",
            "source_path": "src/old.rs",
            "kind": "function",
            "symbol": "removed",
            "file_sha256": "d" * 64,
            "census_source_sha": "a" * 40,
            "status": "RUNTIME_FUNCTION_PENDING_FUNCTIONAL_REVIEW",
        })
        result = reconcile(census(), catalog(), reg)
        self.assertTrue(any(
            row.get("symbol") == "removed" and row["status"] == "UNRESOLVED"
            for row in result["candidates"]
        ))


if __name__ == "__main__":
    unittest.main()
