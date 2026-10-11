{
  description = "khoi flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (system:
      let
        pkgs = import nixpkgs { inherit system; };

        khoi-default = pkgs.rustPlatform.buildRustPackage {
          pname = "khoi";
          version = "latest";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            wayland
            libxkbcommon
          ];

          postFixup = ''
            patchelf --add-rpath ${
              pkgs.lib.makeLibraryPath (with pkgs; [
                wayland
                libxkbcommon
                vulkan-loader
                libGL
              ])
            } $out/bin/khoi
          '';

          doCheck = false;
        };
      in
      {
        packages.default = khoi-default;

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo cargo-edit rustc rust-analyzer rustfmt clippy
            tokei
            nil

            # Wayland
            pkg-config
            wayland
            wayland-protocols
            libxkbcommon

            # GPU (wgpu)
            vulkan-loader
            vulkan-tools
            libGL
          ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.wayland
            pkgs.libxkbcommon
            pkgs.vulkan-loader
            pkgs.libGL
          ] + ":/run/opengl-driver/lib";
        };
      }
    );
}
