{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  nixConfig = {
    extra-substituters = [ "https://ultraflame4.cachix.org" ];
    extra-trusted-substituters = [ "https://ultraflame4.cachix.org" ];
    extra-trusted-public-keys = [
      "ultraflame4.cachix.org-1:PcGRhKKbj0tBNtv/wyRw8AF/Vk+AFS/DhO7fujE5RVo="
    ];
  };
  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    let

      supportedSystems = [
        "aarch64-linux"
        "i686-linux"
        "x86_64-linux"
      ];

    in

    flake-utils.lib.eachSystem supportedSystems (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = import ./devshell.nix {
          inherit pkgs;
        };

        packages.default = import ./build-package.nix pkgs;
      }
    );

}
