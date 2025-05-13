{ self, ... }:
{
  perSystem =
    { pkgs, ... }:
    {
      packages.default = pkgs.callPackage "${self}/nix/packages/lrxed" { };
    };
}
