import unittest

from tools.validate_org_repository_drift import validate


HEAD = "a" * 40
TREE = "b" * 40


def manifest():
    return {
        "owner": "example",
        "repositories": [{
            "name": "sample",
            "archived": False,
            "observed_head_sha": HEAD,
            "observed_root_tree_sha": TREE,
        }],
    }


class OrganizationRepositoryDriftTests(unittest.TestCase):
    def test_matching_live_head_and_tree_pass(self):
        report = validate(manifest(), None, lambda owner, name, token: {
            "head_sha": HEAD, "root_tree_sha": TREE,
        })
        self.assertEqual(report["status"], "PASS_NO_DRIFT")
        self.assertEqual(report["summary"], {"MATCH": 1, "DRIFT": 0, "BLOCKED": 0})
        self.assertFalse(report["pins_rewritten"])

    def test_head_or_tree_mismatch_blocks(self):
        report = validate(manifest(), None, lambda owner, name, token: {
            "head_sha": "c" * 40, "root_tree_sha": TREE,
        })
        self.assertEqual(report["status"], "BLOCKED")
        self.assertEqual(report["repositories"][0]["status"], "DRIFT")
        self.assertEqual(report["summary"]["DRIFT"], 1)

    def test_api_failure_blocks_instead_of_skipping_repository(self):
        def unavailable(owner, name, token):
            raise RuntimeError("HTTP 403")
        report = validate(manifest(), None, unavailable)
        self.assertEqual(report["status"], "BLOCKED")
        self.assertEqual(report["summary"], {"MATCH": 0, "DRIFT": 0, "BLOCKED": 1})
        self.assertIn("API_OR_RESPONSE_ERROR", report["repositories"][0]["error"])

    def test_malformed_expected_sha_blocks(self):
        data = manifest()
        data["repositories"][0]["observed_head_sha"] = "short"
        report = validate(data, None, lambda *args: self.fail("API must not be called"))
        self.assertEqual(report["status"], "BLOCKED")
        self.assertEqual(report["repositories"][0]["error"], "INVALID_EXPECTED_HEAD_SHA")


if __name__ == "__main__":
    unittest.main()
