{ pkgs ? import <nixpkgs> { } }:

let
  libPath = with pkgs; lib.makeLibraryPath [
    fontconfig
    libxkbcommon
    wayland
  ];
in
pkgs.mkShell {
  packages = with pkgs; [
    openssl
    pkg-config
  ];

  LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${libPath}";
}
