{
  description = "Development shell for cross-compiling to 32-bit Windows MSVC";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rust-toolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
          targets = [  ];
        };

        buildTools = with pkgs; [
          rust-toolchain
          libllvm
          clang
          lld
          wine
        ];

        devLibs = with pkgs; [
          libxkbcommon
          wayland
          libX11
          libXcursor
          libXrandr
          libXi
          alsa-lib
          fontconfig
          freetype
          libGL
          vulkan-loader
          vulkan-validation-layers
          mesa
          vulkan-tools
        ];
      in
      {
        nixpkgs.config.allowUnfree = true;

        devShells.default = pkgs.mkShell {
          buildInputs = buildTools ++ devLibs;

          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
              pkgs.lib.makeLibraryPath (
                with pkgs;
                [
                  wayland
                  libxkbcommon
                  vulkan-loader
                  mesa
                  libGL
                ]
              )
            }"
          '';
        };
      }
    );
}
