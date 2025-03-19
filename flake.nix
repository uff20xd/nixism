{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgsFor = nixpkgs.legacyPackages;

    in {

      packages = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./default { };
      });

      devShell = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./shell.nix { };
      });

  };
}
