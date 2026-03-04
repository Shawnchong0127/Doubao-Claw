#!/usr/bin/env bash
set -euo pipefail

# ── Config ─────────────────────────────────────────────────────────────────
BINARY="dbclaw"
REPO="yourusername/doubao-claw"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

# ── Helpers ────────────────────────────────────────────────────────────────
bold()  { printf '\033[1m%s\033[0m\n' "$*"; }
green() { printf '\033[32m%s\033[0m\n' "$*"; }
red()   { printf '\033[31m%s\033[0m\n' "$*" >&2; }
die()   { red "Error: $*"; exit 1; }

# ── Detect platform ────────────────────────────────────────────────────────
OS=$(uname -s)
ARCH=$(uname -m)

[[ "$OS" == "Darwin" ]] || die "This installer only supports macOS. For Linux/Windows, build from source."

case "$ARCH" in
  arm64)  TARGET="aarch64-apple-darwin"  ;;
  x86_64) TARGET="x86_64-apple-darwin"   ;;
  *)      die "Unsupported architecture: $ARCH" ;;
esac

bold "🐾 Doubao Claw Installer"
echo   "   Platform : macOS / $ARCH"
echo   "   Target   : $TARGET"
echo   "   Install  : $INSTALL_DIR/$BINARY"
echo

# ── Get latest release ─────────────────────────────────────────────────────
bold "→ Fetching latest release..."
RELEASE_URL="https://api.github.com/repos/${REPO}/releases/latest"
VERSION=$(curl -fsSL "$RELEASE_URL" | grep '"tag_name"' | sed 's/.*"tag_name": "\(.*\)".*/\1/')

[[ -n "$VERSION" ]] || die "Could not determine latest version"
echo "   Version  : $VERSION"

# ── Download ───────────────────────────────────────────────────────────────
ARCHIVE="${BINARY}-${VERSION}-${TARGET}.tar.gz"
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${ARCHIVE}"

bold "→ Downloading $ARCHIVE..."
curl -fsSL "$DOWNLOAD_URL" -o "$TMP/$ARCHIVE" || \
  die "Download failed. Check that $DOWNLOAD_URL exists."

# ── Extract ────────────────────────────────────────────────────────────────
bold "→ Extracting..."
tar -xzf "$TMP/$ARCHIVE" -C "$TMP"

BIN_PATH="$TMP/$BINARY"
[[ -f "$BIN_PATH" ]] || BIN_PATH=$(find "$TMP" -name "$BINARY" -type f | head -1)
[[ -f "$BIN_PATH" ]] || die "Binary not found in archive"

chmod +x "$BIN_PATH"

# Strip Gatekeeper quarantine on macOS
xattr -d com.apple.quarantine "$BIN_PATH" 2>/dev/null || true

# ── Install ────────────────────────────────────────────────────────────────
bold "→ Installing to $INSTALL_DIR/$BINARY..."

if [[ -w "$INSTALL_DIR" ]]; then
  cp "$BIN_PATH" "$INSTALL_DIR/$BINARY"
else
  sudo cp "$BIN_PATH" "$INSTALL_DIR/$BINARY"
fi

# ── Verify ─────────────────────────────────────────────────────────────────
if command -v "$BINARY" &>/dev/null; then
  INSTALLED_VERSION=$("$BINARY" --version 2>&1 || echo "unknown")
  green "✓ $BINARY installed successfully! ($INSTALLED_VERSION)"
else
  green "✓ $BINARY installed to $INSTALL_DIR"
  echo  "  Make sure $INSTALL_DIR is in your PATH:"
  echo  "  export PATH=\"$INSTALL_DIR:\$PATH\""
fi

echo
bold "Next steps:"
echo "  1. Get an API key at https://console.volcengine.com/"
echo "  2. dbclaw config set api_key <your-key>"
echo "  3. dbclaw chat"
echo
green "Happy chatting! 🐾"
