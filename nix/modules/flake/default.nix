{ inputs, ... }:
{
  systems = import inputs.systems;

  imports = [
    ./overlays.nix
    ./packages.nix
    ./shells.nix
    ./treefmt.nix
  ];
}
