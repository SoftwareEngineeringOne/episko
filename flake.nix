{
  description = "A basic flake with a shell";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.systems.url = "github:nix-systems/default";
  inputs.flake-utils = {
    url = "github:numtide/flake-utils";
    inputs.systems.follows = "systems";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rust-analyzer
            cargo
            rustc
            rustfmt
          ];
          nativeBuildInputs = with pkgs; [
            bacon
            bun
            cargo
            cargo-expand
            cargo-tauri
            cargo-tauri
            cargo-update
            cargo-watch
            gobject-introspection
            gtk3
            gtk4
            nodejs
            nushell
            pkg-config
            sql-formatter
            sqlx-cli
            uv
            xsel
            wrapGAppsHook
          ];

          buildInputs = with pkgs; [
            at-spi2-atk
            atkmm
            cairo
            gdk-pixbuf
            glib
            gtk3
            gtk4
            harfbuzz
            librsvg
            libsoup_3
            openssl
            pango
            webkitgtk_4_1
          ];

          shellHook = ''
            export XDG_DATA_DIRS=${pkgs.gtk3.out}/share/gsettings-schemas/gtk+3-3.24.48
          '';
        };
      }
    );
}
