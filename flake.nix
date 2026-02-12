{
  inputs = {
    nixpkgs.url = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        libraries = with pkgs; [
        ];
        packages = with pkgs; [
          # rust
          rust-analyzer
          rustfmt
          clippy
          rustc
          cargo
          cargo-deny
          cargo-edit
          bacon
        ];
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = packages ++ [
            (
              # Needed for rust-analyzer
              pkgs.rust-bin.stable.latest.default.override {
                extensions = [ "rust-src" ];
              }
            )
          ];

          # Needed for rust-analyzer
          RUST_SRC_PATH = "${
            pkgs.rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
            }
          }/lib/rustlib/src/rust/library";
        };
      }
    );
}
