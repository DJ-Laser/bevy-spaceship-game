{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
  } @ inputs: let
    system = "x86_64-linux";
    overlays = [(fenix.overlays.default)];
    pkgs = import nixpkgs {inherit system overlays;};
    lib = pkgs.lib;

    rustToolchain = pkgs.fenix.stable.toolchain;

    bevyDeps = with pkgs; [
      pkg-config

      # for Linux
      # Audio (Linux only)
      alsa-lib
      # Cross Platform 3D Graphics API
      vulkan-loader
      # For debugging around vulkan
      vulkan-tools
      # Other dependencies
      libudev-zero
      libx11
      libxcursor
      libxi
      libxrandr
      libxkbcommon
      wayland
    ];
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [rustToolchain] ++ bevyDeps;
      RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/src";
      LD_LIBRARY_PATH = with pkgs;
        lib.makeLibraryPath [
          vulkan-loader
          libx11
          libxi
          libxcursor
          libxkbcommon
          wayland
        ];
    };
  };
}
