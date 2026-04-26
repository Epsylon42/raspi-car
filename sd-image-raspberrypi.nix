{ config, lib, pkgs, nixpkgs, ... }:
{
  imports = [
    # ./sd-image.nix
    "${nixpkgs}/nixos/modules/installer/sd-card/sd-image.nix"
  ];

  # Include some utilities that are useful for installing or repairing
  # the system.
  environment.systemPackages = [
    # Some text editors.
    pkgs.vim

    # Some networking tools.
    pkgs.screen

    # Hardware-related tools.
    pkgs.sdparm
    pkgs.hdparm
    pkgs.smartmontools # for diagnosing hard disks
    pkgs.pciutils
    pkgs.usbutils
    pkgs.nvme-cli

    # Some compression/archiver tools.
    pkgs.unzip
    pkgs.zip
  ];

  boot.supportedFilesystems = [ "vfat" ];

  boot.loader.grub.enable = false;
  boot.loader.generic-extlinux-compatible.enable = true;

  boot.consoleLogLevel = lib.mkDefault 7;
  boot.kernelPackages = pkgs.linuxKernel.packages.linux_rpi1;

  networking.modemmanager.enable = false;
  hardware.bluetooth.enable = false;

  sdImage = {
    populateFirmwareCommands =
      let
        configTxt = pkgs.writeText "config.txt" ''
          # u-boot refuses to start (gets stuck at rainbow polygon) without this,
          # at least on Raspberry Pi 0.
          enable_uart=1

          # Prevent the firmware from smashing the framebuffer setup done by the mainline kernel
          # when attempting to show low-voltage or overtemperature warnings.
          avoid_warnings=1

          start_x=1
          gpu_mem=256

          [pi0]
          kernel=u-boot-rpi0.bin

          [pi1]
          kernel=u-boot-rpi1.bin
        '';
      in
      ''
        (cd ${pkgs.raspberrypifw}/share/raspberrypi/boot && cp bootcode.bin fixup*.dat start*.elf *.dtb $NIX_BUILD_TOP/firmware/)
        cp ${pkgs.ubootRaspberryPiZero}/u-boot.bin firmware/u-boot-rpi0.bin
        cp ${pkgs.ubootRaspberryPi}/u-boot.bin firmware/u-boot-rpi1.bin
        cp ${configTxt} firmware/config.txt
      '';
    populateRootCommands = ''
      mkdir -p ./files/boot
      ${config.boot.loader.generic-extlinux-compatible.populateCmd} -c ${config.system.build.toplevel} -d ./files/boot
    '';
  };
}

