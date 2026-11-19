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
      tapes-deck = pkgs.callPackage ./packaging/nix/package.nix {
        tapes = tapes.packages.${system}.default;
      };
    in {
      packages = {
        default = tapes-deck;
        inherit tapes-deck;
      };
    })
    // {
      overlays = {
        default = final: prev: {
          tapes-deck = self.packages.${prev.stdenv.hostPlatform.system}.tapes-deck;
        };
        tapes-deck = self.overlays.default;
      };
    };
}
