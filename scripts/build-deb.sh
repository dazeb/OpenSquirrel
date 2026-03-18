#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT_DIR}"

PACKAGE_NAME="opensquirrel"
VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n1)"
ARCH="$(dpkg --print-architecture)"
DIST_DIR="${ROOT_DIR}/dist"
DEB_ROOT="${DIST_DIR}/deb-root"
INSTALL_ROOT="${DEB_ROOT}/usr"
CONTROL_DIR="${DEB_ROOT}/DEBIAN"
PACKAGE_PATH="${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_${ARCH}.deb"

"${ROOT_DIR}/scripts/build-linux-release.sh"

rm -rf "${DEB_ROOT}"
mkdir -p "${CONTROL_DIR}"
mkdir -p "${INSTALL_ROOT}/bin"
mkdir -p "${INSTALL_ROOT}/share/applications"
mkdir -p "${INSTALL_ROOT}/share/icons/hicolor/256x256/apps"
mkdir -p "${INSTALL_ROOT}/share/metainfo"
mkdir -p "${INSTALL_ROOT}/share/doc/opensquirrel"

cp "${DIST_DIR}/linux/opensquirrel" "${INSTALL_ROOT}/bin/opensquirrel"
cp "${DIST_DIR}/linux/build-info.txt" "${INSTALL_ROOT}/share/doc/opensquirrel/build-info.txt"
cp "${ROOT_DIR}/linux/opensquirrel.desktop" "${INSTALL_ROOT}/share/applications/io.github.Infatoshi.OpenSquirrel.desktop"
cp "${ROOT_DIR}/assets/logo.png" "${INSTALL_ROOT}/share/icons/hicolor/256x256/apps/opensquirrel.png"
cp "${ROOT_DIR}/linux/opensquirrel.appdata.xml" "${INSTALL_ROOT}/share/metainfo/io.github.Infatoshi.OpenSquirrel.appdata.xml"

chmod 0755 "${INSTALL_ROOT}/bin/opensquirrel"

cat > "${CONTROL_DIR}/control" <<EOF
Package: ${PACKAGE_NAME}
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Maintainer: dazeb <dazeb@users.noreply.github.com>
Depends: libc6, libasound2, libfontconfig1, libvulkan1
Description: Native control plane for AI coding agents
 OpenSquirrel is a native Rust desktop app for running AI coding agents
 side by side with persistent sessions, remote targets, and structured
 transcript rendering.
EOF

dpkg-deb --build --root-owner-group "${DEB_ROOT}" "${PACKAGE_PATH}"
echo "Debian package created at ${PACKAGE_PATH}"
