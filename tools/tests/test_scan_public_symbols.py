import tempfile
import unittest
from pathlib import Path

from tools.scan_public_symbols import scan


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
            self.assertEqual(report["completeness_status"], "RESIDUAL")
            self.assertEqual(report["functional_verification"], "NOT_PERFORMED")

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
