#!/usr/bin/env bash
#
# txodds CLI installer
# Usage: curl -sfL https://github.com/swingkiddo/txline-cli/releases/latest/download/install.sh | sh
#
set -euo pipefail

REPO="${TXODDS_INSTALL_REPO:-swingkiddo/txline-cli}"
INSTALL_DIR="${TXODDS_INSTALL_DIR:-$HOME/.txodds/bin}"
BINARY_NAME="txodds"

info() { printf '\033[1;34m==>\033[0m %s\n' "$*" >&2; }
err()  { printf '\033[1;31merror:\033[0m %s\n' "$*" >&2; }

detect_target() {
  local os arch
  case "$(uname -s)" in
    Linux)  os="unknown-linux" ;;
    Darwin) os="apple-darwin" ;;
    *) err "Unsupported OS: $(uname -s)"; return 1 ;;
  esac
  case "$(uname -m)" in
    x86_64|amd64)  arch="x86_64" ;;
    aarch64|arm64) arch="aarch64" ;;
    *) err "Unsupported architecture: $(uname -m)"; return 1 ;;
  esac
  echo "${arch}-${os}"
}

fetch_latest_tag() {
  local url="https://api.github.com/repos/${REPO}/releases/latest"
  if command -v curl >/dev/null 2>&1; then
    curl -sSL "$url" | grep -m1 '"tag_name"' | sed -E 's/.*"v?([^"]+)".*/\1/'
  elif command -v wget >/dev/null 2>&1; then
    wget -qO- "$url" | grep -m1 '"tag_name"' | sed -E 's/.*"v?([^"]+)".*/\1/'
  else
    err "Need curl or wget installed"; return 1
  fi
}

download_archive() {
  local tag="$1" target="$2" archive
  case "$target" in
    *-windows-msvc) archive="txodds-${target}.zip" ;;
    *)              archive="txodds-${target}.tar.xz" ;;
  esac
  local url="https://github.com/${REPO}/releases/download/v${tag}/${archive}"
  info "Downloading ${url}"
  if command -v curl >/dev/null 2>&1; then
    curl -fSL -o "/tmp/${archive}" "$url"
  else
    wget -q -O "/tmp/${archive}" "$url"
  fi
  echo "/tmp/${archive}"
}

extract_archive() {
  local archive="$1"
  case "$archive" in
    *.zip)  unzip -o -q "$archive" -d /tmp/txodds-extract ;;
    *.tar.xz) mkdir -p /tmp/txodds-extract && tar -xJf "$archive" -C /tmp/txodds-extract ;;
    *) err "Unknown archive format: $archive"; return 1 ;;
  esac
}

main() {
  info "Installing txodds CLI"
  local target
  target="$(detect_target)"
  info "Detected target: ${target}"

  local tag
  tag="$(fetch_latest_tag)"
  if [[ -z "$tag" ]]; then
    err "Could not determine latest release tag"; return 1
  fi
  info "Latest release: v${tag}"

  mkdir -p "$INSTALL_DIR"

  local archive
  archive="$(download_archive "$tag" "$target")"
  extract_archive "$archive"

  local src
  if [[ "$target" == *windows* ]]; then src="/tmp/txodds-extract/${BINARY_NAME}.exe"
  else                                   src="/tmp/txodds-extract/${BINARY_NAME}"
  fi
  if [[ ! -f "$src" ]]; then
    err "Binary not found after extraction: $src"; return 1
  fi

  install -m 0755 "$src" "${INSTALL_DIR}/${BINARY_NAME}"
  info "Installed to ${INSTALL_DIR}/${BINARY_NAME}"

  case ":$PATH:" in
    *":${INSTALL_DIR}:"*) ;;
    *) info "Add to PATH: export PATH=\"${INSTALL_DIR}:\$PATH\"" ;;
  esac

  info "Verifying install..."
  "${INSTALL_DIR}/${BINARY_NAME}" --version
}

main "$@"
