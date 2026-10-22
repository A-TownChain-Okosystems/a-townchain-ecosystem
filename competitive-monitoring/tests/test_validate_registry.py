import unittest

from validate_registry import validate_overlay, validate_registry


def entry(ident="ACME-PRODUCT-001"):
    return {
        "id": ident, "name": "Acme", "areas": ["Agents"], "products": ["Agent"],
        "urls": {"product": "https://example.com/product", "pricing": None},
        "checked": "2026-10-22", "source_status": "PARTIAL_OFFICIAL_VERIFIED",
        "unverified": ["pricing"], "snapshot": {"date": "2026-10-22", "status": "BASELINE", "prices": None},
    }


class RegistryValidatorTests(unittest.TestCase):
    def test_valid_registry_passes(self):
        self.assertEqual(validate_registry({"entries": [entry()]}), [])

    def test_duplicate_stable_id_fails(self):
        errors = validate_registry({"entries": [entry(), entry()]})
        self.assertTrue(any("duplicate stable id" in error for error in errors))

    def test_missing_source_is_not_silently_accepted(self):
        item = entry()
        item["urls"] = {"product": None, "pricing": None}
        item["unverified"] = []
        errors = validate_registry({"entries": [item]})
        self.assertTrue(any("no usable source URL" in error for error in errors))

    def test_unknown_status_fails(self):
        item = entry()
        item["snapshot"]["status"] = "GREEN"
        errors = validate_registry({"entries": [item]})
        self.assertTrue(any("unsupported value" in error for error in errors))

    def test_price_missing_currency_without_unverified_marker_fails(self):
        item = entry()
        item["snapshot"]["prices"] = [{
            "amount": 10, "billing": "monthly", "effective_date": "2026-10-22"
        }]
        item["unverified"] = []
        errors = validate_registry({"entries": [item]})
        self.assertTrue(any("currency missing/unverified" in error for error in errors))

    def test_unresolved_update_fails(self):
        errors = validate_overlay(
            {"entries": [entry()]},
            {"deltas": [{"type": "UPDATE", "id": "MISSING-PRODUCT-001"}]},
        )
        self.assertTrue(any("UPDATE target does not exist" in error for error in errors))

    def test_add_existing_id_fails(self):
        errors = validate_overlay(
            {"entries": [entry()]},
            {"deltas": [{"type": "ADD", "id": "ACME-PRODUCT-001"}]},
        )
        self.assertTrue(any("ADD target already exists" in error for error in errors))

    def test_invalid_delta_type_fails(self):
        errors = validate_overlay(
            {"entries": [entry()]},
            {"deltas": [{"type": "UPSERT", "id": "ACME-PRODUCT-001"}]},
        )
        self.assertTrue(any("type is invalid" in error for error in errors))


if __name__ == "__main__":
    unittest.main()
