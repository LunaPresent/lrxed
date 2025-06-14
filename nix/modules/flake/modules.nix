{ self, ... }:
{
  flake.homeModules.default = import "${self}/nix/modules/home" { inherit self; };
}
