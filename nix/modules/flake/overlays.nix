{ self, ... }:
{
  flake.overlays.default =
    final: prev:
    let
      inherit (prev.stdenv.hostPlatform) system;
    in
    if builtins.hasAttr system self.packages then self.packages.${system} else { };
}
