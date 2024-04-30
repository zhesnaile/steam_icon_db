{
  description = "Rust Project with Flake Parts";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-parts.url = "github:hercules-ci/flake-parts";
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    devenv.url = "github:cachix/devenv";
  };

  outputs = inputs@{ self, nixpkgs, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-linux" ];

      imports = [
        inputs.devenv.flakeModule
      ];

      perSystem = { self', pkgs, system, ... }:
      let
        rustVersion = "1.76.0";
        rustPkgs = pkgs.rustBuilder.makePackageSet {
          inherit rustVersion;
          packageFun = import ./backend/Cargo.nix;
        };

      in 

      {
        _module.args.pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
          overlays = [ inputs.cargo2nix.overlays.default (import inputs.rust-overlay) ];
        };

        devenv.shells.default = {
          languages = {
            rust.enable = true;
            javascript = {
                enable = true;
                corepack.enable = true;

                directory = "frontend";
                pnpm = {
                    enable = true;
                    install.enable = true;
                };
            };
          };
          packages = with pkgs; [
            steamcmd
            openssl.dev
            pkg-config
          ];

          scripts = {
            build-backend.exec = "cd backend && cargo build --release";
            run-backend.exec = "cd backend && cargo run --release";
            test-backend.exec = "cd backend && cargo test --release";
          };

          processes = {
            backend.exec = "cd backend && cargo run";
          };
          
          services.redis.enable = true;
        };

        # Packages
        packages = rec {
            steam_icon_db_api = (rustPkgs.workspace.steam_icon_db_api {}).bin;
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
