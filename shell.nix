{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  buildInputs = [
    rustup
    nodejs-18_x
    nodePackages.npm
  ];

  shellHook = ''
    sudo npm install -g wasm4
  '';
}