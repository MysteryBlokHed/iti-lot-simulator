{
  description = "An optimized simulator for an old assignment.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    fenix,
    crane,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        toolchain = fenix.packages.${system}.default.toolchain;
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

        commonArgs = {
          pname = "iti-lot-simulator";
          version = "0.1.0";

          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;
        };

        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {});

        clippy = craneLib.cargoClippy (commonArgs
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

        package = craneLib.buildPackage (commonArgs
          // {
            inherit cargoArtifacts;
          });
      in {
        formatter = pkgs.alejandra;

        packages.default = package;

        checks = {
          inherit clippy;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [package];
          packages = [toolchain];
        };
      }
    );
}
