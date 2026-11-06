{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      lib = nixpkgs.lib;
      stdenv = nixpkgs.stdenv;
    in
    {
      devShells.${system}.default = import ./devshell.nix { inherit pkgs; };
      packages.${system}.default = import ./build-package.nix pkgs;
    };
}
