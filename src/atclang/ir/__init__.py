"""ATCLang IR — normalisierte JSON-Zwischendarstellung (Tooling-Basis)."""

from .json_ir import IRValidationError, ir_hash, to_json_ir, validate_ir

__all__ = ["IRValidationError", "ir_hash", "to_json_ir", "validate_ir"]
