#!/bin/sh
set -eu

tape="$PWD"
cache="$HOME/.cache/tapes-deck"
mkdir -p "$cache"

jobs() {
  mem=$(awk '/MemAvailable/ { print int($2 / 2097152) }' /proc/meminfo)
  cores=$(nproc)
  [ "$mem" -lt 1 ] && mem=1
  [ "$mem" -gt "$cores" ] && mem=$cores
  echo "$mem"
}

need() {
  missing=""
  for tool in "$@"; do
    command -v "$tool" >/dev/null 2>&1 || missing="$missing $tool"
  done
  if [ -n "$missing" ]; then
    echo "==> Can't build without:$missing. Install them and install this tape again."
    exit 1
  fi
}

rm -rf install.sh design log*.txt

if [ -s /usr/lib/libm3shapes.so ] && [ -s /usr/lib/qt6/qml/M3Shapes/libm3shapesplugin.so ]; then
  echo "==> M3Shapes is already installed"
else
  need git cmake ninja
  echo "==> Building M3Shapes, a drawing library the shell needs. This compiles C++ and takes a few minutes."
  src="$cache/m3shapes"
  rm -rf "$src"
  git clone --depth 1 https://github.com/soramanew/m3shapes.git "$src"
  cmake -S "$src" -B "$src/build" -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/usr -DCMAKE_INSTALL_LIBDIR=lib
  cmake --build "$src/build" --parallel "$(jobs)"
  echo "==> Installing M3Shapes. A password window will open."
  pkexec cmake --install "$src/build"
  sync
fi
