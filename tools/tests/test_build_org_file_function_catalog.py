import hashlib
import json
import subprocess
import tempfile
import unittest
from pathlib import Path

from tools.build_org_file_function_catalog import build_catalog


def git(root: Path, *args: str) -> str:
    return subprocess.run(["git", "-C", str(root), *args], check=True,
                          capture_output=True, text=True).stdout.strip()


class FileFunctionCatalogTests(unittest.TestCase):
    def test_catalogs_every_tracked_file_with_provenance_and_confidence(self):
        with tempfile.TemporaryDirectory() as tmp:
            workspace = Path(tmp)
            root = workspace / "sample"
            root.mkdir()
            git(root, "init", "-q")
            git(root, "config", "user.email", "test@example.invalid")
            git(root, "config", "user.name", "Catalog Test")
            (root / "src").mkdir()
            (root / "src" / "lib.py").write_text('"""Parse a signed manifest."""\n\ndef parse_manifest(data):\n    return data\n', encoding="utf-8")
            (root / "README.md").write_text("# Sample module\n\nLoads and validates sample manifests.\n", encoding="utf-8")
            (root / "config.json").write_text('{"enabled": true}\n', encoding="utf-8")
            git(root, "add", ".")
            git(root, "commit", "-qm", "fixture")
            sha = git(root, "rev-parse", "HEAD")
            manifest = {"owner": "example", "repositories": [{"name": "sample", "role": "fixture"}]}
            census = {"repositories": [{"repository": "example/sample", "source_sha": sha,
                                       "report": {"files": [{"path": "src/lib.py",
                                                             "candidate_public_symbols": [{"kind": "function", "name": "parse_manifest"}]}]}}]}
            result = build_catalog(workspace, manifest, census)
            self.assertEqual(result["status"], "RESIDUAL")
            self.assertEqual(result["summary"]["tracked_files_cataloged"], 3)
            files = {f["path"]: f for f in result["repositories"][0]["files"]}
            self.assertEqual(set(files), {"README.md", "config.json", "src/lib.py"})
            self.assertEqual(files["src/lib.py"]["git_blob_sha"], git(root, "rev-parse", "HEAD:src/lib.py"))
            self.assertEqual(files["src/lib.py"]["file_sha256"], hashlib.sha256((root / "src/lib.py").read_bytes()).hexdigest())
            self.assertEqual(files["src/lib.py"]["candidate_public_symbols"][0]["name"], "parse_manifest")
            self.assertEqual(files["README.md"]["purpose_confidence"], "HIGH")
            self.assertEqual(files["config.json"]["status"], "DISCOVERY_ONLY")
            self.assertEqual(result["functional_verification"], "NOT_PERFORMED")

    def test_private_exclusion_blocks_global_completeness(self):
        with tempfile.TemporaryDirectory() as tmp:
            result = build_catalog(Path(tmp), {"owner": "example", "repositories": [{"name": "private-demo", "private": True}]}, {"repositories": []})
            self.assertEqual(result["status"], "BLOCKED")
            self.assertEqual(result["excluded_private_repositories"], ["example/private-demo"])

    def test_missing_public_checkout_blocks_catalog(self):
        with tempfile.TemporaryDirectory() as tmp:
            result = build_catalog(Path(tmp), {"owner": "example", "repositories": [{"name": "missing"}]}, {"repositories": []})
            self.assertEqual(result["status"], "BLOCKED")
            self.assertEqual(result["blocked_repositories"], ["example/missing"])


if __name__ == "__main__":
    unittest.main()
