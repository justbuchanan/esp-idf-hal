# copied from https://github.com/ChocolateLoverRaj/rust-esp32c3-examples/commit/63542b30dd1b79d94bf8e80d612749bda7a1cec0
{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
      fhs = pkgs.buildFHSEnv {
        name = "fhs-shell";
        targetPkgs = pkgs: with pkgs; [
          gcc

          pkg-config
          libclang.lib
          gnumake
          cmake
          ninja

          git
          wget

          rustup
          cargo-generate

          espflash
          python3
          python3Packages.pip
          python3Packages.virtualenv
          python3Packages.wxpython
          # python3Packages.yaml # TODO: layout.py needs yaml pkg
          # python3Packages.wxtools
          # wxpython
          ldproxy
          trunk
          wasm-bindgen-cli
          libxml2
        ];
      };
    in
    {
      devShells.default = fhs.env;
      formatter = pkgs.nixpkgs-fmt;
    }
    );
}
