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
          # Утилиты для сборки C-зависимостей (КРИТИЧЕСКИ ВАЖНО для PKG_CONFIG_PATH)
          pkg-config 
          
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
          dbus
          xdg-desktop-portal

          # GTK3 и его зависимости
          gtk3
          glib
          cairo
          pango
          gdk-pixbuf
          atk
          
          # Темы и схемы (нужны для shellHook ниже)
          gsettings-desktop-schemas
          adwaita-icon-theme
        ];

        craneLib = crane.mkLib pkgs;

        commonArgs = {
          src = comet;
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs =
            with pkgs;
            [ openssl ]
            ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
              xorg.libX11
              xorg.libXrandr
            ];
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        iced_comet = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
          }
        );

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
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = buildTools ++ devLibs;

          shellHook = ''
            # Настраиваем пути к иконкам и темам
            export XDG_DATA_DIRS="${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.adwaita-icon-theme}/share:${pkgs.gtk3}/share:$XDG_DATA_DIRS"
            
            # Явно добавляем пути к .pc файлам GTK, на случай если что-то их перезаписывает
            export PKG_CONFIG_PATH="${pkgs.glib.dev}/lib/pkgconfig:${pkgs.gtk3.dev}/lib/pkgconfig:${pkgs.cairo.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
            
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
              pkgs.lib.makeLibraryPath (
                with pkgs;
                [
                  wayland
                  libxkbcommon
                  vulkan-loader
                  mesa
                  libGL
                  gtk3
                  glib
                  cairo
                  pango
                  gdk-pixbuf
                  atk
                ]
              )
            }"
          '';
        };
      }
    );
}
