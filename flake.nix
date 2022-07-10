{
  description = "C Funciton Signatures Parser";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/2b71ddd869ad592510553d09fe89c9709fa26b2b";
    rust-overlay = { 
      url = "github:oxalica/rust-overlay/7da3fe8b192f96dfe8b2223f4fe55e2e40850a2a"; 
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlay ];
          };
        in
        {
          devShell = pkgs.mkShell rec {
            buildInputs = with pkgs; [
              rust-bin.stable.latest.default

              libxkbcommon
              libGL
              vulkan-loader

              xorg.libXcursor
              xorg.libXrandr
              xorg.libXi
              xorg.libX11
              xorg.libxcb

              pkgconfig
            ];
            LD_LIBRARY_PATH = "/run/opengl-driver/lib/:${pkgs.lib.makeLibraryPath buildInputs}";
            WINIT_UNIX_BACKEND = "x11";
          };
        }
    );
}
