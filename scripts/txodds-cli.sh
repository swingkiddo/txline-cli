#!/usr/bin/env bash
#
# txodds wrapper for use by the txodds opencode skill.
# Forwards all arguments to the `txodds` binary, downloading it
# via the repo's install.sh if the binary is not on PATH.
#
set -euo pipefail

REPO="${TXODDS_INSTALL_REPO:-swingkiddo/txline-cli}"
INSTALL_DIR="${TXODDS_INSTALL_DIR:-$HOME/.txodds/bin}"

ensure_txodds() {
  if command -v txodds >/dev/null 2>&1; then
    command -v txodds
    return 0
  fi
  local candidate="$INSTALL_DIR/txodds"
  if [[ -x "$candidate" ]]; then
    echo "$candidate"
    return 0
  fi
  echo "txodds not found. Installing..." >&2
  local script
  script="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/install.sh"
  if [[ ! -f "$script" ]]; then
    # Fall back to fetching the installer from the latest release
    script="$(mktemp)"
    curl -sfL -o "$script" "https://github.com/${REPO}/releases/latest/download/install.sh"
  fi
  bash "$script"
  if [[ -x "$candidate" ]]; then
    echo "$candidate"
    return 0
  fi
  echo "txodds install failed" >&2
  return 1
}

binary="$(ensure_txodds)"
exec "$binary" "$@"
