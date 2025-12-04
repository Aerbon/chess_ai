{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell (with pkgs; {
  nativeBuildInputs = [ pkg-config fontconfig ];
  buildInputs = [
    rust-analyzer
    cargo
    rustc
    rustfmt
    alsa-lib
    udev
    wayland-protocols
    wayland
    libxkbcommon
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath [
    wayland
    libglvnd
    libxkbcommon
    vulkan-loader
  ];
})