{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
    buildInputs = with pkgs; [
        libiconv
        rustc
        rustfmt
        cargo
    ];
}
