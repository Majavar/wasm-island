{
  description = "Heightmap generator based of 2d noise functions";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.stable."1.82.0".minimal.override {
          extensions = [ "rustfmt" "clippy" "rust-src" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
        rustPlatform = pkgs.makeRustPlatform { cargo = rust; rustc = rust; };
        dioxus-cli = rustPlatform.buildRustPackage rec {
          pname = "dioxus-cli";
          version = "0.5.7";
          src = pkgs.fetchCrate {
            inherit pname version;
            sha256 = "sha256-/LeMh5WX4dvkveu5w6qBQLbtoi5yUW6iad0YatA/tMQ=";
          };
          cargoLock = {
            lockFileContents = (builtins.readFile ./.nix/dioxus-cli.lock);
          };
          postPatch = ''
            rm Cargo.lock
            ln -s ${./.nix/dioxus-cli.lock} Cargo.lock
          '';

          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];

          checkFlags = [
          # requires network access, thanks nixpkgs for figuring this out
            "--skip=server::web::proxy::test::add_proxy"
            "--skip=server::web::proxy::test::add_proxy_trailing_slash"
          ];

          # Tell openssl-sys to use the system's provided openssl.
          OPENSSL_NO_VENDOR = 1;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell.override { stdenv = stdenvNoCC; } {
          buildInputs = [
            dioxus-cli
            rust
          ];
        };
      }
    );
}
