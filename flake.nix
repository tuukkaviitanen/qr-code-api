{
  description = "A development shell with specific Rust toolchain";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";

    rust-nixpkgs = {
      url = "github:NixOS/nixpkgs/1cb1c02a6b1b7cf67e3d7731cbbf327a53da9679"; # has Rust 1.86.0
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-nixpkgs,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
        };

        rustPkgs = import rust-nixpkgs {inherit system;};
      in {
        devShells.default = pkgs.mkShell {
          packages = with rustPkgs; [
            rustc
            cargo
            rustfmt
            clippy
          ];
        };
      }
    );
}
