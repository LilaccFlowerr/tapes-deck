#!/bin/sh
# Extra setup for this tape, run by tapes-deck inside the tape's folder right after it is installed.
set -eu

cache="$HOME/.cache/tapes-deck"
mkdir -p "$cache"

# Installs packages (official repos and AUR) with whichever AUR helper is there. Their root steps go
# through pkexec (a password popup) instead of sudo, which can't prompt without a terminal.
install_packages() {
  for helper in yay paru; do
    if command -v "$helper" >/dev/null 2>&1; then
      "$helper" --sudo pkexec -S --needed --noconfirm "$@"
      return
    fi
  done
  echo "No AUR helper (yay or paru) found. Install these yourself: $*"
  return 1
}

# Nebula's own installer, notes and design files are not configs.
rm -rf install.sh design log*.txt

echo "Installing the Nebula packages"
install_packages \
  hyprland hypridle hyprlock quickshell \
  matugen xdg-desktop-portal-hyprland ghostty firefox \
  nautilus fuzzel cliphist wl-clipboard \
  grim slurp mate-polkit playerctl \
  brightnessctl cava zenity libnotify \
  ffmpeg jq pciutils fastfetch \
  gpu-screen-recorder inter-font ttf-material-symbols-variable ttf-nerd-fonts-symbols \
  ttf-jetbrains-mono-nerd noto-fonts-emoji papirus-icon-theme git \
  cmake ninja gcc qt6-shadertools

# M3Shapes is built from source, like Nebula's own installer does. Installing it needs root too.
echo "Building M3Shapes"
src="$cache/m3shapes"
rm -rf "$src"
git clone --depth 1 https://github.com/soramanew/m3shapes.git "$src"
cmake -S "$src" -B "$src/build" -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/usr -DCMAKE_INSTALL_LIBDIR=lib
cmake --build "$src/build" --parallel
pkexec cmake --install "$src/build"
