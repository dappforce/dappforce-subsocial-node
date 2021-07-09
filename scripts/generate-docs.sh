#!/usr/bin/env bash

set -e

CDIR=$(dirname "$0")

if [[ -z $1 ]]; then echo "Run this script specifying a version"; exit 1; fi
VERSION="$1"

cargo doc --frozen --release --workspace \
  --exclude pallet-donations \
  --exclude pallet-moderation \
  --exclude pallet-session-keys \
  --exclude pallet-space-multi-ownership \
  --exclude pallet-subscriptions

cp -r "$CDIR/../target/doc/" "$CDIR/../$VERSION"
