#!/bin/sh
set -eu

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TARGET_DIR="$ROOT/target/aarch64-apple-darwin/release"
APP_DIR="$TARGET_DIR/ZaiMeter.app"
CONTENTS="$APP_DIR/Contents"
MACOS_DIR="$CONTENTS/MacOS"
RESOURCES="$CONTENTS/Resources"

cargo build --release --target aarch64-apple-darwin

rm -rf "$APP_DIR"
mkdir -p "$MACOS_DIR" "$RESOURCES"

cp "$TARGET_DIR/zaimeter" "$RESOURCES/zaimeter-agent"
chmod +x "$RESOURCES/zaimeter-agent"

# Menu bar severity dots: convert the .ico assets to PNGs the Swift app loads
# from its bundle. Selected at runtime by usage level (green/yellow/red/gray).
for color in green yellow red gray; do
  sips -s format png "$ROOT/assets/icon_$color.ico" \
    --out "$RESOURCES/dot_$color.png" >/dev/null 2>&1 || true
done

swiftc \
  -parse-as-library \
  -O \
  -framework AppKit \
  -framework Foundation \
  -framework UniformTypeIdentifiers \
  "$ROOT/macos/ZaiMeterApp.swift" \
  -o "$MACOS_DIR/ZaiMeter"

cat > "$CONTENTS/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key>
  <string>ZaiMeter</string>
  <key>CFBundleIdentifier</key>
  <string>com.klivak.zaimeter</string>
  <key>CFBundleName</key>
  <string>ZaiMeter</string>
  <key>CFBundleDisplayName</key>
  <string>ZaiMeter</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>5.0.0</string>
  <key>CFBundleVersion</key>
  <string>5.0.0</string>
  <key>LSMinimumSystemVersion</key>
  <string>12.0</string>
  <key>LSUIElement</key>
  <true/>
  <key>NSHighResolutionCapable</key>
  <true/>
</dict>
</plist>
EOF

if command -v codesign >/dev/null 2>&1; then
  IDENTITY="${MACOS_CODESIGN_IDENTITY:--}"
  codesign --force --deep --sign "$IDENTITY" "$APP_DIR"
fi

ditto -c -k --keepParent "$APP_DIR" "$TARGET_DIR/ZaiMeter-macos-arm64.app.zip"
cp "$TARGET_DIR/zaimeter" "$TARGET_DIR/zaimeter-macos-arm64"
