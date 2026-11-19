{
  rustPlatform,
  lib,
  pkg-config,
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
  cargoToml = builtins.fromTOML (builtins.readFile ../../Cargo.toml);

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
  rustPlatform.buildRustPackage {
    pname = "tapes-deck";
    version = cargoToml.package.version;

    src = lib.cleanSource ../../.;

    cargoLock.lockFile = ../../Cargo.lock;

    nativeBuildInputs = [
      pkg-config
      makeWrapper
    ];

    buildInputs = runtimeLibs;

    env.TAPES_BINARY = "${tapes}/bin/tapes";

    postInstall = ''
      install -Dm644 packaging/tapes-deck.desktop "$out/share/applications/tapes-deck.desktop"
      install -Dm644 assets/icon.svg "$out/share/icons/hicolor/scalable/apps/tapes-deck.svg"
    '';

    postFixup = ''
      wrapProgram "$out/bin/tapes-deck" \
        --prefix LD_LIBRARY_PATH : "${lib.makeLibraryPath runtimeLibs}" \
        --prefix PATH : "${lib.makeBinPath [tapes]}" \
        --suffix PATH : "${lib.makeBinPath [git util-linux]}"
    '';

    meta = {
      description = "The GUI for Tapes, an atomic dotfile switcher";
      homepage = "https://github.com/LilaccFlowerr/tapes-deck";
      license = with lib.licenses; [
        gpl3Only
      ];
      mainProgram = "tapes-deck";
      platforms = lib.platforms.linux;
    };
  }
