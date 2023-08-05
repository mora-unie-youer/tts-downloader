{ pkgs, ... }:

pkgs.devShell.mkShell {
  name = "tts-downloader";
  packages = with pkgs; [
    # Toolchain required for C + Rust binaries building
    binutils
    gcc

    # Libraries required to build some crates
    pkg-config
    openssl

    # Nightly Rust toolchain
    bacon
    (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
      # Extensions which ease your development process
      extensions = [ "rust-analyzer" "rust-src" ];
    }))
  ];
}
