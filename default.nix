{ pkgs ? import <nixpkgs> { }}: 
pkgs.rustPlatform.buildRustPackage rec {
  pname = "nixism";
  version = "0.0.1";
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;
}
