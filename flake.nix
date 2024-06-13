{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forAllSystems =
        function: nixpkgs.lib.genAttrs systems (system: function nixpkgs.legacyPackages.${system});
    in
    {
      packages = forAllSystems (pkgs: rec {
        default = enkei;
        enkei = pkgs.callPackage ./nix/default.nix { rev = self.dirtyRev or self.rev; };
      });

      devShells = forAllSystems (pkgs: {
        default = pkgs.callPackage ./nix/shell.nix { };
      });

      overlays.default = _: prev: {
        enkei = prev.callPackage ./nix/default.nix { rev = self.dirtyRev or self.rev; };
      };

      formatter = forAllSystems (pkgs: pkgs.nixfmt-rfc-style);

      homeManagerModules.default = import ./nix/hm-module.nix self;
    };
}
