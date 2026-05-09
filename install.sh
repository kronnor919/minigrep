#!/bin/bash
set -e

REPO="kronnor919/minigrep"
BINARY_NAME="minigrep"
INSTALL_DIR="/usr/local/bin"
ARCH="x86_64"
OS="linux"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

info() {
  echo -e "${GREEN}[INFO]${NC} $1"
}

error() {
  echo -e "${RED}[ERROR]${NC} $1"
  exit 1
}

warn() {
  echo -e "${YELLOW}[WARN]${NC} $1"
}

check_system() {
  info "Checking system requirements..."

  ARCH_SYSTEM=$(uname -m)
  if [[ "$ARCH_SYSTEM" != "x86_64" ]]; then
    error "Architecture $ARCH_SYSTEM not supported. Only x86_64 is available."
  fi

  OS_SYSTEM=$(uname -s | tr '[:upper:]' '[:lower:]')
  if [[ "$OS_SYSTEM" != "linux" ]]; then
    error "Operating system $OS_SYSTEM not supported. Only Linux is available."
  fi

  info "System OK: $OS_SYSTEM/$ARCH_SYSTEM"
}

check_dependencies() {
  info "Checking dependencies..."

  if ! command -v curl &>/dev/null; then
    error "curl is required but not installed. Please install it first."
  fi

  if ! command -v tar &>/dev/null; then
    error "tar is required but not installed. Please install it first."
  fi

  info "All dependencies are satisfied"
}

get_download_url() {
  info "Fetching latest version from GitHub..."

  DOWNLOAD_URL=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" |
    grep -o '"browser_download_url": "[^"]*'"$BINARY_NAME"'-[^"]*'"$ARCH"'-'"$OS"'[^"]*\.tar\.gz"' |
    head -1 |
    cut -d '"' -f 4)

  if [ -z "$DOWNLOAD_URL" ]; then
    error "Could not find binary for $ARCH-$OS in latest release"
  fi

  FILENAME=$(basename "$DOWNLOAD_URL")
  info "Found: $FILENAME"
}

download_binary() {
  info "Downloading from GitHub..."

  TMP_DIR=$(mktemp -d)
  cd "$TMP_DIR"

  curl -L -o "$FILENAME" "$DOWNLOAD_URL" || error "Failed to download file"

  info "Download complete"
}

extract_and_install() {
  info "Extracting files..."

  tar -xzf "$FILENAME" || error "Failed to extract archive"

  if [ ! -f "$BINARY_NAME" ]; then
    error "Binary $BINARY_NAME not found in downloaded archive"
  fi

  chmod +x "$BINARY_NAME"

  info "Installing to $INSTALL_DIR..."

  if [ ! -w "$INSTALL_DIR" ]; then
    warn "No write permission to $INSTALL_DIR"
    error "Please run with sudo: sudo ./install.sh"
  fi

  cp "$BINARY_NAME" "$INSTALL_DIR/"

  info "$BINARY_NAME successfully installed to $INSTALL_DIR!"
}

verify_installation() {
  info "Verifying installation..."

  if command -v "$BINARY_NAME" &>/dev/null; then
    VERSION=$($BINARY_NAME --version 2>/dev/null || echo "unknown")
    info "✅ $BINARY_NAME is installed and available (version: $VERSION)"
  else
    warn "Could not verify installation. Make sure $INSTALL_DIR is in your PATH"
  fi
}

cleanup() {
  if [ -n "$TMP_DIR" ] && [ -d "$TMP_DIR" ]; then
    rm -rf "$TMP_DIR"
    info "Cleaned up temporary files"
  fi
}

main() {
  info "=== minigrep Installer ==="
  echo ""

  check_system
  check_dependencies
  get_download_url
  download_binary
  extract_and_install
  verify_installation

  echo ""
  info "Installation complete! Try running: $BINARY_NAME --help"
}

trap cleanup EXIT
main
