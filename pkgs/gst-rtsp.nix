{
  stdenv,
  meson,
  ninja,
  gst_all_1,
  pkg-config,
  cmake,
  gobject-introspection,
  python3
}:
stdenv.mkDerivation rec {
  name = "gst-rtsp";
  version = "1.19.2";

  src = builtins.fetchGit {
    url = "https://github.com/GStreamer/gst-rtsp-server";
    ref = version;
    rev = "0b037e35e7ed3259ca05be748c382bc40e2cdd91";
  };

  nativeBuildInputs = [
    meson
    ninja
    gst_all_1.gstreamer
    pkg-config
    cmake
    gobject-introspection
    python3
  ];

  buildInputs = [
    gst_all_1.gstreamer
    gst_all_1.gst-plugins-base
  ];

  mesonFlags = [
    "-Dtests=disabled"
    "-Dexamples=disabled"
    "-Ddoc=disabled"
  ];

  postPatch = ''
    patchShebangs scripts
  '';
}
