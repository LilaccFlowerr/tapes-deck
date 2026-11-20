{
  stdenv,
  lib,
  fetchurl,
  autoPatchelfHook,
  makeWrapper,
  fontconfig,
  freetype,
  libxkbcommon,
  wayland,
  libGL,
  vulkan-loader,
  libx11,
  libxcursor,
  libxi,
  libxrandr,
  git,
  util-linux,
  tapes,
}: let
  version = "0.5.0-beta";

  runtimeLibs = [
    fontconfig
    freetype
    libxkbcommon
    wayland
    libGL
    vulkan-loader
    libx11
    libxcursor
    libxi
    libxrandr
  ];
in
  stdenv.mkDerivation {
    pname = "tapes-deck-bin";
    inherit version;

    src = fetchurl {
      url = "https://github.com/LilaccFlowerr/tapes-deck/releases/download/v${version}/tapes-deck-x86_64-unknown-linux-gnu";
      hash = "sha256-cIP5uuubgo9E0EttRsEurkUnhAo/AAWi2+8akr0W7Uk=";
    };

    nativeBuildInputs = [
      autoPatchelfHook
      makeWrapper
    ];

    buildInputs = [
      stdenv.cc.cc.lib
      fontconfig
    ];

    dontUnpack = true;
    dontBuild = true;

    installPhase = ''
      runHook preInstall
      install -Dm755 "$src" "$out/bin/tapes-deck"
      install -Dm644 ${../tapes-deck.desktop} "$out/share/applications/tapes-deck.desktop"
      install -Dm644 ${../../assets/icon.svg} "$out/share/icons/hicolor/scalable/apps/tapes-deck.svg"
      runHook postInstall
    '';

    postFixup = ''
      wrapProgram "$out/bin/tapes-deck" \
        --set-default TAPES_BIN "${tapes}/bin/tapes" \
        --prefix LD_LIBRARY_PATH : "${lib.makeLibraryPath runtimeLibs}" \
        --prefix PATH : "${lib.makeBinPath [tapes]}" \
        --suffix PATH : "${lib.makeBinPath [git util-linux]}"
    '';

    meta = {
      description = "The GUI for Tapes, an atomic dotfile switcher (prebuilt binary)";
      homepage = "https://github.com/LilaccFlowerr/tapes-deck";
      license = with lib.licenses; [
        gpl3Only
      ];
      mainProgram = "tapes-deck";
      platforms = ["x86_64-linux"];
    };
  }
