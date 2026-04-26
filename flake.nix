{
  description = "Build image";
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11";
  inputs.deploy-rs.url = "github:serokell/deploy-rs";

  outputs = { self, nixpkgs, deploy-rs }: 
    let
      sys.current = "x86_64-linux";
      sys.cross = "armv6l-linux";

      my-pkgs = pkgs: {
        balena-wifi-connect = pkgs.callPackage ./balena-wifi/package.nix {};
        gpiosrv = pkgs.callPackage ./pkgs/gpiosrv/package.nix {};
        gst-rtsp = pkgs.callPackage ./pkgs/gst-rtsp.nix {};
        gst-videoparsers = pkgs.callPackage ./pkgs/gst-videoparsers.nix {};
        pigpio = pkgs.callPackage ./pkgs/pigpio.nix { rpiBoardRevision = "0x9000c1"; };
        v4l-setup = pkgs.callPackage ./pkgs/v4l-setup.nix {};
      };
    in rec {
      nixosConfigurations.raspi-car = nixpkgs.lib.nixosSystem {
        specialArgs = { inherit nixpkgs; };
        modules = [
          {
            nixpkgs.hostPlatform.system = sys.cross;
            nixpkgs.buildPlatform.system = sys.current;

            nixpkgs.overlays = [
              (import ./package-overrides.nix)
              (final: prev: my-pkgs final)
              (deploy-rs.overlays.default)
            ];
          }
          ./sd-image-raspberrypi.nix
          ./configuration.nix
          ./services.nix
          ./balena-wifi/module.nix
        ];
      };
      images.rpi = nixosConfigurations.raspi-car.config.system.build.sdImage;
      packages.${sys.current} = {
        default = nixosConfigurations.raspi-car.config.system.build.toplevel;
      } // (my-pkgs nixpkgs.legacyPackages.${sys.current});

      deploy.nodes.raspi-car = {
        hostname = "raspi-car.lan";
        profiles.system = {
          user = "root";
          sshUser = "root";
          path = nixosConfigurations.raspi-car.pkgs.deploy-rs.lib.activate.nixos nixosConfigurations.raspi-car;
        };
      };

      devShells.${sys.current}.default = nixpkgs.legacyPackages.${sys.current}.mkShell {
        packages = [
          deploy-rs.packages.${sys.current}.default
        ];
      };
    };
}

