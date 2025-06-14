{ inputs, ... }:
{
  systems = import inputs.systems;

  imports = [
    ./modules.nix
    ./overlays.nix
    ./packages.nix
    ./shells.nix
    ./treefmt.nix
  ];
}
