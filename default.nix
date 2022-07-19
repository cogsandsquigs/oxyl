{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "go-env";
  buildInputs = [
	  pkgs.cargo
	  pkgs.rustc
    pkgs.go_1_18
  ];
}