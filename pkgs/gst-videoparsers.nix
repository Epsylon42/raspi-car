{
  stdenv,
  fetchurl,
  meson,
  ninja,
  pkg-config,
  python3,
  gst_all_1,
  gobject-introspection,
}:

stdenv.mkDerivation (finalAttrs: {
  pname = "gst-videoparsers";
  version = "1.26.11";

  outputs = [
    "out"
    "dev"
  ];

  src = fetchurl {
    url = "https://gstreamer.freedesktop.org/src/gst-plugins-bad/gst-plugins-bad-${finalAttrs.version}.tar.xz";
    hash = "sha256-EQ+4J5Xw5Wmx4nsSq5aZ01x3YuH/TblTNdasjRRCrz0=";
  };

  nativeBuildInputs = [
    meson
    ninja
    pkg-config
    python3
    gst_all_1.gstreamer # for gst-tester-1.0
    gobject-introspection
  ];

  buildInputs = [
    gst_all_1.gst-plugins-base
  ];

  mesonFlags = [
    "-Dauto_features=disabled"
    "-Dvideoparsers=enabled"
  ];

  postPatch = ''
    patchShebangs \
      scripts/extract-release-date-from-doap-file.py
  '';

  # This package has some `_("string literal")` string formats
  # that trip up clang with format security enabled.
  hardeningDisable = [ "format" ];

  doCheck = false; # fails 20 out of 58 tests, expensive
})
