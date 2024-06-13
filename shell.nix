{
  clippy,
  rustfmt,
  cargo-shear,
  callPackage,
  rust-analyzer,
  ncurses,
}:
let
  mainPkg = callPackage ./default.nix { };
in
mainPkg.overrideAttrs (oa: {
  nativeBuildInputs = [
    # Additional rust tooling
    clippy
    rustfmt
    rust-analyzer
    cargo-shear
    ncurses.dev
  ] ++ (oa.nativeBuildInputs or [ ]);
})
