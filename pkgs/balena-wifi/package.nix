{ lib
, rustPlatform
, fetchFromGitHub
, fetchurl
, pkg-config
, dbus
, makeWrapper
, dnsmasq
}:

rustPlatform.buildRustPackage rec {
  pname = "balena-wifi-connect";
  version = "4.11.84";

  src = fetchFromGitHub {
    owner = "balena-os";
    repo = "wifi-connect";
    rev = "v${version}";
    # Replace with the real hash on first build. Nix will print the correct
    # value when the fake one doesn't match.
    hash = "sha256-h8OaLpYxtwqGZsP22f5zOidUDcKBXBO6Y0HQ3eYkFsY=";
  };

  # Balena publishes a pre-built UI bundle on every release, so we grab that
  # instead of pulling in Node/npm just to rebuild the React frontend.
  ui = fetchurl {
    url = "https://github.com/balena-os/wifi-connect/releases/download/v${version}/wifi-connect-ui.tar.gz";
    hash = "sha256-5Xo87FWXKVFt7PiSvrHn8ZGyPnGy4TvNQ9NrmAA0/74=";
  };

  cargoPatches = [
    ./cargo-lock.patch
  ];

  # Hash of the vendored cargo dependency tree. Replace on first build.
  cargoHash = "sha256-6SpTs7twjKXm0pb64fUum6tGOwIguaDGD5bMtnDg6S0=";

  nativeBuildInputs = [
    pkg-config
    makeWrapper
  ];

  buildInputs = [
    dbus # libdbus-1: required by the `dbus` crate at build and runtime
  ];

  # wifi-connect spawns `dnsmasq` as a child process, so it must be reachable
  # at runtime. The UI directory default is resolved relative to CWD upstream,
  # which is not useful for a system install — we pin it via $UI_DIRECTORY.
  postInstall = ''
    mkdir -p $out/share/wifi-connect/ui
    tar -xzf ${ui} -C $out/share/wifi-connect/ui

    wrapProgram $out/bin/wifi-connect \
      --prefix PATH : ${lib.makeBinPath [ dnsmasq ]} \
      --set-default UI_DIRECTORY $out/share/wifi-connect/ui
  '';

  # Upstream has no test suite wired up; skip to avoid spurious failures.
  doCheck = false;

  meta = {
    description = "WiFi provisioning via a captive portal for embedded Linux (NetworkManager-based)";
    longDescription = ''
      WiFi Connect is a utility for dynamically configuring a Linux device's
      WiFi connection. When no network is configured it brings up a temporary
      access point with a captive-portal web UI; the user picks a network and
      enters credentials, and WiFi Connect hands them off to NetworkManager
      and exits. Useful for headless IoT devices that need a first-time setup
      flow similar to consumer smart-home products.

      Runtime requirements on the host: NetworkManager managing the wireless
      interface, and D-Bus access. The binary itself needs to run as root
      (or with CAP_NET_ADMIN) to manipulate the interface.
    '';
    homepage = "https://github.com/balena-os/wifi-connect";
    changelog = "https://github.com/balena-os/wifi-connect/blob/v${version}/CHANGELOG.md";
    license = lib.licenses.asl20; # verify against upstream LICENSE
    platforms = lib.platforms.linux;
    mainProgram = "wifi-connect";
    maintainers = [ ];
  };
}
