{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        inherit (nixpkgs) lib;
        pkgs = nixpkgs.legacyPackages.${system};
        rpath = lib.makeLibraryPath (with pkgs; [
          wlroots
          wayland
          waylandpp 
          wayland-utils
        ]);
      in
      {
        devShells.default = pkgs.mkShell {

          dependencies = with pkgs; [
            pkg-config
            wayland
            waylandpp
            wayland-utils
          ];

          OPENSSL_DIR= lib.makeLibraryPath (with pkgs; [  
            # openssl 
            ]);
        
          LD_LIBRARY_PATH = rpath;
        };
      }
    );
}