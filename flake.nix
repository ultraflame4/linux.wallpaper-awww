{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux"; # Adjust to your architecture if needed
      pkgs = nixpkgs.legacyPackages.${system};
      libPath =
        with pkgs;
        lib.makeLibraryPath [
          libGL
          libxkbcommon
          wayland
        ];
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          pkg-config
          wrapGAppsHook4
          cargo
          cargo-tauri # Optional, Only needed if Tauri doesn't work through the traditional way.
          nodejs # Optional, this is for if you have a js frontend
          rustc # Needed for dev server (npm tauri dev)
          gst_all_1.gst-plugins-base # Optional, if you get `GStreamer element appsink not found. Please install it.`
        ];

        buildInputs = with pkgs; [
          librsvg
          webkitgtk_4_1
        ];

        packages = with pkgs; [

          cargo
          cargo-tauri # Optional, Only needed if Tauri doesn't work through the traditional way.
          nodejs # Optional, this is for if you have a js frontend
          rustc # Needed for dev server (npm tauri dev)
          gcc

          rust-analyzer
          pkg-config
          wrapGAppsHook4

        ];

        RUST_LOG = "debug";
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        LD_LIBRARY_PATH = libPath;

        shellHook = ''
          export XDG_DATA_DIRS="$GSETTINGS_SCHEMAS_PATH" # Needed on Wayland to report the correct display scale
        '';
      };
    };
}
