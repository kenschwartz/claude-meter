#!/bin/sh
set -eu

APP_NAME="ZaiMeter"
LABEL="com.klivak.zaimeter"
PLIST_DIR="$HOME/Library/LaunchAgents"
PLIST_PATH="$PLIST_DIR/$LABEL.plist"

if [ $# -gt 0 ]; then
  BIN_PATH="$1"
elif [ -x "/Applications/ZaiMeter.app/Contents/MacOS/ZaiMeter" ]; then
  # Launch the bundle's executable directly. Going through /usr/bin/open
  # would exit immediately and, with KeepAlive, relaunch in a tight loop.
  BIN_PATH="/Applications/ZaiMeter.app/Contents/MacOS/ZaiMeter"
else
  BIN_PATH="$(cd "$(dirname "$0")/.." && pwd)/target/aarch64-apple-darwin/release/zaimeter"
fi

if [ ! -x "$BIN_PATH" ]; then
  echo "$APP_NAME binary is not executable: $BIN_PATH" >&2
  echo "Build it first: cargo build --release --target aarch64-apple-darwin" >&2
  exit 1
fi

mkdir -p "$PLIST_DIR"

cat > "$PLIST_PATH" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>$LABEL</string>
  <key>ProgramArguments</key>
  <array>
    <string>$BIN_PATH</string>
  </array>
  <key>RunAtLoad</key>
  <true/>
  <key>KeepAlive</key>
  <false/>
  <key>StandardOutPath</key>
  <string>$HOME/Library/Logs/zaimeter.out.log</string>
  <key>StandardErrorPath</key>
  <string>$HOME/Library/Logs/zaimeter.err.log</string>
</dict>
</plist>
EOF

launchctl unload "$PLIST_PATH" >/dev/null 2>&1 || true
launchctl load "$PLIST_PATH"

echo "Installed and started $LABEL"
