{ self }:
{
  pkgs,
  lib,
  config,
  ...
}:
let
  inherit (pkgs.stdenv.hostPlatform) system isDarwin;

  cfg = config.programs.lrxed;
  package = if cfg.package != null then cfg.package else self.packages.${system}.default;

  settingsFormat = pkgs.formats.toml { };
  settingsFile = settingsFormat.generate "lrxed-config.toml" cfg.settings;

  settingsPath.xdg = "lrxed/config.toml";
  settingsPath.darwin = "Library/Application Support/LunaPresent.lrxed/config.toml";
in
{
  options.programs.lrxed = {
    enable = lib.mkEnableOption "lrxed";

    package = lib.mkPackageOption pkgs "lrxed" {
      default = null;
      nullable = true;
    };

    settings = lib.mkOption {
      inherit (settingsFormat) type;
      default = { };

      description = ''
        Configuration written to `$XDG_CONFIG_HOME/${settingsPath.xdg}` and `${settingsPath.darwin}`
        on MacOS.
      '';
    };
  };

  config = lib.mkIf cfg.enable {
    home.packages = [ package ];
    xdg.configFile.${settingsPath.xdg}.source = settingsFile;
    home.file.${settingsPath.darwin}.source = lib.mkIf isDarwin settingsFile;
  };
}
