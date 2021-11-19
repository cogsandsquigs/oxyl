{ pkgs ? import <nixpkgs> {} }:
 
pkgs.mkShell {
  name = "go-env";
  buildInputs = [
    pkgs.go
  ];
}