"""Focused fail-closed tests for source evidence inspection."""
from __future__ import annotations

import unittest

from tools.classify_unbound_candidates import inspect_source_location


SHA = "a" * 40
FILE_HASH = "b" * 64
SYMBOL = {"kind": "function", "name": "run", "line_start": 12, "signature": "fn run()"}
ROW = {
    "repository": "owner/repo",
    "source_sha": SHA,
    "source_path": "src/lib.rs",
    "source_file_sha256": FILE_HASH,
    "symbol_kind": "function",
    "symbol_or_entrypoint": "run",
    "line_start": 12,
}
REPOS = {
    "owner/repo": {
        "source_sha": SHA,
        "report": {
            "files": [{
                "path": "src/lib.rs",
                "file_sha256": FILE_HASH,
                "candidate_public_symbols": [SYMBOL],
                "candidate_internal_symbols": [],
                "candidate_test_symbols": [],
            }]
        },
    }
}


class InspectSourceLocationTests(unittest.TestCase):
    def inspect(self, row=None, repos=None):
        return inspect_source_location(dict(ROW if row is None else row),
                                       REPOS if repos is None else repos)

    def test_exact_sha_path_hash_and_symbol_confirm_location(self):
        self.assertEqual(self.inspect()["status"], "SOURCE_LOCATION_CONFIRMED")

    def test_repository_sha_mismatch_fails_closed(self):
        row = dict(ROW, source_sha="c" * 40)
        self.assertEqual(self.inspect(row)["status"], "SOURCE_SHA_MISMATCH")

    def test_missing_census_path_fails_closed(self):
        row = dict(ROW, source_path="src/missing.rs")
        self.assertEqual(self.inspect(row)["status"], "SOURCE_PATH_NOT_FOUND_IN_CENSUS")

    def test_missing_candidate_file_hash_fails_closed(self):
        row = dict(ROW)
        del row["source_file_sha256"]
        self.assertEqual(self.inspect(row)["status"], "SOURCE_FILE_HASH_MISSING")

    def test_mismatched_file_hash_fails_closed(self):
        row = dict(ROW, source_file_sha256="c" * 64)
        self.assertEqual(self.inspect(row)["status"], "SOURCE_FILE_HASH_MISMATCH")

    def test_missing_census_file_hash_fails_closed(self):
        repos = {
            "owner/repo": {
                "source_sha": SHA,
                "report": {"files": [{
                    "path": "src/lib.rs",
                    "candidate_public_symbols": [SYMBOL],
                    "candidate_internal_symbols": [],
                    "candidate_test_symbols": [],
                }]},
            }
        }
        self.assertEqual(self.inspect(repos=repos)["status"], "CENSUS_FILE_HASH_MISSING")

    def test_symbol_kind_mismatch_fails_closed(self):
        row = dict(ROW, symbol_kind="method")
        self.assertEqual(self.inspect(row)["status"], "SYMBOL_NOT_FOUND_AT_RECORDED_LOCATION")

    def test_symbol_name_mismatch_fails_closed(self):
        row = dict(ROW, symbol_or_entrypoint="stop")
        self.assertEqual(self.inspect(row)["status"], "SYMBOL_NOT_FOUND_AT_RECORDED_LOCATION")

    def test_symbol_line_mismatch_fails_closed(self):
        row = dict(ROW, line_start=13)
        self.assertEqual(self.inspect(row)["status"], "SYMBOL_NOT_FOUND_AT_RECORDED_LOCATION")

    def test_invalid_symbol_position_fails_closed(self):
        row = dict(ROW, line_start=0)
        self.assertEqual(self.inspect(row)["status"], "INVALID_SYMBOL_POSITION")

    def test_duplicate_exact_symbol_match_is_ambiguous(self):
        repos = {
            "owner/repo": {
                "source_sha": SHA,
                "report": {"files": [{
                    "path": "src/lib.rs",
                    "file_sha256": FILE_HASH,
                    "candidate_public_symbols": [SYMBOL],
                    "candidate_internal_symbols": [SYMBOL],
                    "candidate_test_symbols": [],
                }]},
            }
        }
        self.assertEqual(self.inspect(repos=repos)["status"], "AMBIGUOUS_SYMBOL_MATCH")

    def test_missing_repository_or_report_fails_closed(self):
        self.assertEqual(self.inspect(repos={})["status"], "SOURCE_EVIDENCE_UNAVAILABLE")


if __name__ == "__main__":
    unittest.main()

class ExecutionModelTests(unittest.TestCase):
    def test_main_entrypoint_is_runtime(self):
        from tools.classify_unbound_candidates import execution_model
        model, _ = execution_model("src/main.rs", "rust", "main")
        self.assertEqual(model, "RUNTIME")

    def test_bin_directory_is_runtime(self):
        from tools.classify_unbound_candidates import execution_model
        model, _ = execution_model("bin/node/start.rs", "rust", "start")
        self.assertEqual(model, "RUNTIME")

    def test_plain_module_defaults_to_library(self):
        from tools.classify_unbound_candidates import execution_model
        model, _ = execution_model("src/lib/codec.rs", "rust", "encode_frame")
        self.assertEqual(model, "LIBRARY")

    def test_test_surface_is_not_executable_inventory(self):
        from tools.classify_unbound_candidates import execution_model
        model, _ = execution_model("tests/test_net.py", "python", "test_roundtrip")
        self.assertEqual(model, "NOT_EXECUTABLE_INVENTORY")

