{
  description = "A devShell that can run three-d examples";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils}:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rust = pkgs.rust-bin.nightly.latest.default.override{
          targets = ["wasm32-unknown-unknown"];
        };
        
        graphicLibs = with pkgs; lib.makeLibraryPath [
          libGL
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
         ];    

        bacon = pkgs.bacon;

        web_run = pkgs.writeScriptBin "run" ./run.fish;
        browse = pkgs.writeScriptBin "browse" "firefox http://127.0.0.1:8080/";

        
        bacon_script = pkgs.writeScriptBin "bac" ''
          export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${graphicLibs}
          ${bacon}/bin/bacon "$@"
        '';
        
        cargo_script = pkgs.writeScriptBin "car" ''
          export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${graphicLibs}
          export PATH=$PATH:${rust}/bin
          ${rust}/bin/cargo "$@"
        '';

        buildDeps = with pkgs; [
          openssl
          pkg-config
          cargo_script
          bacon_script
          rust
          # nodejs_20
          wasm-pack
          wasm-bindgen-cli
          git-lfs
        ];

        utils = with pkgs; [
          #  video driver info
          pciutils 
          glxinfo
          nil
          gdb
          lldb
          rust-analyzer
          python311
          trunk
          web_run
          browse
          freecad
        ];
      in
      with pkgs;
      {
        devShells.default = mkShell {
          name = "rust graphics env"; 
          buildInputs = buildDeps ++ utils;
          shellHook = ''
            echo Entering rust env!
            echo 'use "car" or "bac" to run cargo or bacon with: LD_LIBRARY_PATH='
            echo "    ${graphicLibs}" | sed 's/:/\n    /g'
          '';
        };
      }
    );
}
