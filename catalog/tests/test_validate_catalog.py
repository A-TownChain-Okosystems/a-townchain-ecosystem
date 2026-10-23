from __future__ import annotations

import json
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

SOURCE_ROOT = Path(__file__).resolve().parents[2]
SOURCE_CATALOG = SOURCE_ROOT / "catalog"
VALIDATOR = SOURCE_CATALOG / "scripts" / "validate_catalog.py"


class CatalogValidatorNegativeTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        shutil.copytree(SOURCE_CATALOG, self.root / "catalog")
        self.validator = self.root / "catalog" / "scripts" / "validate_catalog.py"
        self.catalog = self.root / "catalog"

    def tearDown(self) -> None:
        self.temp.cleanup()

    def read_json(self, name: str):
        return json.loads((self.catalog / name).read_text(encoding="utf-8"))

    def write_json(self, name: str, value) -> None:
        (self.catalog / name).write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")

    def run_validator(self) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [sys.executable, str(self.validator)],
            cwd=self.root,
            text=True,
            capture_output=True,
            check=False,
        )

    def test_valid_fixture_passes(self) -> None:
        result = self.run_validator()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn('"status": "PASS"', result.stdout)

    def test_invalid_repository_commit_sha_fails(self) -> None:
        manifest = self.read_json("repositories.json")
        manifest["repositories"][0]["head_commit_sha"] = "bad-sha"
        self.write_json("repositories.json", manifest)
        result = self.run_validator()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("invalid head_commit_sha", result.stderr)

    def test_duplicate_repository_id_fails(self) -> None:
        manifest = self.read_json("repositories.json")
        manifest["repositories"][1]["repository"] = manifest["repositories"][0]["repository"]
        self.write_json("repositories.json", manifest)
        result = self.run_validator()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("not unique", result.stderr)

    def test_truncated_tree_fails(self) -> None:
        manifest = self.read_json("repositories.json")
        manifest["repositories"][0]["tree_truncated"] = True
        self.write_json("repositories.json", manifest)
        result = self.run_validator()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("tree_truncated must be false", result.stderr)

    def test_demo_live_gate_exclusion_regression_fails(self) -> None:
        manifest = self.read_json("repositories.json")
        demo = next(item for item in manifest["repositories"] if item["repository"].endswith("/demo-repository"))
        demo["exclusion_from_live_drift_gate"] = False
        self.write_json("repositories.json", manifest)
        result = self.run_validator()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("demo-repository must be excluded", result.stderr)

    def test_coverage_total_mismatch_fails(self) -> None:
        coverage = self.read_json("coverage.json")
        coverage["totals"]["tracked_files_all_repositories"] += 1
        self.write_json("coverage.json", coverage)
        result = self.run_validator()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("tracked file total does not match", result.stderr)

    def test_missing_required_file_record_field_fails(self) -> None:
        path = self.catalog / "files.jsonl"
        record = json.loads(path.read_text(encoding="utf-8").splitlines()[0])
        del record["blob_sha"]
        path.write_text(json.dumps(record) + "\n", encoding="utf-8")
        coverage = self.read_json("coverage.json")
        coverage["file_level_records"] = 1
        self.write_json("coverage.json", coverage)
        result = self.run_validator()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("required property", result.stderr)

    def test_malformed_jsonl_fails(self) -> None:
        path = self.catalog / "files.jsonl"
        path.write_text("{not-json}\n", encoding="utf-8")
        coverage = self.read_json("coverage.json")
        coverage["file_level_records"] = 1
        self.write_json("coverage.json", coverage)
        result = self.run_validator()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("is not valid JSON", result.stderr)


if __name__ == "__main__":
    unittest.main(verbosity=2)
