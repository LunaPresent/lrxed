{
  alsa-lib,
  lib,
  pkg-config,
  rustPlatform,
  stdenv,
}:
let
  root = ../../../.;
  cargo = lib.importTOML "${root}/Cargo.toml";
in
rustPlatform.buildRustPackage {
  pname = cargo.package.name;
  version = cargo.package.version;
  src = root;
  cargoLock.lockFile = "${root}/Cargo.lock";

  nativeBuildInputs = [ pkg-config ];
  buildInputs = if stdenv.hostPlatform.isLinux then [ alsa-lib ] else [ ];

  meta = {
    description = "A tui application for synchronising lyrics.";
    homepage = "https://github.com/LunaPresent/lrxed";
  };
}
