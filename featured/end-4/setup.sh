#!/bin/sh
set -eu

# end-4 sets shadow render_power to 10, but many Hyprland versions only accept
# 1 to 4 and show "value 10 is more than the maximum of 4" on every reload.
# Clamp it to 4 in the tape's own copy of the config.
echo "==> Making end-4's shadow setting fit your Hyprland"
if [ -d hypr ]; then
  grep -rlE 'render_power' hypr 2>/dev/null | while IFS= read -r file; do
    sed -i -E 's/(render_power[[:space:]]*=[[:space:]]*)([5-9]|[1-9][0-9]+)/\14/' "$file"
  done
fi

# The shell's shapes library is a git submodule upstream. The install only
# copies files, so that folder arrives empty and the shell fails to load
# ("module qs.modules.common.widgets.shapes is not installed").
shapes=quickshell/ii/modules/common/widgets/shapes
if [ -d quickshell/ii ] && [ -z "$(ls -A "$shapes" 2>/dev/null)" ]; then
  echo "==> Downloading the shapes library the shell needs"
  command -v git >/dev/null 2>&1 || { echo "==> Needs git to download the shapes library. Install git and install this tape again."; exit 1; }
  rm -rf "$shapes"
  git clone --depth 1 https://github.com/end-4/rounded-polygon-qmljs.git "$shapes"
  rm -rf "$shapes/.git"
fi
