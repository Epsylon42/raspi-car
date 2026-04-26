# Optional NixOS module. Import it from your system flake via
#   imports = [ inputs.balena-wifi-connect.nixosModules.default ];
# and enable with `services.balena-wifi-connect.enable = true;`.
#
# This runs wifi-connect as a oneshot at boot: if NetworkManager already has
# a working connection it exits immediately; otherwise it raises the AP and
# sits until the user completes the portal flow.

{ config, lib, pkgs, ... }:

let
  cfg = config.services.balena-wifi-connect;
in
{
  options.services.balena-wifi-connect = {
    enable = lib.mkEnableOption "balena wifi-connect captive-portal provisioning";

    package = lib.mkOption {
      type = lib.types.package;
      default = pkgs.balena-wifi-connect;
      defaultText = lib.literalExpression "pkgs.balena-wifi-connect";
      description = "wifi-connect package to use (requires the flake overlay).";
    };

    portalSsid = lib.mkOption {
      type = lib.types.str;
      default = "WiFi Connect";
      description = "SSID advertised by the setup access point.";
    };

    portalPassphrase = lib.mkOption {
      type = lib.types.nullOr lib.types.str;
      default = null;
      description = "WPA2 passphrase for the setup AP. null = open network.";
    };

    portalGateway = lib.mkOption {
      type = lib.types.str;
      default = "192.168.42.1";
      description = "Gateway IP for the setup AP subnet.";
    };

    interface = lib.mkOption {
      type = lib.types.nullOr lib.types.str;
      default = null;
      example = "wlan0";
      description = "Wireless interface to use. null = auto-detect.";
    };

    extraArgs = lib.mkOption {
      type = lib.types.listOf lib.types.str;
      default = [ ];
      description = "Extra command-line flags passed to wifi-connect.";
    };
  };

  config = lib.mkIf cfg.enable {
    networking.networkmanager.enable = lib.mkDefault true;

    systemd.services.wifi-provisioning-check = {
      description = "WiFi provisioning check";
      after = [ "NetworkManager.service" ];
      before = [ "sshd.service" ];
      wants = [ "NetworkManager.service" ];
      wantedBy = [ "multi-user.target" ];
      onFailure = [ "balena-wifi-connect.service" ];
      serviceConfig = {
        Type = "oneshot";
        ExecStart = "${pkgs.networkmanager}/bin/nm-online -t 60";
      };
    };
    systemd.services.balena-wifi-connect = {
      description = "balena wifi-connect (captive-portal WiFi provisioning)";
      requires = [ "NetworkManager.service" ];

      serviceConfig = {
        Type = "oneshot";
        ExecStart = lib.escapeShellArgs (
          [ (lib.getExe cfg.package)
            "--portal-ssid" cfg.portalSsid
            "--portal-gateway" cfg.portalGateway
          ]
          ++ lib.optionals (cfg.portalPassphrase != null)
               [ "--portal-passphrase" cfg.portalPassphrase ]
          ++ lib.optionals (cfg.interface != null)
               [ "--portal-interface" cfg.interface ]
          ++ cfg.extraArgs
        );
        Restart = "on-failure";
        RestartSec = 5;
      };
    };
  };
}
