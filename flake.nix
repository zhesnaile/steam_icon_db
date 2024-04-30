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
      {
        _module.args.pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
          overlays = [ (import inputs.rust-overlay) ];
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
          steam_icon_db_api = pkgs.rustPlatform.buildRustPackage rec {
            pname = "steam_icon_db_api";
            version = "0.1.0";

            src = ./backend;

            doCheck = false;

            cargoHash = "sha256-JM3Nt6Zn4bkLxw54s2YnHrHkT200Pgov1X6XZdXFD/4=";

            buildInputs = with pkgs; [ steamcmd openssl.dev ];
            nativeBuildInputs = with pkgs; [ steamcmd pkg-config ];

          };

          default = steam_icon_db_api;

        };

        apps = {
          steam_icon_db_api = {
              type = "app";
              program = "${self'.packages.steam_icon_db_api}/bin/steam_icon_db_api";
              buildInputs = with pkgs; [ steamcmd openssl.dev ];
            };
          };
      };
    };
}
