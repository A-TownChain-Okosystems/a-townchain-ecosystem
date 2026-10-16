#!/usr/bin/env python3
"""Regression tests for live repository HEAD/root-tree drift comparison."""
from __future__ import annotations

import copy
import unittest
import urllib.error

from validate_org_repository_drift import compare

SHA_A = "a" * 40
SHA_B = "b" * 40
TREE_A = "c" * 40
TREE_B = "d" * 40

MANIFEST = {
    "schema_id": "ATC-ORG-REPOSITORY-MANIFEST-001",
    "schema_version": "1.0.0",
    "owner": "ExampleOrg",
    "repositories": [
        {"name": "active-repo", "archived": False, "role": "runtime"},
        {"name": "archived-repo", "archived": True, "role": "reference"},
        {"name": "demo-repository", "archived": False, "private": True, "role": "demo/reference"},
    ],
}
POLICY = {
    "schema_id": "ATC-ORG-REPOSITORY-DRIFT-SCOPE-001",
    "schema_version": "1.0.0",
    "default_policy": {
        "active_repository": "INCLUDE",
        "archived_repository": "EXCLUDE",
        "private_repository": "NO_OVERRIDE",
        "unknown_repository": "BLOCK",
        "unknown_scope_action": "BLOCK",
        "reason_required_for_every_explicit_override": True,
    },
    "explicit_overrides": [{
        "name": "demo-repository",
        "decision": "EXCLUDE",
        "reason_code": "TEMPLATE_FIXTURE_NOT_PRODUCTION",
        "reason": "Fixture is not production.",
        "expected_archived": False,
        "expected_private": True,
        "expected_role": "demo/reference",
    }],
}
INTEGRATION = {
    "schema_id": "ATC-ORG-REPOSITORY-INTEGRATION-001",
    "schema_version": "1.0.0",
    "snapshot_date": "2026-10-11",
    "repositories": [{
        "full_name": "ExampleOrg/active-repo",
        "default_branch": "main",
        "observed_archive_state": "ACTIVE",
        "observed_head_sha": SHA_A,
        "observed_tree_sha": TREE_A,
    }, {
        "full_name": "ExampleOrg/archived-repo",
        "default_branch": "main",
        "observed_archive_state": "ARCHIVED",
        "observed_head_sha": SHA_A,
        "observed_tree_sha": TREE_A,
    }, {
        "full_name": "ExampleOrg/demo-repository",
        "default_branch": "main",
        "observed_archive_state": "ACTIVE",
        "observed_head_sha": SHA_A,
        "observed_tree_sha": TREE_A,
    }],
}


def live_payloads(head=SHA_A, tree=TREE_A, branch="main", archived=False):
    return {
        "https://api.github.com/repos/ExampleOrg/active-repo": {
            "default_branch": branch, "archived": archived,
        },
        f"https://api.github.com/repos/ExampleOrg/active-repo/commits/{branch}": {
            "sha": head, "commit": {"tree": {"sha": tree}},
        },
    }


class DriftValidatorTests(unittest.TestCase):
    def test_exact_head_and_tree_match(self):
        payloads = live_payloads()
        report = compare(MANIFEST, POLICY, INTEGRATION, fetch=lambda url, token=None: payloads[url])
        self.assertEqual(report["validation"], "NO_DRIFT_OBSERVED")
        self.assertEqual(report["repositories_matched"], 1)
        self.assertEqual(report["repositories_excluded"], 2)
        self.assertEqual(report["entries"][0]["status"], "MATCH")

    def test_head_or_tree_change_is_drift_not_verified(self):
        payloads = live_payloads(head=SHA_B, tree=TREE_B)
        report = compare(MANIFEST, POLICY, INTEGRATION, fetch=lambda url, token=None: payloads[url])
        self.assertEqual(report["validation"], "DRIFT_DETECTED")
        self.assertEqual(report["entries"][0]["status"], "DRIFT")
        self.assertNotIn("VERIFIED", {e["status"] for e in report["entries"]})

    def test_default_branch_change_is_drift(self):
        payloads = live_payloads(branch="stable")
        report = compare(MANIFEST, POLICY, INTEGRATION, fetch=lambda url, token=None: payloads[url])
        self.assertEqual(report["entries"][0]["reason_code"], "DEFAULT_BRANCH_CHANGED")

    def test_api_failure_blocks_and_fails_closed(self):
        def fail(url, token=None):
            raise urllib.error.HTTPError(url, 403, "forbidden", {}, None)
        report = compare(MANIFEST, POLICY, INTEGRATION, fetch=fail)
        self.assertEqual(report["validation"], "BLOCKED")
        self.assertEqual(report["entries"][0]["status"], "BLOCKED")
        self.assertEqual(report["entries"][0]["reason_code"], "LIVE_GITHUB_QUERY_FAILED")

    def test_invalid_baseline_sha_blocks(self):
        integration = copy.deepcopy(INTEGRATION)
        integration["repositories"][0]["observed_head_sha"] = "short"
        report = compare(MANIFEST, POLICY, integration, fetch=lambda url, token=None: {})
        self.assertEqual(report["entries"][0]["reason_code"], "INVALID_BASELINE_IDENTITY")
        self.assertEqual(report["validation"], "BLOCKED")

    def test_demo_fixture_is_excluded_without_live_query(self):
        def unexpected(url, token=None):
            raise AssertionError("excluded fixture must not be queried")
        report = compare(MANIFEST, POLICY, INTEGRATION, fetch=unexpected)
        demo = next(e for e in report["entries"] if e["repository"].endswith("/demo-repository"))
        self.assertEqual(demo["status"], "EXCLUDED")
        self.assertEqual(demo["reason_code"], "TEMPLATE_FIXTURE_NOT_PRODUCTION")

    def test_archive_state_mismatch_blocks(self):
        payloads = live_payloads(archived=True)
        report = compare(MANIFEST, POLICY, INTEGRATION, fetch=lambda url, token=None: payloads[url])
        self.assertEqual(report["validation"], "BLOCKED")
        self.assertEqual(report["entries"][0]["reason_code"], "REPOSITORY_METADATA_MISMATCH")


if __name__ == "__main__":
    unittest.main()
