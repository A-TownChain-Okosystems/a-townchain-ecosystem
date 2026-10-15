import copy
import unittest

from tools.validate_component_function_catalog import validate


class ContractAnalysisValidationTests(unittest.TestCase):
    def setUp(self):
        self.doc = {
            "schema_id": "ATC-COMPONENT-FUNCTION-CATALOG-001",
            "schema_version": "1.1.0",
            "scope": {
                "organization_repository_count": 1,
                "active_repository_count": 1,
                "archived_repository_count": 0,
            },
            "repository_coverage": [{
                "repository": "example/repo",
                "role": "implementation",
                "canonicality": "CANONICAL",
                "observed_archive_state": "ACTIVE",
                "function_inventory_status": "RESIDUAL",
                "production_readiness": "RESIDUAL",
            }],
            "production_gates": [{
                "id": "PG-001", "status": "RESIDUAL", "acceptance": ["evidence required"],
            }],
            "cross_component_interfaces": [],
            "components": [{
                "id": "COMP-SAMPLE", "name": "sample", "layer": "L1",
                "canonical_owner": "example/repo", "role": "sample",
                "criticality": "P1", "depends_on": [], "readiness": "RESIDUAL",
                "functions": [{
                    "id": "FN-SAMPLE-001", "name": "run", "purpose": "Run the sample operation.",
                    "inputs": ["request"], "outputs": ["result"],
                    "failure_modes": ["invalid request"], "acceptance": ["fail closed"],
                    "test_classes": ["unit"], "readiness": "RESIDUAL",
                    "contract_analysis": {
                        "schema_version": "1.0.0",
                        "purpose": {"status": "DOCUMENTED_RESIDUAL", "summary": "Run the sample operation."},
                        "inputs": {"status": "DOCUMENTED_RESIDUAL", "items": ["request"]},
                        "outputs": {"status": "DOCUMENTED_RESIDUAL", "items": ["result"]},
                        "preconditions": {"status": "UNRESOLVED", "reason": "source review pending"},
                        "behavior": {"status": "UNRESOLVED", "summary": None, "invariants": [],
                                     "side_effects_status": "UNRESOLVED", "trust_boundary_status": "UNRESOLVED"},
                        "architecture_placement": {"status": "DOCUMENTED_RESIDUAL", "component_id": "COMP-SAMPLE",
                                                   "layer": "L1", "canonical_owner": "example/repo",
                                                   "source_path": None, "symbol": None},
                        "consumers": {"status": "UNRESOLVED", "entries": [], "rationale": "call graph pending"},
                        "reuse": {"status": "UNRESOLVED", "entries": [], "rationale": "reuse audit pending"},
                        "failure_semantics": {"status": "DOCUMENTED_RESIDUAL", "failure_modes": ["invalid request"],
                                              "error_contract_status": "UNRESOLVED", "recovery_status": "UNRESOLVED"},
                        "source_evidence": {"status": "UNRESOLVED", "repository": "example/repo",
                                            "commit_sha": None, "path": None, "symbol": None, "blob_sha": None},
                    },
                }],
            }],
            "release_decision": {"status": "BLOCKED"},
        }

    def test_unresolved_consumers_and_reuse_are_not_misrepresented_as_verified(self):
        errors, _ = validate(self.doc)
        self.assertEqual(errors, [])

    def test_preconditions_list_is_accepted_as_explicit_residual_contract_data(self):
        self.doc["components"][0]["functions"][0]["contract_analysis"]["preconditions"] = [
            "source-level precondition review is pending"
        ]
        errors, _ = validate(self.doc)
        self.assertEqual(errors, [])

    def test_missing_structured_contract_is_rejected(self):
        del self.doc["components"][0]["functions"][0]["contract_analysis"]
        errors, _ = validate(self.doc)
        self.assertTrue(any("contract_analysis must be an object" in error for error in errors))

    def test_wrong_component_placement_is_rejected(self):
        self.doc["components"][0]["functions"][0]["contract_analysis"]["architecture_placement"]["component_id"] = "COMP-OTHER"
        errors, _ = validate(self.doc)
        self.assertTrue(any("architecture_placement.component_id" in error for error in errors))

    def test_source_mapped_evidence_requires_full_shas(self):
        evidence = self.doc["components"][0]["functions"][0]["contract_analysis"]["source_evidence"]
        evidence.update({"status": "SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED", "repository": "example/repo",
                         "commit_sha": "short", "path": "src/lib.rs", "symbol": "run", "blob_sha": "short"})
        errors, _ = validate(self.doc)
        self.assertTrue(any("source_evidence.commit_sha must be a full 40-character SHA" in error for error in errors))
        self.assertTrue(any("source_evidence.blob_sha must be a full 40-character SHA" in error for error in errors))


if __name__ == "__main__":
    unittest.main()
