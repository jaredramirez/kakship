{
  description = "Kakship";

  outputs = { self, nixpkgs }:
    {
      defaultPackage.x86_64-darwin =
        let
          pkgs = import nixpkgs {
            system = "x86_64-darwin";
          };
        in
          import ./default.nix { inherit pkgs; };

      defaultPackage.x86_64-linux =
        let
          pkgs = import nixpkgs {
            system = "x86_64-linux";
          };
        in
          import ./default.nix { inherit pkgs; };
    };
}
