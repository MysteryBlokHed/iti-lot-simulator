{
  description = "An optimized simulator for an old assignment.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    fenix,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};

        toolchain = fenix.packages.${system}.default.toolchain;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        };

        rustPackage = rustPlatform.buildRustPackage {
          pname = "iti-lot-simulator";
          version = "0.1.0";

          src = ./.;
          cargoLock. lockFile = ./Cargo.lock;
        };
      in {
        formatter = pkgs.alejandra;

        packages.default = rustPackage;

        devShells.default = pkgs.mkShell {
          inputsFrom = [rustPackage];
          packages = [toolchain];
        };
      }
    );
}
