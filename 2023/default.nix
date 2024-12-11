{ pkgs ? import <nixpkgs> { } }:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "adventofcode";
  version = "0.1.0";
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;
}
