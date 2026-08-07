{
  description = "Development shell for cross-compiling to 32-bit Windows MSVC";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    comet = {
      url = "github:iced-rs/comet";
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      comet,
      crane,
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
          targets = [ ];
        };

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

        craneLib = crane.mkLib pkgs;

        commonArgs = {
          src = comet;
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ openssl ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [ xorg.libX11 xorg.libXrandr ];
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        iced_comet = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });

        buildTools = with pkgs; [
          rust-toolchain
          libllvm
          clang
          lld
          wine
          iced_comet
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
