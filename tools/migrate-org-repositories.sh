#!/usr/bin/env bash
set -euo pipefail

ORG="A-TownChain-Okosystems"
ROOT="${1:-components}"
REPOS=(
  atc-standards
  atclang
  a-townchain
  a-townchain-os
  a-townchain-os-docs
  atc-shivacore
  atc-contracts
  atc-node
  atc-compute
  atc-oracle
  atc-sdk
  atc-marketplace
  atc-wallet
  atc-vm
  atc-zkp
  aurora-ai
  globus-os
  genesis-engine
  genesis-chronicles
  atc-mining
  atc-indexer
  atc-interop
  atc-launchpad
  atc-explorer
  atc-storage
  atc-algorithm
)

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

mkdir -p "$ROOT"
for repo in "${REPOS[@]}"; do
  echo "==> Syncing $ORG/$repo"
  rm -rf "$tmp/$repo"
  git clone --depth 1 --quiet "https://github.com/$ORG/$repo.git" "$tmp/$repo"
  rm -rf "$tmp/$repo/.git"
  rm -rf "$ROOT/$repo"
  mkdir -p "$ROOT/$repo"
  cp -a "$tmp/$repo/." "$ROOT/$repo/"
done

echo "Migration complete: ${#REPOS[@]} repositories copied under $ROOT/"
