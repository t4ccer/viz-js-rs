{
  description = "rust-yew-template";
  inputs = {
    nixpkgs.follows = "rust-overlay/nixpkgs";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    pre-commit-hooks-nix = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.nixpkgs-stable.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
    };
  };
  outputs = inputs @ {self, ...}:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.pre-commit-hooks-nix.flakeModule
      ];

      # `nix flake show --impure` hack
      systems =
        if builtins.hasAttr "currentSystem" builtins
        then [builtins.currentSystem]
        else inputs.nixpkgs.lib.systems.flakeExposed;

      perSystem = {
        config,
        self',
        inputs',
        pkgs,
        system,
        ...
      }: {
        _module.args.pkgs = import self.inputs.nixpkgs {
          inherit system;
          overlays = [inputs.rust-overlay.overlays.rust-overlay];
        };

        pre-commit.settings = {
          src = ./.;
          hooks = {
            alejandra.enable = true;
            statix.enable = true;
            rustfmt.enable = true;
          };
        };

        devShells.default = pkgs.mkShell {
          shellHook = config.pre-commit.installationScript;
          nativeBuildInputs = [
            pkgs.alejandra
            pkgs.fd
            pkgs.trunk
            pkgs.wasm-bindgen-cli
            (pkgs.rust-bin.fromRustupToolchain {
              channel = "stable";
              components = ["rust-analyzer" "rust-src" "rustfmt" "rustc" "cargo"];
              targets = ["wasm32-unknown-unknown"];
            })
          ];
        };
        formatter = pkgs.alejandra;
      };
    };
}
