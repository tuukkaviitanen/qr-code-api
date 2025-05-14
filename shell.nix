{ pkgs ? import <nixpkgs> {} }:

let 
  rust-pkgs = (import (builtins.fetchTarball "https://github.com/NixOS/nixpkgs/archive/1cb1c02a6b1b7cf67e3d7731cbbf327a53da9679.tar.gz") {});#1.86.0
in
  pkgs.mkShell {
    packages = with rust-pkgs; [ 
      cargo
      rustfmt
      clippy
    ];
  }
