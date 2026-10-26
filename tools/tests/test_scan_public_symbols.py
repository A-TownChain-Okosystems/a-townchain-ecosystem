import tempfile
import unittest
from contextlib import redirect_stderr
from io import StringIO
import subprocess
import sys
from unittest.mock import patch
from pathlib import Path

from tools.scan_public_symbols import main, scan


class PublicSymbolScanTests(unittest.TestCase):
    def test_rust_exports_are_discovered_without_claiming_verification(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "src").mkdir()
            (root / "src" / "lib.rs").write_text(
                "pub struct Api;\npub fn execute() {}\nfn private_helper() {}\n",
                encoding="utf-8",
            )
            report = scan(root, "example/repo", "a" * 40)
            symbols = {
                (entry["kind"], entry["name"])
                for file in report["files"]
                for entry in file["candidate_public_symbols"]
            }
            self.assertIn(("type", "Api"), symbols)
            self.assertIn(("function", "execute"), symbols)
            self.assertNotIn(("function", "private_helper"), symbols)
            internal = {
                (entry["kind"], entry["name"])
                for file in report["files"]
                for entry in file["candidate_internal_symbols"]
            }
            self.assertIn(("function", "private_helper"), internal)
            self.assertEqual(report["completeness_status"], "RESIDUAL")
            self.assertEqual(report["functional_verification"], "NOT_PERFORMED")



    def test_helpers_in_test_paths_are_not_runtime_candidates(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "tests").mkdir()
            (root / "tests" / "stdlib_reference_conformance.rs").write_text(
                "fn encode_be(value: u64) -> Vec<u8> { value.to_be_bytes().to_vec() }\n"
                "#[test]\n"
                "fn test_reference_vector() { assert_eq!(encode_be(1).len(), 8); }\n",
                encoding="utf-8",
            )
            report = scan(root, "example/repo", "b" * 40)
            file = report["files"][0]
            self.assertEqual(file["candidate_public_symbols"], [])
            self.assertEqual(file["candidate_internal_symbols"], [])
            names = {x["name"] for x in file["candidate_test_symbols"]}
            self.assertIn("encode_be", names)
            self.assertIn("test_reference_vector", names)

    def test_duplicate_rust_functions_keep_distinct_line_identity(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "src").mkdir()
            (root / "src" / "lib.rs").write_text(
                "impl First {\n"
                "  pub fn new() -> Self { Self }\n"
                "}\n"
                "impl Second {\n"
                "  pub fn new() -> Self { Self }\n"
                "}\n",
                encoding="utf-8",
            )
            report = scan(root, "example/repo", "a" * 40)
            candidates = [x for x in report["files"][0]["candidate_public_symbols"] if x["name"] == "new"]
            self.assertEqual(len(candidates), 2)
            self.assertNotEqual(candidates[0]["line_start"], candidates[1]["line_start"])
            self.assertTrue(all(x["signature"].startswith("pub fn new") for x in candidates))

    def test_typescript_internal_helpers_are_discovered_separately(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "src").mkdir()
            (root / "src" / "chain.ts").write_text(
                "function pushU64BE(out: number[], value: bigint): void {\n"
                "  out.push(Number(value));\n"
                "}\n"
                "export function canonicalSigningPreimage(): Uint8Array {\n"
                "  return new Uint8Array();\n"
                "}\n"
                "export class Api {\n"
                "  private decode(value: string): string {\n"
                "    return value;\n"
                "  }\n"
                "  public encode(value: string): string {\n"
                "    return value;\n"
                "  }\n"
                "}\n",
                encoding="utf-8",
            )
            report = scan(root, "example/repo", "f" * 40)
            file = report["files"][0]
            public = {(x["kind"], x["name"]) for x in file["candidate_public_symbols"]}
            internal = {(x["kind"], x["name"]) for x in file["candidate_internal_symbols"]}
            self.assertIn(("function", "canonicalSigningPreimage"), public)
            self.assertIn(("function", "encode"), public)
            self.assertIn(("function", "pushU64BE"), internal)
            self.assertIn(("function", "decode"), internal)

    def test_typescript_class_methods_are_discovered_as_function_candidates(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "src").mkdir()
            (root / "src" / "matcher.ts").write_text(
                "export class OrderMatcher {\n"
                "  match(newOrder: Order, book: Orderbook): { matched: Order[] } {\n"
                "    return { matched: [] };\n"
                "  }\n"
                "}\n",
                encoding="utf-8",
            )
            report = scan(root, "example/repo", "e" * 40)
            symbols = {(x["kind"], x["name"]) for x in report["files"][0]["candidate_public_symbols"]}
            self.assertIn(("type", "OrderMatcher"), symbols)
            self.assertIn(("function", "match"), symbols)

    def test_rust_inline_cfg_test_module_is_separated_from_runtime_candidates(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "src").mkdir()
            (root / "src" / "lib.rs").write_text(
                "pub fn run() {}\n"
                "#[cfg(test)]\nmod tests {\n"
                "  #[test] fn regression_case() {}\n"
                "  fn helper_only_for_tests() {}\n"
                "}\n",
                encoding="utf-8",
            )
            report = scan(root, "example/repo", "d" * 40)
            file = report["files"][0]
            runtime = {(x["kind"], x["name"]) for x in file["candidate_public_symbols"]}
            internal = {(x["kind"], x["name"]) for x in file["candidate_internal_symbols"]}
            test_symbols = {x["name"] for x in file["candidate_test_symbols"]}
            self.assertIn(("function", "run"), runtime)
            self.assertNotIn(("function", "regression_case"), runtime)
            self.assertNotIn(("function", "helper_only_for_tests"), internal)
            self.assertIn("regression_case", test_symbols)
            self.assertIn("helper_only_for_tests", test_symbols)
            self.assertEqual(report["summary"]["candidate_test_symbols"], 2)

    def test_documented_scope_exclusion_records_reason_and_count(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "archive").mkdir()
            (root / "archive" / "old.rs").write_text("pub fn old_api() {}\n", encoding="utf-8")
            (root / "src").mkdir()
            (root / "src" / "lib.rs").write_text("pub fn live_api() {}\n", encoding="utf-8")
            report = scan(root, "example/repo", "a" * 40, [{"path": "archive/", "reason": "historical mirror"}])
            paths = {entry["path"] for entry in report["files"]}
            self.assertIn("src/lib.rs", paths)
            self.assertNotIn("archive/old.rs", paths)
            self.assertEqual(report["summary"]["files_excluded_by_scope"], 1)
            self.assertEqual(report["scope_exclusions"][0]["files_excluded"], 1)
            self.assertEqual(report["scope_exclusions"][0]["reason"], "historical mirror")

    def test_python_private_names_are_excluded(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "api.py").write_text(
                "def run():\n    pass\ndef _helper():\n    pass\nclass Client:\n    pass\n",
                encoding="utf-8",
            )
            report = scan(root, "example/python-api", "b" * 40)
            symbols = {
                (entry["kind"], entry["name"])
                for file in report["files"]
                for entry in file["candidate_public_symbols"]
            }
            self.assertIn(("function", "run"), symbols)
            self.assertIn(("type", "Client"), symbols)
            self.assertNotIn(("function", "_helper"), symbols)

    def test_cli_rejects_source_sha_that_differs_from_git_head(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "lib.rs").write_text("pub fn execute() {}\n", encoding="utf-8")
            subprocess.run(["git", "-C", str(root), "init"], check=True, capture_output=True)
            subprocess.run(["git", "-C", str(root), "config", "user.email", "test@example.invalid"], check=True)
            subprocess.run(["git", "-C", str(root), "config", "user.name", "Scanner Test"], check=True)
            subprocess.run(["git", "-C", str(root), "add", "lib.rs"], check=True)
            subprocess.run(["git", "-C", str(root), "commit", "-m", "test: source fixture"], check=True, capture_output=True)
            args = ["scan_public_symbols.py", "--root", str(root), "--repository", "example/repo",
                    "--source-sha", "0" * 40, "--output", str(root / "report.json")]
            error = StringIO()
            with patch.object(sys, "argv", args), redirect_stderr(error):
                with self.assertRaises(SystemExit) as raised:
                    main()
            self.assertEqual(raised.exception.code, 2)
            self.assertIn("source-sha mismatch", error.getvalue())
            self.assertFalse((root / "report.json").exists())

    def test_typescript_exported_entrypoints_are_discovered(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "index.ts").write_text(
                "export function execute() {}\nexport class Client {}\nexport const VERSION = '1';\n",
                encoding="utf-8",
            )
            report = scan(root, "example/typescript-api", "c" * 40)
            symbols = {
                (entry["kind"], entry["name"])
                for file in report["files"]
                for entry in file["candidate_public_symbols"]
            }
            self.assertIn(("function", "execute"), symbols)
            self.assertIn(("type", "Client"), symbols)
            self.assertIn(("value", "VERSION"), symbols)


if __name__ == "__main__":
    unittest.main()
