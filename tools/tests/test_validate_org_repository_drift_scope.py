#!/usr/bin/env python3
"""Regression tests for versioned live-drift scope policy."""
from __future__ import annotations

import copy
import json
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools"))
from validate_org_repository_drift_scope import resolve  # noqa: E402

MANIFEST = json.loads((ROOT / "docs/audit/ATC-ORG-REPOSITORY-MANIFEST-001.json").read_text())
POLICY = json.loads((ROOT / "docs/audit/ATC-ORG-REPOSITORY-DRIFT-SCOPE-001.json").read_text())


class DriftScopeTests(unittest.TestCase):
    def test_demo_fixture_explicitly_excluded(self):
        errors, report = resolve(MANIFEST, POLICY)
        self.assertEqual(errors, [])
        demo = next(item for item in report["entries"] if item["name"] == "demo-repository")
        self.assertEqual(demo["decision"], "EXCLUDE")
        self.assertEqual(demo["reason_code"], "TEMPLATE_FIXTURE_NOT_PRODUCTION")

    def test_active_repository_included(self):
        manifest = copy.deepcopy(MANIFEST)
        repo = next(item for item in manifest["repositories"] if item["name"] == "a-townchain")
        errors, report = resolve(manifest, POLICY)
        self.assertEqual(errors, [])
        item = next(item for item in report["entries"] if item["name"] == repo["name"])
        self.assertEqual(item["decision"], "INCLUDE")

    def test_archived_repository_excluded_by_default(self):
        errors, report = resolve(MANIFEST, POLICY)
        self.assertEqual(errors, [])
        item = next(item for item in report["entries"] if item["name"] == "atc-wallet")
        self.assertEqual(item["decision"], "EXCLUDE")

    def test_private_flag_alone_does_not_exclude(self):
        manifest = copy.deepcopy(MANIFEST)
        repo = next(item for item in manifest["repositories"] if item["name"] == "a-townchain")
        repo["private"] = True
        errors, report = resolve(manifest, POLICY)
        self.assertEqual(errors, [])
        item = next(item for item in report["entries"] if item["name"] == repo["name"])
        self.assertEqual(item["decision"], "INCLUDE")

    def test_missing_private_metadata_is_not_assumed_public(self):
        manifest = copy.deepcopy(MANIFEST)
        repo = next(item for item in manifest["repositories"] if item["name"] == "a-townchain")
        repo.pop("private", None)
        errors, report = resolve(manifest, POLICY)
        self.assertEqual(errors, [])
        item = next(item for item in report["entries"] if item["name"] == repo["name"])
        self.assertEqual(item["decision"], "INCLUDE")
        self.assertIsNone(item["private"])

    def test_unsafe_default_policy_fails_closed(self):
        policy = copy.deepcopy(POLICY)
        policy["default_policy"]["active_repository"] = "EXCLUDE"
        errors, report = resolve(MANIFEST, policy)
        self.assertTrue(any("active repositories must be included" in error for error in errors))
        self.assertEqual(report["validation"], "FAIL")

    def test_unknown_override_repository_fails_closed(self):
        policy = copy.deepcopy(POLICY)
        policy["explicit_overrides"].append({
            "name": "unknown-repository", "decision": "EXCLUDE",
            "reason_code": "TEST", "reason": "must not silently pass"
        })
        errors, report = resolve(MANIFEST, policy)
        self.assertTrue(any("unknown repository" in error for error in errors))
        self.assertEqual(report["validation"], "FAIL")

    def test_override_manifest_mismatch_fails_closed(self):
        manifest = copy.deepcopy(MANIFEST)
        repo = next(item for item in manifest["repositories"] if item["name"] == "demo-repository")
        repo["private"] = False
        errors, report = resolve(manifest, POLICY)
        self.assertTrue(any("expected_private" in error for error in errors))
        self.assertEqual(report["validation"], "FAIL")

    def test_missing_reason_fails_closed(self):
        policy = copy.deepcopy(POLICY)
        policy["explicit_overrides"][0]["reason"] = ""
        errors, report = resolve(MANIFEST, policy)
        self.assertTrue(any("requires a reason" in error for error in errors))
        self.assertEqual(report["validation"], "FAIL")

    def test_invalid_archive_state_fails_closed(self):
        manifest = copy.deepcopy(MANIFEST)
        repo = next(item for item in manifest["repositories"] if item["name"] == "a-townchain")
        repo["archived"] = "false"
        errors, report = resolve(manifest, POLICY)
        self.assertTrue(any("archived must be boolean" in error for error in errors))
        self.assertEqual(report["validation"], "FAIL")


if __name__ == "__main__":
    unittest.main()
