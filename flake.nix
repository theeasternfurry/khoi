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

          doCheck = false;
        };
      in
      {
        packages.default = khoi-default;

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo cargo-edit rustc rust-analyzer rustfmt
            tokei
            nil
          ];
        };
      }
    );
}
