{
  description = "Tapes-Deck - The Official GUI for Tapes";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    tapes = {
      url = "github:futurekismo-oss/tapes";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    tapes,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
      tapes-deck-src = pkgs.callPackage ./packaging/nix/package.nix {
        tapes = tapes.packages.${system}.default;
      };
      tapes-deck-bin = pkgs.callPackage ./packaging/nix/bin.nix {
        tapes = tapes.packages.${system}.default;
      };
      useBin =
        system == "x86_64-linux"
        && tapes-deck-bin.version == tapes-deck-src.version
        && pkgs.lib.versionAtLeast pkgs.glibc.version "2.43";
    in {
      packages =
        {
          default =
            if useBin
            then tapes-deck-bin
            else tapes-deck-src;
          tapes-deck = tapes-deck-src;
          inherit tapes-deck-src;
        }
        // pkgs.lib.optionalAttrs (system == "x86_64-linux") {
          inherit tapes-deck-bin;
        };
    })
    // {
      overlays = {
        default = final: prev: {
          tapes-deck = self.packages.${prev.stdenv.hostPlatform.system}.default;
        };
        tapes-deck = self.overlays.default;
      };
    };
}
