#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
APP_NAME="OpenSquirrel"
PROFILE="${OPEN_SQUIRREL_PROFILE:-release}"

if [[ "${PROFILE}" == "release" ]]; then
  cargo build --release --manifest-path "${ROOT_DIR}/Cargo.toml"
  BINARY_PATH="${ROOT_DIR}/target/release/opensquirrel"
else
  cargo build --manifest-path "${ROOT_DIR}/Cargo.toml"
  BINARY_PATH="${ROOT_DIR}/target/debug/opensquirrel"
fi

APP_DIR="${ROOT_DIR}/dist/${APP_NAME}.app"
CONTENTS_DIR="${APP_DIR}/Contents"
MACOS_DIR="${CONTENTS_DIR}/MacOS"
RESOURCES_DIR="${CONTENTS_DIR}/Resources"
ICON_PATH="${ROOT_DIR}/assets/OpenSquirrel.icns"

mkdir -p "${MACOS_DIR}" "${RESOURCES_DIR}"

# Copy the real binary alongside the wrapper
cp "${BINARY_PATH}" "${MACOS_DIR}/${APP_NAME}-bin"
cp "${ICON_PATH}" "${RESOURCES_DIR}/${APP_NAME}.icns"
chmod +x "${MACOS_DIR}/${APP_NAME}-bin"

# Create a wrapper script that inherits the user's shell PATH
# macOS .app bundles get a minimal environment, so tools like claude/npx/uvx aren't found
cat > "${MACOS_DIR}/${APP_NAME}" <<'WRAPPER'
#!/usr/bin/env bash
# Source the user's shell profile to get PATH (homebrew, npm, cargo, etc)
if [ -f "$HOME/.zprofile" ]; then source "$HOME/.zprofile" 2>/dev/null; fi
if [ -f "$HOME/.zshrc" ]; then source "$HOME/.zshrc" 2>/dev/null; fi
if [ -f "$HOME/.bash_profile" ]; then source "$HOME/.bash_profile" 2>/dev/null; fi
if [ -f "$HOME/.profile" ]; then source "$HOME/.profile" 2>/dev/null; fi
# Ensure common paths are included
export PATH="/opt/homebrew/bin:/usr/local/bin:$HOME/.cargo/bin:$HOME/.local/bin:$PATH"
DIR="$(cd "$(dirname "$0")" && pwd)"
exec "$DIR/OpenSquirrel-bin" "$@"
WRAPPER
chmod +x "${MACOS_DIR}/${APP_NAME}"

cat > "${CONTENTS_DIR}/Info.plist" <<'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key>
  <string>OpenSquirrel</string>
  <key>CFBundleDisplayName</key>
  <string>OpenSquirrel</string>
  <key>CFBundleExecutable</key>
  <string>OpenSquirrel</string>
  <key>CFBundleIdentifier</key>
  <string>com.opensquirrel.app</string>
  <key>CFBundleVersion</key>
  <string>1</string>
  <key>CFBundleShortVersionString</key>
  <string>0.1.0</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleIconFile</key>
  <string>OpenSquirrel.icns</string>
  <key>LSMinimumSystemVersion</key>
  <string>13.0</string>
  <key>NSHighResolutionCapable</key>
  <true/>
  <key>NSMicrophoneUsageDescription</key>
  <string>OpenSquirrel uses the microphone for voice-to-text input.</string>
</dict>
</plist>
EOF

touch "${APP_DIR}"
pkill -x opensquirrel >/dev/null 2>&1 || true
pkill -x OpenSquirrel >/dev/null 2>&1 || true
open "${APP_DIR}"
