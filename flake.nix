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
      inherit (pkgs.lib) getExe;

      rustToolchain = fenix.packages.${system}.stable.withComponents [
        "rustc"
        "rustfmt"
        "cargo"
        "clippy"
        "rust-src"
        "rust-analyzer"
      ];

      sqliteVec0 = "${pkgs.sqlite-vec}/lib/vec0.so";

      sqlite = pkgs.writeShellScriptBin "sqlite3" ''
        exec ${getExe pkgs.sqlite} -cmd ".load ${sqliteVec0}" "$@"
      '';

      shell = mkShell {
        name = "sfd-shell";

        buildInputs =
          [rustToolchain sqlite]
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
