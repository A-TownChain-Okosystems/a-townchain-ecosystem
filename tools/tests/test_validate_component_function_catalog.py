import unittest

from tools.validate_component_function_catalog import validate


def valid_interface():
    return {
        "id": "IF-001",
        "producer": "COMP-SAMPLE",
        "consumer": "COMP-SAMPLE",
        "contract": "sample interface",
        "status": "RESIDUAL",
        "status_reason": "Exact source and contract tests are pending.",
        "contract_version": None,
        "version_status": "RESIDUAL",
        "request_schema": {"status": "RESIDUAL", "format": None, "fields": [], "notes": "Schema pending."},
        "response_schema": {"status": "RESIDUAL", "format": None, "fields": [], "notes": "Schema pending."},
        "error_timeout_semantics": {"status": "RESIDUAL", "typed_errors": [], "timeout_behavior": None},
        "authorization_capability": {"status": "RESIDUAL", "required_capabilities": [], "trust_boundary": "RESIDUAL"},
        "compatibility": {"status": "RESIDUAL", "policy": None},
        "contract_tests": {"status": "RESIDUAL", "required_classes": ["schema-conformance"], "exact_source_sha": None},
        "source_evidence": {"status": "RESIDUAL", "producer_source_sha": None, "consumer_source_sha": None},
        "resolution_rule": "Remain residual until exact-SHA source and contract test evidence is available.",
    }


def valid_catalog():
    function = {
        "id": "FN-SAMPLE-001",
        "name": "execute",
        "purpose": "Execute the sample operation under an explicit contract.",
        "canonical_owner": "example/sample",
        "source_binding": {
            "status": "SOURCE_MAPPED_NOT_FUNCTIONALLY_VERIFIED",
            "source_path": "src/lib.rs",
            "symbol_or_entrypoint": "execute",
            "symbol_kind": "function",
            "exact_source_sha": "a" * 40,
        },
        "inputs": ["validated request"],
        "outputs": ["typed result"],
        "preconditions": ["request is validated"],
        "postconditions": ["state is unchanged on failure"],
        "failure_modes": ["invalid input"],
        "resource_limits": {"status": "RESIDUAL", "reason": "Pending review."},
        "security_boundary": {"status": "RESIDUAL", "reason": "Pending review."},
        "versioning_compatibility": {"status": "RESIDUAL", "reason": "Pending review."},
        "dependencies": [],
        "acceptance": ["invalid input is rejected"],
        "test_classes": ["unit", "negative"],
        "verification_evidence": {"status": "RESIDUAL"},
        "readiness": "RESIDUAL",
    }
    return {
        "schema_id": "ATC-COMPONENT-FUNCTION-CATALOG-001",
        "schema_version": "1.0.0",
        "components": [{
            "id": "COMP-SAMPLE",
            "name": "sample",
            "layer": "L0",
            "canonical_owner": "example/sample",
            "role": "sample component",
            "criticality": "low",
            "depends_on": [],
            "functions": [function],
            "readiness": "RESIDUAL",
            "execution_model": "LIBRARY",
            "execution_model_rationale": "Sample library component consumed as a dependency.",
        }],
        "production_gates": [{"id": "GATE-001", "status": "RESIDUAL", "acceptance": ["catalog remains residual until evidence passes"]}],
        "cross_component_interfaces": [valid_interface()],
        "repository_coverage": [{
            "repository": "example/sample",
            "role": "canonical sample",
            "canonicality": "CANONICAL",
            "observed_archive_state": "ACTIVE",
            "function_inventory_status": "RESIDUAL",
            "production_readiness": "RESIDUAL",
        }],
        "scope": {
            "organization_repository_count": 1,
            "active_repository_count": 1,
            "archived_repository_count": 0,
        },
        "release_decision": {"status": "BLOCKED"},
    }


class CatalogValidatorTests(unittest.TestCase):
    def test_valid_residual_catalog_is_structurally_accepted(self):
        errors, metrics = validate(valid_catalog())
        self.assertEqual(errors, [])
        self.assertEqual(metrics["validation"], "PASS_STRUCTURAL_ONLY")
        self.assertEqual(metrics["functions_total"], 1)

    def test_invalid_execution_model_is_rejected(self):
        catalog = valid_catalog()
        catalog["components"][0]["execution_model"] = "SERVER"
        errors, _ = validate(catalog)
        self.assertTrue(any("execution_model" in e for e in errors))

    def test_missing_execution_model_rationale_is_rejected(self):
        catalog = valid_catalog()
        del catalog["components"][0]["execution_model_rationale"]
        errors, _ = validate(catalog)
        self.assertTrue(any("execution_model_rationale" in e for e in errors))

    def test_function_id_must_follow_stable_traceable_convention(self):
        doc = valid_catalog()
        doc["components"][0]["functions"][0]["id"] = "execute"
        errors, _ = validate(doc)
        self.assertTrue(any("function id must follow the stable" in error for error in errors))

    def test_namespaced_function_id_is_accepted(self):
        doc = valid_catalog()
        doc["components"][0]["functions"][0]["id"] = "ATC-FUNC-AURORA-MEM-001"
        errors, _ = validate(doc)
        self.assertFalse(any("function id must follow the stable" in error for error in errors))

    def test_source_mapped_function_requires_explicit_function_symbol_kind(self):
        doc = valid_catalog()
        doc["components"][0]["functions"][0]["source_binding"].pop("symbol_kind")
        errors, _ = validate(doc)
        self.assertTrue(any("source_binding.symbol_kind" in error for error in errors))

    def test_type_symbol_cannot_be_declared_function_level_mapping(self):
        doc = valid_catalog()
        doc["components"][0]["functions"][0]["source_binding"]["symbol_kind"] = "type"
        errors, _ = validate(doc)
        self.assertTrue(any("function-level source mapping cannot use symbol_kind" in error for error in errors))

    def test_interface_requires_machine_readable_contract_fields(self):
        doc = valid_catalog()
        doc["cross_component_interfaces"][0].pop("error_timeout_semantics")
        errors, _ = validate(doc)
        self.assertTrue(any("missing interface contract fields" in error for error in errors))

    def test_blocked_interface_requires_explicit_reason(self):
        doc = valid_catalog()
        doc["cross_component_interfaces"][0]["status"] = "BLOCKED"
        doc["cross_component_interfaces"][0]["status_reason"] = ""
        errors, _ = validate(doc)
        self.assertTrue(any("missing required field status_reason" in error for error in errors))


if __name__ == "__main__":
    unittest.main()
