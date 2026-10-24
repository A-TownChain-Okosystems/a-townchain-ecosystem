import importlib.util
import json
import tempfile
import unittest
from pathlib import Path

from jsonschema import Draft202012Validator

MODULE_PATH = Path(__file__).resolve().parents[1] / "scripts" / "validate_engineering_handoff.py"
SPEC = importlib.util.spec_from_file_location("validate_engineering_handoff", MODULE_PATH)
validator = importlib.util.module_from_spec(SPEC)
assert SPEC and SPEC.loader
SPEC.loader.exec_module(validator)


class EngineeringHandoffTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.sha = "a" * 40
        self.census = self.root / "repositories.json"
        self.census.write_text(json.dumps({
            "repositories": [{
                "repository": "A-TownChain-Okosystems/atc-engineering",
                "head_commit_sha": self.sha
            }]
        }), encoding="utf-8")
        self.manifest = {
            "schema_version": "atc.build-evidence/v1",
            "repository": "A-TownChain-Okosystems/atc-engineering",
            "source_sha": self.sha,
            "checked_out_sha": self.sha,
            "event_name": "push",
            "event_ref": "refs/heads/ci/exact-head-sha-pr32",
            "pr_head_sha": "N/A (push event: no PR-head context)",
            "release_binary_path": "target/release/atc-audit-cli",
            "release_binary_sha256": "b" * 64,
            "workflow_ref": "A-TownChain-Okosystems/atc-engineering/.github/workflows/engineering-ci.yml@refs/heads/main",
            "run_id": 123,
            "run_attempt": 1,
            "gate_report_path": "artifacts/gate-report.json",
            "gate_report_sha256": "c" * 64,
            "gate_report_schema": "atc.gate-report/v1"
        }

    def tearDown(self):
        self.temp.cleanup()

    def validate(self, manifest=None, expected_sha=None):
        return validator.validate_manifest(manifest or self.manifest, self.census, expected_sha)

    def test_manifest_matches_shared_json_schema(self):
        schema_path = Path(__file__).resolve().parents[1] / "schema" / "atc-build-evidence-handoff.schema.json"
        schema = json.loads(schema_path.read_text(encoding="utf-8"))
        Draft202012Validator(schema).validate(self.manifest)

    def test_accepts_exact_census_binding(self):
        result = self.validate()
        self.assertEqual(result["status"], "PASS_SCHEMA_AND_CENSUS_BINDING")
        self.assertEqual(result["functional_verification"], "NOT_INFERRED")

    def test_rejects_stale_head(self):
        self.census.write_text(json.dumps({"repositories": [{
            "repository": "A-TownChain-Okosystems/atc-engineering",
            "head_commit_sha": "d" * 40
        }]}), encoding="utf-8")
        with self.assertRaisesRegex(ValueError, "stale evidence"):
            self.validate()

    def test_rejects_mismatched_checkout(self):
        manifest = dict(self.manifest, checked_out_sha="d" * 40)
        with self.assertRaisesRegex(ValueError, "source_sha and checked_out_sha differ"):
            self.validate(manifest)

    def test_rejects_wrong_expected_sha(self):
        with self.assertRaisesRegex(ValueError, "explicitly expected head"):
            self.validate(expected_sha="d" * 40)

    def test_rejects_missing_digest(self):
        manifest = dict(self.manifest)
        del manifest["gate_report_sha256"]
        with self.assertRaisesRegex(ValueError, "gate_report_sha256.*required property"):
            self.validate(manifest)

    def test_rejects_invalid_digest(self):
        manifest = dict(self.manifest, release_binary_sha256="not-a-digest")
        with self.assertRaisesRegex(ValueError, "release_binary_sha256.*64"):
            self.validate(manifest)

    def test_rejects_push_without_explicit_na(self):
        manifest = dict(self.manifest, pr_head_sha="d" * 40)
        with self.assertRaisesRegex(ValueError, "explicit N/A PR-head context"):
            self.validate(manifest)

    def test_rejects_tag_ref_for_push(self):
        manifest = dict(self.manifest, event_ref="refs/tags/v1")
        with self.assertRaisesRegex(ValueError, "branch ref"):
            self.validate(manifest)


if __name__ == "__main__":
    unittest.main()
