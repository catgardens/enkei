self:
{
  lib,
  pkgs,
  config,
  ...
}:
let
  inherit (lib)
    mkIf
    mkOption
    mkEnableOption
    mkPackageOption
    ;

  settingsFormat = pkgs.formats.toml { };
in
{
  meta.maintainers = with lib.maintainers; [
    comfysage
    isabelroses
  ];

  options.programs.izrss = {
    enable = mkEnableOption "A fast and once simple cli todo tool";

    package = mkPackageOption self.packages.${pkgs.stdenv.hostPlatform.system} "enkei" { };

    settings = mkOption {
      inherit (settingsFormat) type;
      default = { };
      example = lib.literalExpression ''
        shadow = false;
        borders = "simple";

        colors = {
          background = "black";
          shadow = "#000000";
          view = "#d3d7cf";
          primary = "#111111";
          secondary = "#EEEEEE";
          tertiary = "#444444";
          title_primary = "#ff5555";
          title_secondary = "#ffff55";
          highlight = "#F00";
          highlight_inactive = "#5555FF";
        };
      '';
      description = ''
        Configuration written to {file}`$XDG_CONFIG_HOME/enkei/theme.toml`.

        See <https://docs.rs/cursive/latest/cursive/theme/index.html> for the documentation.
      '';
    };
  };

  config =
    let
      cfg = config.programs.izrss;
    in
    mkIf cfg.enable {
      home.packages = [ cfg.package ];

      xdg.configFile."enkei/theme.toml" = mkIf (cfg.settings != { }) {
        source = (settingsFormat.generate "enkei-theme.toml" cfg.theme);
      };
    };
}
