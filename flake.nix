{
  inputs = {
    nixpkgs.url = github:nixos/nixpkgs/nixos-unstable;
    flake-utils.url = github:numtide/flake-utils;
    rust-overlay = {
      url = github:oxalica/rust-overlay;
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        rust = pkgs.rust-bin.stable.latest.default;
        drv = pkgs.rustPlatform.buildRustPackage {
          pname = "mpd-utils";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          nativeBuildInputs = [ rust ];
        };
      in {
        packages.default = drv;
        devShells.default = pkgs.mkShell {
          inputsFrom = [ drv ];
          nativeBuildInputs = [
            (rust.override { extensions = ["rust-src" "rust-analyzer"]; })
          ];
        };
      });
}
