{
  rustPlatform,
  lib,
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "tapes-deck";
  version = "0.4.4-beta";

  src = ../../.;

  cargoHash = "sha256-K+MnK86M51toI/X3pxYDhl4JPG0yJpyASLXjIC70bVw=";

  meta = {
    description = "The GUI for Tapes, An atomic dotfile switcher";
    homepage = "https://github.com/LilaccFlowerr/tapes-deck";
    license = with lib.licenses; [
      gpl3
    ];
    maintainers = with lib.maintainers; [
      futurekismo
    ];
    mainProgram = "tapes-deck";
    platforms = [ "x86_64-linux" ];
  };
})
