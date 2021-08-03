{ pkgs }:

let
  starship = pkgs.starship;
  kakship =
    pkgs.rustPlatform.buildRustPackage rec {
      pname = "kakship";
      version = "0.2.8";
      src = ./.;
      cargoSha256 = "sha256-2kOCoAgi5Yx3i4A18pZQk8fvtgq6tmluZnvw5mc4+R0=";
      buildInputs = [ starship ];
    };
in
pkgs.symlinkJoin {
  name = "kakship-${kakship.version}";
  nativeBuildInputs = [ pkgs.makeWrapper ];
  paths = [ kakship ];
  postBuild = ''
    rm "$out/bin/kakship"
    makeWrapper "${kakship}/bin/kakship" "$out/bin/kakship" --add-flags "--starship_path=${starship}/bin/starship"
  '';
}
