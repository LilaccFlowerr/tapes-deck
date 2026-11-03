#!/bin/sh
# Installs the latest tapesgui release into ~/.local/bin, plus an app-launcher entry.
#   curl -fsSL https://raw.githubusercontent.com/LilaccFlowerr/tapes-gui/main/install.sh | sh
set -eu

REPO="LilaccFlowerr/tapes-gui"
BIN_DIR="${XDG_BIN_HOME:-$HOME/.local/bin}"
DATA_DIR="${XDG_DATA_HOME:-$HOME/.local/share}"

if [ "$(uname -s)" != "Linux" ] || [ "$(uname -m)" != "x86_64" ]; then
  echo "tapesgui only has Linux x86_64 builds." >&2
  exit 1
fi

fetch() { # url dest
  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$1" -o "$2"
  elif command -v wget >/dev/null 2>&1; then
    wget -qO "$2" "$1"
  else
    echo "Need curl or wget." >&2
    exit 1
  fi
}

mkdir -p "$BIN_DIR" "$DATA_DIR/applications" "$DATA_DIR/icons/hicolor/scalable/apps"

echo "Downloading tapesgui..."
tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT
fetch "https://github.com/$REPO/releases/latest/download/tapesgui-x86_64-unknown-linux-gnu" "$tmp"
install -m755 "$tmp" "$BIN_DIR/tapesgui"

raw="https://raw.githubusercontent.com/$REPO/main"
fetch "$raw/assets/add-tape.svg" "$DATA_DIR/icons/hicolor/scalable/apps/tapesgui.svg"
fetch "$raw/packaging/tapesgui.desktop" "$DATA_DIR/applications/tapesgui.desktop"

echo "Installed to $BIN_DIR/tapesgui"
case ":$PATH:" in
  *":$BIN_DIR:"*) ;;
  *) echo "Note: $BIN_DIR is not in your \$PATH. Add this to your shell config:"
     echo "  export PATH=\"$BIN_DIR:\$PATH\"" ;;
esac
command -v tapes-v2 >/dev/null 2>&1 || [ -x "$BIN_DIR/tapes-v2" ] ||
  echo "Note: tapes-v2 is not installed yet: https://github.com/futurekismo-oss/tapes-v2"
