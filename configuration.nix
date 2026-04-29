{ pkgs, lib, ... }:
{
  system.stateVersion = "25.05";
  nix.settings.extra-experimental-features = [ "nix-command" "flakes" ];

  boot.kernelPatches = [ {
    name = "disable-broken-div64";
    patch = null;
    structuredExtraConfig = with lib.kernel; {
      STRICT_DEVMEM = no;
      IO_STRICT_DEVMEM = no;

      PWM_RP1 = no;
      I2C_DESIGNWARE_CORE = no;
      I2C_DESIGNWARE_SLAVE = no;
      I2C_DESIGNWARE_PLATFORM = no;
      I2C_DESIGNWARE_PCI = no;
      VIDEO_RP1_CFE = no;
    };
  } ];

  boot.kernelModules = [ "bcm2835-v4l2" ];

  networking.networkmanager = {
    enable = true;
    plugins = lib.mkForce [];
  };
  security.polkit.enable = lib.mkForce false;

  networking = {
    hostName = "raspi-car";

    interfaces."wlan0".useDHCP = true;
    firewall.enable = false;
  };

  services.openssh.enable = true;
  services.openssh.settings = {
    PermitRootLogin = "yes";
  };
  users.users.root.openssh.authorizedKeys.keys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIM5Pbiym0oPKlOZM3eujd2PdmJ1HSMYVrjVw1ZJv0GFQ"
  ];

  system.activationScripts.configs = let
    copy = "${pkgs.rsync}/bin/rsync -r";
  in {
    deps = [ "users" ];
    text = ''
      mkdir -p /root
      ${copy} ${./data}/ /root
      ${copy} ${pkgs.gpiosrv.config-file} /root/gpiosrv.json
      chmod -R +w /root
    '';
  };

  programs.command-not-found.enable = false;
  environment.systemPackages = with pkgs; [
    file
    htop-vim
    procs

    mediamtx
    caddy
    tmux
    gpiosrv
    v4l-utils
    v4l-setup

    gst_all_1.gstreamer.bin
    gst_all_1.gstreamer.out
    gst_all_1.gst-plugins-base
    gst_all_1.gst-plugins-good
    gst_all_1.gst-plugins-ugly
    # gst_all_1.gst-plugins-rs
    gst-videoparsers
    gst-rtsp
  ];
}
