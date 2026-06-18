{
  inputs = {
    nixpkgs.url = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    fenix,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};

      inherit (pkgs) mkShell;

      rustToolchain = fenix.packages.${system}.stable.withComponents [
        "rustc"
        "rustfmt"
        "cargo"
        "clippy"
        "rust-src"
        "rust-analyzer"
      ];

      shell = mkShell {
        name = "sfd-shell";

        buildInputs =
          [rustToolchain]
          ++ (with pkgs; [
            cargo-expand
            cargo-machete
            cargo-autoinherit
            cmake
            openssl
            pkg-config
          ]);
      };
    in {
      devShells.default = shell;
    });
}
