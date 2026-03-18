#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT_DIR}"

VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n1)"
COMMIT_SHA="$(git rev-parse --short HEAD)"
COMMIT_FULL="$(git rev-parse HEAD)"
BUILD_DATE="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"

mkdir -p dist/linux
cargo build --release
cp target/release/opensquirrel dist/linux/opensquirrel
cat > dist/linux/build-info.txt <<EOF
OpenSquirrel
Version: ${VERSION}
Commit: ${COMMIT_SHA}
Commit-Full: ${COMMIT_FULL}
Built-At: ${BUILD_DATE}
EOF
