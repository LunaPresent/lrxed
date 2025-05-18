{
  perSystem =
    { pkgs, ... }:
    {
      devShells.default = pkgs.mkShell {
        RUST_SRC_PATH = toString pkgs.rust.packages.stable.rustPlatform.rustLibSrc;

        buildInputs = if pkgs.stdenv.hostPlatform.isLinux then [ pkgs.alsa-lib.dev ] else [ ];

        nativeBuildInputs = [
          pkgs.cargo
          pkgs.pkg-config
          pkgs.rustc
        ];
      };
    };
}
