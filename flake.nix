{
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs;
    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-compat, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
        aoc19 = pkgs.rustPlatform.buildRustPackage {
          pname = manifest.name;
          version = manifest.version;
          src = builtins.path { path = ./.; name = "aoc19"; };

          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "aoclib-rs-0.0.14" = "sha256-WRzigGCgqWQd9mFpFE1EiffAi57vD+M86WxKIHIAJMY=";
            };
          };
        };
        aoc19-shell = pkgs.mkShell {
          inputsFrom = [ aoc19 ];
          packages = with pkgs; [
            clippy
            rustfmt
          ];
        };
      in
      {
        packages = {
          inherit aoc19;
          default = aoc19;
        };
        devShells = {
          inherit aoc19-shell;
          default = aoc19-shell;
        };
      }
    );
}
