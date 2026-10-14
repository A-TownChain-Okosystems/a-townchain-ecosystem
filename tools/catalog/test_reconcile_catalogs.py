#!/usr/bin/env python3
"""Standard-library tests for reconcile_catalogs.py."""
import importlib.util
import json
import tempfile
import unittest
from pathlib import Path

SCRIPT = Path(__file__).with_name("reconcile_catalogs.py")
SPEC = importlib.util.spec_from_file_location("reconcile_catalogs", SCRIPT)
MODULE = importlib.util.module_from_spec(SPEC)
assert SPEC and SPEC.loader
SPEC.loader.exec_module(MODULE)


class CatalogReconcilerTests(unittest.TestCase):
    def test_collects_known_collections_and_preserves_record(self):
        doc = {
            "catalog_id": "example",
            "functions": [{"id": "f-1", "name": "same"}],
            "nested": {"interfaces": [{"id": "i-1", "name": "same"}]},
            "unrelated": [{"name": "not-a-catalog-record"}],
        }
        records = MODULE.collect_records(doc)
        self.assertEqual([r["record_kind"] for r in records], ["function", "interface"])
        self.assertEqual(records[0]["source_record"], {"id": "f-1", "name": "same"})
        self.assertEqual(records[0]["json_pointer"], "$.functions[0]")

    def test_hash_and_namespaced_id_are_deterministic(self):
        item = {"id": "function/one", "name": "run"}
        self.assertEqual(MODULE.digest(item), MODULE.digest({"name": "run", "id": "function/one"}))
        self.assertEqual(MODULE.safe_namespace("function/one"), "function-one")

    def test_name_is_not_an_identity_key(self):
        a = {"id": "a", "name": "execute"}
        b = {"id": "b", "name": "execute"}
        self.assertNotEqual(MODULE.digest(a), MODULE.digest(b))
        self.assertEqual(MODULE.display_name(a), MODULE.display_name(b))

    def test_markdown_table_rows_are_preserved_as_catalog_entries(self):
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / "catalog.md"
            path.write_text(
                "# Catalog\n\n| Component / path | State |\n|---|---|\n| system/core | IMPLEMENTED |\n",
                encoding="utf-8",
            )
            records = MODULE.collect_markdown_tables(path)
            self.assertEqual(len(records), 1)
            self.assertEqual(records[0]["record_kind"], "catalog_entry")
            self.assertEqual(records[0]["source_record"]["component_path"], "system/core")
            self.assertEqual(records[0]["source_record"]["state"], "IMPLEMENTED")

    def test_invalid_json_raises_for_caller_to_fail_closed(self):
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / "bad.json"
            path.write_text("{", encoding="utf-8")
            with self.assertRaises(json.JSONDecodeError):
                MODULE.load_json(path)


if __name__ == "__main__":
    unittest.main()
