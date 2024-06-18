{
  description = "A cli for the Firefly keyboard customization";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {nixpkgs, ...} @ inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      perSystem = {
        config,
        system,
        lib,
        ...
      }: let
        overlays = [(import inputs.rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustPlatform = pkgs.makeRustPlatform {
          rustc = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
          cargo = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
        };
        graphicLibs = with pkgs; [
          libGL
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          xorg.libXxf86vm
          xorg.libxcb
          libxkbcommon
          wayland
        ];
      in {
        devShells.default = pkgs.mkShell rec {
          inputsFrom = [config.packages.firefly-cli];
          packages = with pkgs;
            [
              pre-commit
              glxinfo
              vscode-extensions.llvm-org.lldb-vscode
              taplo
              glib-networking
              wireshark
              usbutils
            ]
            ++ graphicLibs;
          LD_LIBRARY_PATH = "${lib.makeLibraryPath packages}";

          GIO_MODULE_DIR = "${pkgs.glib-networking}/lib/gio/modules/";
        };

        packages =
          {
            firefly-cli= rustPlatform.buildRustPackage {
              pname = "firefly-cli";
              version = "0.1.0";
              src = ./.;
              cargoLock = {
                lockFile = ./Cargo.lock;
              };

              nativeBuildInputs = with pkgs; [
                pkg-config
              ];
              buildInputs = with pkgs; [
              systemd
              ];
            };
          }
          // {default = config.packages.firefly-cli;};

        formatter = pkgs.alejandra;
      };
    };
}
