{
  perSystem =
    { pkgs, ... }:
    {
      devShells.default = pkgs.mkShell {
        buildInputs = [
          pkgs.alsa-lib.dev
        ];

        nativeBuildInputs = [
          pkgs.pkg-config
          pkgs.cargo
        ];
      };
    };
}
