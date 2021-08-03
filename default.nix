{ pkgs ? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "kakship";
  version = "0.2.8";
  src = ./.;
  cargoSha256 = "sha256-2kOCoAgi5Yx3i4A18pZQk8fvtgq6tmluZnvw5mc4+R0=";
  buildInputs = [ pkgs.starship ];
}
