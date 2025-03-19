{
  description = "Foo Bar";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  outputs = { self, nixpkgs, nix }:
    let
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgsFor = forAllSystems (
        system: 
        import nixpkgs {
          inherit system;
          overlays = overlayList;
        }
      );

      overlayList = [ self.overlays.default ];
    in {
      overlays.default = final: prev: { nixism = final.callPackage ./default.nix { }; };

      packages = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./default.nix { };
      });

      nixosModules = import ./default.nix { overlays = overlayList; };
      devShells = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./shell.nix { };
      });
    };
}

