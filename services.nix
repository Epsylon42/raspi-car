{ pkgs, ... }:
let
  template = {
    wantedBy = [ "rc-control.target" ];
  };

  service = template: input: template // {
    serviceConfig = input;
  };
in {
  systemd.targets.rc-control = {
    wantedBy = [ "multi-user.target" ];
  };

  systemd.services = {
    v4l-setup = service template {
      Type = "oneshot";
      ExecStart = "${pkgs.v4l-setup}/bin/v4l2-setup";
      RemainAfterExit = true;
    };

    mediamtx = service {
      inherit (template) wantedBy;
      wants = [ "v4l-setup.service" ];
      after = [ 
        "v4l-setup.service"
      ];
    } {
      ExecStart = "${pkgs.mediamtx}/bin/mediamtx";
      WorkingDirectory = "/root";
      Restart = "always";
      Environment = [
        "PATH=/run/current-system/sw/bin"
        "GST_PLUGIN_SYSTEM_PATH_1_0=/run/current-system/sw/lib/gstreamer-1.0"
      ];
    };

    gpiosrv = service template {
      ExecStart = "${pkgs.gpiosrv}/bin/gpiosrv";
      WorkingDirectory = "/root";
      Restart = "always";
      Environment = [
        "ROCKET_PORT=3000"
      ];
    };

    caddy = service template {
      ExecStart = "${pkgs.caddy}/bin/caddy run";
      WorkingDirectory = "/root/caddy";
      Restart = "always";
    };

    balena-wifi-connect.serviceConfig = {
      ExecStartPre = "${pkgs.systemd}/bin/systemctl stop caddy.service";
      ExecStartPost = "${pkgs.systemd}/bin/systemctl enable --now caddy.service";
    };
  };
  services.balena-wifi-connect = {
    enable = true;
    portalSsid = "raspi-car";
  };
  networking.networkmanager.connectionConfig = {
    "connection.autoconnect" = true;
  };
  networking.networkmanager.dispatcherScripts = [{
    type = "basic";
    source = pkgs.writeText "restart-mediamtx" ''
      INTERFACE="$1"
      ACTION="$2"

      case "$ACTION" in
          up|down)
              ${pkgs.systemd}/bin/systemctl try-restart mediamtx.service
              ;;
      esac
    '';
  }];
}
