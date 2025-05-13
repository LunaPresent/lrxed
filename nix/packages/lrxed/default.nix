{
  rustPlatform,
  pkg-config,
  alsa-lib,
}:
let
  root = ../../../.;
  cargo = builtins.fromTOML (builtins.readFile "${root}/Cargo.toml");
in
rustPlatform.buildRustPackage {
  pname = cargo.package.name;
  version = cargo.package.version;
  src = root;
  cargoLock.lockFile = "${root}/Cargo.lock";

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ alsa-lib ];

  meta = {
    description = "A tui application for synchronising lyrics.";
    homepage = "https://github.com/LunaPresent/lrxed";
  };
}
