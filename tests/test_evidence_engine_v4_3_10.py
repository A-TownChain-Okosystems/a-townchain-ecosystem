import hashlib
import unittest
from types import SimpleNamespace

from tools.evidence_engine_v4_3_10 import (
    EvidenceBlockedError,
    EvidenceValidationError,
    GitHubClient,
    RunSelectionPolicy,
    sha1,
    sha256,
    git_blob_sha,
    Target,
    Node,
    Edge,
    Ledger,
)


class PagingClient(GitHubClient):
    def __init__(self, responses):
        self.responses = list(responses)
        self.calls = []

    def json(self, path):
        self.calls.append(path)
        response = self.responses.pop(0)
        if response.status < 200 or response.status >= 300:
            raise EvidenceBlockedError(
                f"HTTP {response.status}: {response.error or 'request failed'}"
            )
        return response.payload


def response(status=200, payload=None, error=None):
    return SimpleNamespace(
        status=status,
        payload=payload,
        error=error,
    )


class EvidenceEngineTests(unittest.TestCase):
    def test_sha_validators_are_strict(self):
        self.assertEqual(sha1("A" * 40, "x"), "a" * 40)
        self.assertEqual(sha256("B" * 64, "x"), "b" * 64)

        with self.assertRaises(EvidenceValidationError):
            sha1("a" * 39, "x")

        with self.assertRaises(EvidenceValidationError):
            sha256("g" * 64, "x")

    def test_git_blob_hash_uses_git_header(self):
        data = b"hello\n"
        expected = hashlib.sha1(b"blob 6\0" + data).hexdigest()
        self.assertEqual(git_blob_sha(data), expected)

    def test_pagination_stops_on_partial_page(self):
        client = PagingClient([
            response(200, {"items": [{"id": 1}]}),
        ])
        items = client.paginated("/x", "items", max_pages=1)
        self.assertEqual(items, [{"id": 1}])
        self.assertEqual(len(client.calls), 1)

    def test_pagination_proves_termination_after_full_page(self):
        client = PagingClient([
            response(200, {"items": [{}] * 100}),
            response(200, {"items": []}),
        ])
        items = client.paginated("/x", "items", max_pages=1)
        self.assertEqual(len(items), 100)
        self.assertEqual(len(client.calls), 2)

    def test_pagination_overflow_http_error_is_blocked(self):
        client = PagingClient([
            response(200, {"items": [{}] * 100}),
            response(403, {}, "forbidden"),
        ])
        with self.assertRaises(EvidenceBlockedError):
            client.paginated("/x", "items", max_pages=1)

    def test_pagination_overflow_nonempty_is_blocked(self):
        client = PagingClient([
            response(200, {"items": [{}] * 100}),
            response(200, {"items": [{"id": 101}]}),
        ])
        with self.assertRaises(EvidenceBlockedError):
            client.paginated("/x", "items", max_pages=1)



    def test_node_metadata_is_recursively_immutable(self):
        node = Node("n", "x", "a" * 40, "BLOB", {"nested": {"items": [1, 2]}})
        with self.assertRaises(TypeError):
            node.metadata["nested"]["items"] = ()
        with self.assertRaises(TypeError):
            node.metadata["nested"]["items"][0] = 9

    def test_ledger_freeze_prevents_graph_mutation(self):
        target = Target("owner/repo", "a" * 40, "a" * 40, "b" * 40)
        ledger = Ledger(target)
        ledger.add_node(Node("commit:" + "a" * 40, "", "a" * 40, "COMMIT", {}))
        ledger.add_node(Node("tree:" + "b" * 40, "", "b" * 40, "TREE", {}))
        ledger.add_edge(Edge("commit:" + "a" * 40, "HAS_TREE", "tree:" + "b" * 40))
        ledger.freeze()
        with self.assertRaises(Exception):
            ledger.add_node(Node("x", "", "a" * 40, "BLOB", {}))
        with self.assertRaises(TypeError):
            ledger.nodes["x"] = ledger.nodes["commit:" + "a" * 40]

    def test_policy_scope_is_explicit(self):
        self.assertEqual(
            RunSelectionPolicy().uniqueness_scope,
            "PER_WORKFLOW",
        )
        with self.assertRaises(EvidenceValidationError):
            RunSelectionPolicy(uniqueness_scope="UNDEFINED")


if __name__ == "__main__":
    unittest.main()
