#!/bin/sh
# Extra setup for this tape, run by tapes-deck inside the tape's folder right after it is installed.
# It does what `caelestia install` does for the default components in manifest.toml, but keeps the
# configs in the tape instead of writing them into ~/.config.
set -eu

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

# Only the configs of the default components belong in the tape. The rest has its dots somewhere
# else (firefox, vscode, ...), would replace a whole folder of yours (nvim) or is optional.
rm -rf manifest.toml .luacheckrc .luarc.json firefox vscode spicetify zed zen nvim packages uwsm
# The dots call this folder thunar, the config folder is Thunar.
[ -d thunar ] && mv thunar Thunar

echo "Installing the caelestia packages"
install_packages \
  caelestia-shell caelestia-cli hyprland xdg-desktop-portal-hyprland \
  xdg-desktop-portal-gtk ttf-jetbrains-mono-nerd fish eza \
  zoxide direnv foot fastfetch \
  btop micro thunar starship \
  adw-gtk-theme papirus-icon-theme papirus-folders qtengine \
  frameworkintegration darkly-bin gnome-keyring polkit-gnome \
  networkmanager bluez bluez-utils pipewire \
  pipewire-pulse pipewire-audio pipewire-alsa pipewire-jack \
  wireplumber pwvucontrol wl-clipboard cliphist \
  noto-fonts noto-fonts-cjk noto-fonts-emoji curl \
  git trash-cli jq lazygit \
  bat ripgrep ydotool hyprpicker \
  xdg-user-dirs

xdg-user-dirs-update
