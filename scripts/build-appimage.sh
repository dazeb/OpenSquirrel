#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT_DIR}"

APP_NAME="OpenSquirrel"
APP_ID="io.github.Infatoshi.OpenSquirrel"
ARCH="${ARCH:-x86_64}"
DIST_DIR="${ROOT_DIR}/dist"
LINUX_DIST_DIR="${DIST_DIR}/linux"
APPDIR="${LINUX_DIST_DIR}/AppDir"
APPIMAGE_TOOL="${LINUX_DIST_DIR}/appimagetool-${ARCH}.AppImage"
APPIMAGE_OUT="${DIST_DIR}/${APP_NAME}-${ARCH}.AppImage"
APPIMAGE_URL="https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-${ARCH}.AppImage"

mkdir -p "${LINUX_DIST_DIR}"
"${ROOT_DIR}/scripts/build-linux-release.sh"

rm -rf "${APPDIR}"
mkdir -p "${APPDIR}/usr/bin"
mkdir -p "${APPDIR}/usr/share/applications"
mkdir -p "${APPDIR}/usr/share/icons/hicolor/256x256/apps"
mkdir -p "${APPDIR}/usr/share/metainfo"

cp "${ROOT_DIR}/dist/linux/opensquirrel" "${APPDIR}/usr/bin/opensquirrel"
cp "${ROOT_DIR}/linux/AppRun" "${APPDIR}/AppRun"
cp "${ROOT_DIR}/linux/opensquirrel.desktop" "${APPDIR}/${APP_ID}.desktop"
cp "${ROOT_DIR}/linux/opensquirrel.desktop" "${APPDIR}/usr/share/applications/${APP_ID}.desktop"
cp "${ROOT_DIR}/linux/opensquirrel.appdata.xml" "${APPDIR}/usr/share/metainfo/${APP_ID}.appdata.xml"
cp "${ROOT_DIR}/assets/logo.png" "${APPDIR}/opensquirrel.png"
cp "${ROOT_DIR}/assets/logo.png" "${APPDIR}/usr/share/icons/hicolor/256x256/apps/${APP_ID}.png"

chmod +x "${APPDIR}/AppRun"
chmod +x "${APPDIR}/usr/bin/opensquirrel"

if [[ ! -f "${APPIMAGE_TOOL}" ]]; then
  curl -L "${APPIMAGE_URL}" -o "${APPIMAGE_TOOL}"
  chmod +x "${APPIMAGE_TOOL}"
fi

ARCH="${ARCH}" "${APPIMAGE_TOOL}" --appimage-extract-and-run "${APPDIR}" "${APPIMAGE_OUT}"
chmod +x "${APPIMAGE_OUT}"

echo "AppImage created at ${APPIMAGE_OUT}"
