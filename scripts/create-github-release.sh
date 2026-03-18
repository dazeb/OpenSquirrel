#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT_DIR}"

REPO="${1:-dazeb/OpenSquirrel}"
VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n1)"
TAG="${2:-v${VERSION}-linux-preview}"
TITLE="${3:-OpenSquirrel ${VERSION} Linux Preview}"
NOTES_FILE="${ROOT_DIR}/dist/release-notes.md"
APPIMAGE_PATH="${ROOT_DIR}/dist/OpenSquirrel-x86_64.AppImage"
DEB_PATH="${ROOT_DIR}/dist/opensquirrel_${VERSION}_$(dpkg --print-architecture).deb"

if [[ ! -f "${APPIMAGE_PATH}" || ! -f "${DEB_PATH}" ]]; then
  echo "Missing release assets. Build the AppImage and .deb first." >&2
  exit 1
fi

cat > "${NOTES_FILE}" <<EOF
Linux preview release for OpenSquirrel.

Included assets:
- $(basename "${APPIMAGE_PATH}")
- $(basename "${DEB_PATH}")

Highlights:
- Native Linux build path
- AppImage packaging
- Debian package packaging
- WSL-aware launcher behavior without changing normal Linux behavior
EOF

git tag -f "${TAG}"
git push -f dazeb-fork "${TAG}"
gh.exe release create "${TAG}" \
  --repo "${REPO}" \
  --title "${TITLE}" \
  --notes-file "${NOTES_FILE}" \
  "${APPIMAGE_PATH}" \
  "${DEB_PATH}"
