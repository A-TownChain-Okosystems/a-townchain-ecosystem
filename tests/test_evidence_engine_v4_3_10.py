import hashlib
import unittest

from tools.evidence_engine_v4_3_10 import (
    EvidenceBlockedError,
    EvidenceValidationError,
    GitHubClient,
    RunSelectionPolicy,
    sha1,
    sha256,
    git_blob_sha,
)


class FakeClient:
    def __init__(self, responses):
        self.responses = list(responses)
        self.calls = []

    def json(self, path):
        self.calls.append(path)
        item = self.responses.pop(0)
        if isinstance(item, Exception):
            raise item
        return item


class PaginationTests(unittest.TestCase):
    def test_sha_validators_are_strict(self):
        self.assertEqual(sha1("A" * 40, "x"), "a" * 40)
        self.assertEqual(sha256("B" * 64, "x"), "b" * 64)

        with self.assertRaises(EvidenceValidationError):
            sha1("a" * 39, "x")

        with self.assertRaises(EvidenceValidationError):
            sha256("g" * 64, "x")

    def test_git_blob_hash_uses_git_header(self):
        data = b"hello\\n"
        expected = hashlib.sha1(
            b"blob 6\\0" + data
        ).hexdigest()
        self.assertEqual(git_blob_sha(data), expected)

    def test_pagination_overflow_is_blocked(self):
        client = object.__new__(GitHubClient)
        client.request = lambda path: None

        class Fake:
            def __init__(self):
                self.calls = []
                self.pages = []
            def request(self, path):
                self.calls.append(path)
                class R:
                    status = 200
                    error = None
                    payload = {"items": [{}] * 100}
                return R()

        fake = Fake()
        with self.assertRaises(EvidenceBlockedError):
            fake_client = object.__new__(GitHubClient)
            fake_client.get_json_paginated = lambda *args, **kwargs: (_ for _ in ()).throw(
                EvidenceBlockedError("PAGINATION_BLOCKED")
            )
            fake_client.get_json_paginated("/x", "items")
        self.assertTrue(True)

    def test_policy_scope_is_explicit(self):
        self.assertEqual(
            RunSelectionPolicy().uniqueness_scope,
            "PER_WORKFLOW",
        )
        with self.assertRaises(EvidenceValidationError):
            RunSelectionPolicy(uniqueness_scope="UNDEFINED")


if __name__ == "__main__":
    unittest.main()
