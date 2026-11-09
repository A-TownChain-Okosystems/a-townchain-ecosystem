# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# backend/api/routes/marketplace_routes.py
# Issue #13 — ATC Marketplace Routes
# Stand: Sprint 2.5 | Angepasst an MarketplaceContract v2

import logging

from blockchain.smart_contracts import atc_token, marketplace
from flask import Blueprint, jsonify, request

logger = logging.getLogger(__name__)

marketplace_bp = Blueprint("marketplace", __name__)

# Fehlercodes statt Exception-Text: verhindert Info-Leaks (py/stack-trace-exposure).
_ERROR_CODES = {
    PermissionError: "permission_denied",
    KeyError: "invalid_reference",
    ValueError: "invalid_argument",
}


def _error_response(exc, route: str):
    """Protokolliert die Ursache serverseitig und antwortet ohne interne Details."""
    logger.warning("%s rejected: %s", route, type(exc).__name__, exc_info=True)
    code = "invalid_request"
    for exc_type, mapped in _ERROR_CODES.items():
        if isinstance(exc, exc_type):
            code = mapped
            break
    return jsonify({"error": code}), 400


@marketplace_bp.route("/health", methods=["GET"])
def health():
    return jsonify({"service": "marketplace", "status": "online"})


@marketplace_bp.route("/listings", methods=["GET"])
def listings():
    return jsonify(
        {
            "listings": marketplace.get_listings(
                rarity=request.args.get("rarity"),
                element=request.args.get("element"),
                min_price=float(request.args.get("min_price", 0) or 0) or None,
                max_price=float(request.args.get("max_price", 0) or 0) or None,
                sort_by=request.args.get("sort_by", "price_asc"),
                limit=int(request.args.get("limit", 50)),
            )
        }
    )


@marketplace_bp.route("/listings/<listing_id>", methods=["GET"])
def get_listing(listing_id):
    return jsonify(marketplace.get_listing(listing_id))


@marketplace_bp.route("/token/<token_id>", methods=["GET"])
def token_listing(token_id):
    return jsonify(marketplace.get_token_listing(token_id))


@marketplace_bp.route("/list", methods=["POST"])
def list_nft():
    d = request.json or {}
    try:
        return jsonify(
            marketplace.list_for_sale(
                seller=d.get("seller", ""),
                token_id=d.get("token_id", ""),
                price_atc=float(d.get("price_atc", 0)),
            )
        )
    except (ValueError, PermissionError) as e:
        return _error_response(e, "list_nft")


@marketplace_bp.route("/buy", methods=["POST"])
def buy():
    d = request.json or {}
    marketplace.set_balance_oracle(atc_token._balances)
    try:
        return jsonify(
            marketplace.buy(buyer=d.get("buyer", ""), listing_id=d.get("listing_id", ""))
        )
    except (ValueError, PermissionError, KeyError) as e:
        return _error_response(e, "buy")


@marketplace_bp.route("/cancel", methods=["POST"])
def cancel():
    d = request.json or {}
    try:
        return jsonify(
            marketplace.cancel_listing(
                seller=d.get("seller", ""), listing_id=d.get("listing_id", "")
            )
        )
    except (ValueError, PermissionError, KeyError) as e:
        return _error_response(e, "cancel")


@marketplace_bp.route("/sales", methods=["GET"])
def sales_history():
    seller = request.args.get("seller")
    buyer = request.args.get("buyer")
    return jsonify(
        {
            "sales": marketplace.get_sales_history(
                seller=seller, buyer=buyer, limit=int(request.args.get("limit", 20))
            )
        }
    )


@marketplace_bp.route("/stats", methods=["GET"])
def stats():
    return jsonify(marketplace.get_stats())
