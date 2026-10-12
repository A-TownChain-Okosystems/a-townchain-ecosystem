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


    def test_type_with_same_name_does_not_satisfy_function_binding(self):
        self.census["repositories"][0]["report"]["files"][0]["candidate_public_symbols"] = [
            {"kind": "type", "name": "execute"}
        ]
        report = reconcile(self.catalog, self.census)
        self.assertEqual(
            report["function_findings"][0]["finding"],
            "BOUND_SYMBOL_NOT_FOUND_IN_LEXICAL_SCAN",
        )
        self.assertEqual(report["summary"]["candidate_symbols_unbound"], 1)
        candidate = report["unmatched_candidates"][0]
        self.assertEqual(candidate["symbol_kind"], "type")
        self.assertEqual(candidate["source_sha"], "a" * 40)

    def test_explicit_type_binding_requires_matching_symbol_kind(self):
        fn = self.catalog["components"][0]["functions"][0]
        fn["source_binding"]["symbol_kind"] = "type"
        self.census["repositories"][0]["report"]["files"][0]["candidate_public_symbols"] = [
            {"kind": "type", "name": "execute"}
        ]
        report = reconcile(self.catalog, self.census)
        self.assertEqual(
            report["function_findings"][0]["finding"],
            "LEXICAL_MATCH_ONLY_FUNCTIONAL_VERIFICATION_PENDING",
        )


    def test_completion_gate_blocks_unbound_source_candidates(self):
        report = reconcile(self.catalog, self.census)
        report["unmatched_candidates"] = [{
            "repository": "example/sample",
            "source_path": "src/lib.rs",
            "symbol_kind": "function",
            "symbol_or_entrypoint": "unmapped",
            "finding": "CANDIDATE_NOT_BOUND_TO_CATALOG_FUNCTION",
        }]
        report["summary"]["candidate_symbols_unbound"] = 1
        from tools.reconcile_component_function_catalog import completion_gaps
        self.assertTrue(any("remain unbound" in gap for gap in completion_gaps(report)))


    def test_completion_gate_allows_explicit_not_yet_implemented_contract(self):
        report = reconcile(self.catalog, self.census)
        report["function_findings"][0]["finding"] = "SOURCE_SURFACE_NOT_IDENTIFIED"
        report["function_findings"][0]["binding_status"] = "SOURCE_SURFACE_NOT_IDENTIFIED"
        report["function_findings"][0]["binding_note"] = "No existing source function satisfies this contract; implementation is a later phase."
        from tools.reconcile_component_function_catalog import completion_gaps
        self.assertEqual(completion_gaps(report), [])

    def test_completion_gate_allows_explicit_normative_blocker_without_calling_it_verified(self):
        report = reconcile(self.catalog, self.census)
        report["function_findings"][0]["finding"] = "NORMATIVE_DECISION_BLOCKED"
        report["function_findings"][0]["binding_note"] = "Consensus selection is not normatively finalized."
        from tools.reconcile_component_function_catalog import completion_gaps
        self.assertEqual(completion_gaps(report), [])
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

    def test_test_and_example_symbols_are_reasoned_exclusions(self):
        report_file = self.census["repositories"][0]["report"]["files"][0]
        report_file["candidate_public_symbols"] = [{"kind": "function", "name": "execute"}]
        self.census["repositories"][0]["report"]["files"].append({
            "path": "tests/test_vm.py",
            "candidate_public_symbols": [{"kind": "function", "name": "test_vm"}],
            "candidate_internal_symbols": [],
        })
        self.census["repositories"][0]["report"]["files"].append({
            "path": "examples/demo.rs",
            "candidate_public_symbols": [{"kind": "function", "name": "demo"}],
            "candidate_internal_symbols": [],
        })
        report = reconcile(self.catalog, self.census)
        self.assertEqual(report["summary"]["candidate_symbols_unbound"], 0)
        self.assertEqual(report["summary"]["candidate_symbols_excluded_by_policy"], 2)
        findings = {item["finding"] for item in report["excluded_candidates"]}
        self.assertIn("TEST_OR_FIXTURE_FUNCTION_EXCLUDED_FROM_RUNTIME_CATALOG", findings)
        self.assertIn("EXAMPLE_OR_SAMPLE_FUNCTION_EXCLUDED_FROM_RUNTIME_CATALOG", findings)
        self.assertTrue(all(item["reason"] for item in report["excluded_candidates"]))

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
