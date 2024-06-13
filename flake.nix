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
        enkei = pkgs.callPackage ./default.nix { rev = self.dirtyRev or self.rev; };
      });

      devShells = forAllSystems (pkgs: {
        default = pkgs.callPackage ./shell.nix { };
      });

      overlays.default = _: prev: {
        enkei = prev.callPackage ./default.nix { rev = self.dirtyRev or self.rev; };
      };

      formatter = forAllSystems (pkgs: pkgs.nixfmt-rfc-style);
    };
}
