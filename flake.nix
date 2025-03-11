{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShell = pkgs.mkShell.override {
        stdenv = if pkgs.stdenv.isLinux then
          pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv
        else
          pkgs.clangStdenv;
      } rec {
        packages = with pkgs; [
          (fenix.packages.${system}.complete.withComponents [
            "cargo"
            "rustc"
            "rust-src"
            "rustfmt"
            "rust-analyzer"
            "clippy"
          ])
          cargo-nextest
          pkg-config
          cmake
        ];
        buildInputs = with pkgs; [
          wayland
          xorg.libxcb
          libxkbcommon
          vulkan-tools
          vulkan-headers
          vulkan-loader
          libGL
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          glslang
        ];
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        RUSTFLAGS = "-Zthreads=8"; # (may cause rustc to ice?)
        MANGOHUD_DLSYM = "1"; # mangohud glium fix
      };
    });
}
