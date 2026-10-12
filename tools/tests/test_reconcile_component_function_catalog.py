import unittest

from tools.reconcile_component_function_catalog import reconcile


class CatalogReconciliationTests(unittest.TestCase):
    def setUp(self):
        self.catalog = {
            "schema_id": "ATC-COMPONENT-FUNCTION-CATALOG-001",
            "components": [{
                "id": "COMP-SAMPLE",
                "name": "sample",
                "canonical_owner": "example/sample",
                "source_inventory": {
                    "repository": "example/sample",
                    "observed_head_sha": "a" * 40,
                },
                "functions": [{
                    "id": "FN-SAMPLE-001",
                    "name": "execute",
                    "source_binding": {
                        "source_path": "src/lib.rs",
                        "symbol_or_entrypoint": "execute",
                        "exact_source_sha": "a" * 40,
                    },
                }],
            }],
        }
        self.census = {
            "schema_id": "ATC-ORG-SOURCE-CENSUS-001",
            "status": "RESIDUAL",
            "repositories": [{
                "repository": "example/sample",
                "status": "RESIDUAL",
                "source_sha": "a" * 40,
                "report": {
                    "files": [{
                        "path": "src/lib.rs",
                        "candidate_public_symbols": [{"kind": "function", "name": "execute"}],
                    }],
                },
            }],
        }

    def test_exact_sha_symbol_match_remains_functionally_residual(self):
        report = reconcile(self.catalog, self.census)
        self.assertEqual(report["status"], "RESIDUAL")
        self.assertEqual(report["summary"]["candidate_symbols_unbound"], 0)
        finding = report["function_findings"][0]
        self.assertEqual(finding["finding"], "LEXICAL_MATCH_ONLY_FUNCTIONAL_VERIFICATION_PENDING")
        self.assertEqual(report["summary"]["functional_verification"], "NOT_PERFORMED")

    def test_catalog_self_reference_pr_head_is_not_misreported_as_source_drift(self):
        self.catalog["authority"] = {"repository": "example/sample"}
        self.catalog["components"][0]["source_inventory"]["observed_head_sha"] = "b" * 40
        report = reconcile(self.catalog, self.census)
        self.assertEqual(report["component_findings"], [])
        self.assertEqual(len(report["self_reference_observations"]), 1)
        self.assertEqual(
            report["self_reference_observations"][0]["finding"],
            "SELF_REFERENCE_PR_HEAD_DIFFERS_FROM_DEFAULT_BRANCH",
        )

    def test_internal_symbol_can_be_reconciled_without_becoming_public_api(self):
        self.census["repositories"][0]["report"]["files"][0]["candidate_public_symbols"] = []
        self.census["repositories"][0]["report"]["files"][0]["candidate_internal_symbols"] = [
            {"kind": "function", "name": "execute"}
        ]
        report = reconcile(self.catalog, self.census)
        self.assertEqual(
            report["function_findings"][0]["finding"],
            "LEXICAL_MATCH_ONLY_FUNCTIONAL_VERIFICATION_PENDING",
        )
        self.assertEqual(report["summary"]["functional_verification"], "NOT_PERFORMED")

    def test_reasoned_source_gap_is_not_reported_as_missing_binding(self):
        fn = self.catalog["components"][0]["functions"][0]
        fn["source_binding"] = {"status": "SOURCE_SURFACE_NOT_IDENTIFIED", "source_path": None, "symbol_or_entrypoint": None}
        report = reconcile(self.catalog, self.census)
        self.assertEqual(report["function_findings"][0]["finding"], "SOURCE_SURFACE_NOT_IDENTIFIED")
        self.assertEqual(report["function_findings"][0]["status"], "RESIDUAL")

    def test_non_function_artifact_is_excluded_from_symbol_matching(self):
        fn = self.catalog["components"][0]["functions"][0]
        fn["source_binding"] = {
            "status": "NON_FUNCTION_SOURCE_ARTIFACT",
            "source_path": "specs/contract.yaml",
            "symbol_or_entrypoint": None,
            "exact_source_sha": "a" * 40,
        }
        report = reconcile(self.catalog, self.census)
        self.assertEqual(
            report["function_findings"][0]["finding"],
            "NON_FUNCTION_SOURCE_ARTIFACT_EXCLUDED_FROM_SYMBOL_SCAN",
        )

    def test_related_source_surface_remains_residual_not_verified(self):
        fn = self.catalog["components"][0]["functions"][0]
        fn["source_binding"]["status"] = "RELATED_SOURCE_SURFACE_ONLY"
        report = reconcile(self.catalog, self.census)
        self.assertEqual(
            report["function_findings"][0]["finding"],
            "RELATED_SOURCE_SURFACE_ONLY_CONTRACT_RESIDUAL",
        )
        self.assertEqual(report["function_findings"][0]["status"], "RESIDUAL")

    def test_sha_mismatch_is_not_accepted_as_a_binding(self):
        self.catalog["components"][0]["functions"][0]["source_binding"]["exact_source_sha"] = "b" * 40
        report = reconcile(self.catalog, self.census)
        self.assertEqual(report["function_findings"][0]["finding"], "FUNCTION_BINDING_SHA_MISMATCH")
        self.assertEqual(report["summary"]["candidate_symbols_unbound"], 1)

    def test_missing_source_checkout_blocks_reconciliation(self):
        self.census["repositories"] = []
        report = reconcile(self.catalog, self.census)
        self.assertEqual(report["status"], "BLOCKED")
        self.assertEqual(report["function_findings"][0]["finding"], "SOURCE_CENSUS_UNAVAILABLE")


if __name__ == "__main__":
    unittest.main()
