{ pkgs }:

let
  libPath = with pkgs; lib.makeLibraryPath [
    libGL
    libxkbcommon
    wayland
  ];
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    wrapGAppsHook4
    cargo
    cargo-tauri
    nodejs
    rustc
    gst_all_1.gst-plugins-base
  ];

  buildInputs = with pkgs; [
    librsvg
    webkitgtk_4_1
  ];

  packages = with pkgs; [
    cargo
    cargo-tauri
    nodejs
    rustc
    gcc
    rust-analyzer
    pkg-config
    wrapGAppsHook4
  ];

  RUST_LOG = "debug";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  LD_LIBRARY_PATH = libPath;

  shellHook = ''
    export XDG_DATA_DIRS="$GSETTINGS_SCHEMAS_PATH"
  '';
}