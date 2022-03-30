#{ pkgs ? import <nixpkgs> {} }:
# 
#pkgs.mkShell {
#  name = "go-env";
#  buildInputs = [
#    pkgs.go
#  ];
#}

{ pkgs ? import <nixpkgs> {} }:
 
pkgs.mkShell rec {
  name = "env";

  shellHook = ''
	nix-channel --update
    nix-env -iA nixpkgs.go_1_18
  '';
}