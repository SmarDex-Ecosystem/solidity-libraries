{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    foundry = {
      url = "github:shazow/foundry.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, foundry, rust }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ foundry.overlay rust.overlays.default ];
        };
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        stdenv = if pkgs.stdenv.isLinux then pkgs.stdenvAdapters.useMoldLinker pkgs.stdenv else pkgs.stdenv;
      in
      {
        devShells.default = pkgs.mkShell.override { inherit stdenv; } {
          buildInputs = [
            pkgs.rust-analyzer-unwrapped
            toolchain
          ];
          packages = with pkgs; [
            nodejs-slim_20
            foundry-bin
            lcov
          ];

          shellHook = ''
            npm i
            forge soldeer install
          '';

          RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
        };
      });
}
