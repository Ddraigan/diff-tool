{
  description = "Git diffing TUI, making it easy to quickly check a diff";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
  in {
    packages."x86_64-linux".default = pkgs.rustPlatform.buildRustPackage {
      name = "diff-tool";
      src = ./.;
      buildInputs = [pkgs.glib];
      nativeBuildInputs = [pkgs.pkg-config];
      cargoLock.lockFile = ./Cargo.lock;
    };

    devShells."x86_64-linux".default = pkgs.mkShell {
      buildInputs = with pkgs; [
        cargo
        rustc
        rustfmt
        clippy
        rust-analyzer
      ];
      env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    };
  };
}
