{
  description = "Rust Project with Flake Parts";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-parts.url = "github:hercules-ci/flake-parts";
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
  };

  outputs = inputs@{ self, nixpkgs, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-linux" ];

      perSystem = { self', pkgs, system, ... }:
      let
        rustVersion = "1.76.0";
        rustPkgs = pkgs.rustBuilder.makePackageSet {
          inherit rustVersion;
          packageFun = import ./Cargo.nix;
        };

      in 

      {
        _module.args.pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
          overlays = [ inputs.cargo2nix.overlays.default (import inputs.rust-overlay) ];
        };

        # Development Shell
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-bin.stable.${rustVersion}.default
            cargo-edit
            cargo-watch
            clippy
            rustfmt
            steamcmd
          ];
        };

        # Packages
        packages = rec {
            steam_icon_db_api = (rustPkgs.workspace.steam_icon_db_api {}).bin.overrideAttrs (oldAttrs: {
              buildInputs = (oldAttrs.buildInputs or []) ++ [ pkgs.steamcmd ];
            });
            default = steam_icon_db_api;
        };

        apps = {
          steam_icon_db_api = {
              type = "app";
              program = "${self'.packages.router}/bin/steam_icon_db_api";
              buildInputs = with pkgs; [ steamcmd ];
            };
          };
      };
    };
}
