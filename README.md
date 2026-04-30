# raspi-car

Raspberry Pi Zero NixOS configuration for an RC car with live video streaming and web-based controls.

## Components

- **gpiosrv** — Rust (Rocket) websocket server that drives motors via GPIO/pigpio
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

## Notes on latency

The current implementation achieves ~300ms of camera-to-screen latency on a good wifi connection.
It is possible to get lower than that but likely not in a browser. FPS is also limited to about 15 - this is likely caused by mediamtx having to repackage stuff between stream formats, which hits the CPU bottleneck.

The theoretical best option is RTP directly from gstreamer to e.g. mpv with low-latency profile. This also allows higher FPS (at least 30, maybe slightly more), which improves visuals. All in all the RTP setup achieved latency of about 150ms. (not included here but would look something like this: `gst-launch-1.0 v4l2src ! video/x-h264, width=800, height=600, framerate=30/1 ! h264parse ! rtph264pay config-interval=1 pt=96 ! udpsink host=... port=...`)
