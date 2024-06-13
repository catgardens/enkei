{
  lib,
  ncurses,
  rustPlatform,
  rev ? "dirty",
}:
let
  p = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  pname = p.name;
  inherit (p) version;

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.intersection (lib.fileset.fromSource (lib.sources.cleanSource ./.)) (
      lib.fileset.unions [
        ./Cargo.toml
        ./Cargo.lock
        ./src
      ]
    );
  };

  cargoLock.lockFile = ./Cargo.lock;

  env = {
    BUILD_REV = rev;
  };

  buildInputs = [ ncurses.dev ];

  meta = {
    inherit (p) description homepage;
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [
      comfysage
      isabelroses
    ];
    mainProgram = "enkei";
  };
}
