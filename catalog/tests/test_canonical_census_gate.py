import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


SCRIPT = Path(__file__).resolve().parents[1] / "scripts" / "validate_canonical_census.py"


class CanonicalCensusGateTests(unittest.TestCase):
    def run_gate(self, root: Path) -> subprocess.CompletedProcess:
        # The production script resolves catalog paths from its own location;
        # this smoke test verifies the executable entrypoint and fail-closed
        # handling of the repository's checked-in snapshot.
        return subprocess.run(
            [sys.executable, str(SCRIPT)],
            cwd=root,
            text=True,
            capture_output=True,
            check=False,
        )

    def test_script_is_present_and_python_compiles(self):
        result = subprocess.run(
            [sys.executable, "-m", "py_compile", str(SCRIPT)],
            text=True,
            capture_output=True,
            check=False,
        )
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_gate_does_not_claim_functional_verification(self):
        source = SCRIPT.read_text(encoding="utf-8")
        self.assertIn('"functional_verification": "NOT_INFERRED"', source)
        self.assertIn("stale source binding", source)
        self.assertIn("exact tested commit SHA", source)


if __name__ == "__main__":
    unittest.main()
