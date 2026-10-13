import unittest

from tools.report_cross_repository_hashes import build_report


class CrossRepositoryHashReportTests(unittest.TestCase):
    def test_same_hash_across_repositories_is_reported_not_merged(self):
        digest = "a" * 64
        census = {"repositories": [
            {"repository": "a-townchain", "source_sha": "1" * 40, "report": {"files": [
                {"path": "src/vm.rs", "file_sha256": digest,
                 "candidate_public_symbols": [{"kind": "function", "name": "execute", "line_start": 7}]}
            ]}},
            {"repository": "atc-vm", "source_sha": "2" * 40, "report": {"files": [
                {"path": "components/vm/src/vm.rs", "file_sha256": digest,
                 "candidate_public_symbols": [{"kind": "function", "name": "execute", "line_start": 7}]}
            ]}},
        ]}
        result = build_report(census)
        self.assertEqual(result["summary"]["cross_repository_hash_groups"], 1)
        self.assertEqual(result["summary"]["cross_repository_candidate_rows"], 2)
        self.assertEqual(result["groups"][0]["decision"], "REVIEW_REQUIRED_NO_AUTOMATIC_BINDING")
        self.assertEqual(result["groups"][0]["repository_count"], 2)
        self.assertFalse(result["semantics"]["automatic_candidate_merge"])
        self.assertFalse(result["semantics"]["functional_verification"])

    def test_same_hash_within_one_repository_is_not_cross_repository(self):
        digest = "b" * 64
        census = {"repositories": [
            {"repository": "repo-a", "source_sha": "1" * 40, "report": {"files": [
                {"path": "a.rs", "file_sha256": digest},
                {"path": "b.rs", "file_sha256": digest},
            ]}},
        ]}
        result = build_report(census)
        self.assertEqual(result["summary"]["cross_repository_hash_groups"], 0)
        self.assertEqual(result["groups"], [])


if __name__ == "__main__":
    unittest.main()
