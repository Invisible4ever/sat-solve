{
  description = "satsolve - a little SATsolver";
  inputs = {
    rust-overlay.url = github:oxalica/rust-overlay;
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }: 
  flake-utils.lib.eachDefaultSystem (system:
  let
    import_nixpkgs = import nixpkgs;
    overlays = [ (import rust-overlay) ];
    pkgsLocal = import_nixpkgs { inherit system overlays; };
  in
  {
      # for loacl testing
      devShell = with pkgsLocal; mkShell {
        nativeBuildInputs = [
          rust-bin.stable.latest.default
        ]; 
      };
    });
  }
