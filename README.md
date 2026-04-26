# raspi-car

Raspberry Pi Zero NixOS configuration for an RC car with live video streaming and web-based controls.

## Components

- **gpiosrv** — Rust (Rocket) HTTP server that drives motors via GPIO/pigpio
- **Web UI** — Browser-based control panel with WASD keys and live WebRTC video feed
- **mediamtx** — Video streaming server using GStreamer and the Pi's V4L2 camera
- **Caddy** — Reverse proxy serving the web UI
- **balena-wifi-connect** — Captive portal for Wi-Fi setup

Pin configuration lives in `pkgs/gpiosrv/gpiosrv.json`

## Usage

### Build an SD card image:

Optional: replace ssh key in `configuration.nix` with your own before building

```sh
nix build .#images.rpi
```

Warning: nixos does not have binary cache for armv6 so this will take a long time

### Deploy updates over SSH:

Edit `flake.nix` - `deploy.nodes.raspi-car.hostname` to specify the actual ip address of the car

```sh
deploy
```
