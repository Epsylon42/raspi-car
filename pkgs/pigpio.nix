{
  lib,
  stdenv,
  fetchFromGitHub,
  cmake,
  buildPythonPackage ? null,
  rpiBoardRevision,
}:

let
  mkDerivation =
    if builtins.isNull buildPythonPackage then stdenv.mkDerivation else buildPythonPackage;
in
mkDerivation rec {
  pname = "pigpio";
  version = "79";
  format = if buildPythonPackage == null then null else "setuptools";

  cmakeFlags = [
    # https://github.com/NixOS/nixpkgs/issues/445447
    (lib.cmakeFeature "CMAKE_POLICY_VERSION_MINIMUM" "3.10")
  ];

  src = fetchFromGitHub {
    owner = "joan2937";
    repo = "pigpio";
    tag = "v${version}";
    hash = "sha256-Z+SwUlBbtWtnbjTe0IghR3gIKS43ZziN0amYtmXy7HE=";
  };

  patchPhase = ''
    substituteInPlace pigpio.c \
      --replace-fail 'filp = fopen ("/proc/cpuinfo", "r");' 'filp = NULL; rev = ${rpiBoardRevision};'
  '';

  nativeBuildInputs = [
    cmake
  ];

  meta = {
    description = "C library for the Raspberry which allows control of the General Purpose Input Outputs (GPIO)";
    homepage = "https://github.com/joan2937/pigpio";
    license = lib.licenses.unlicense;
    maintainers = with lib.maintainers; [ doronbehar ];
    platforms = lib.platforms.linux;
  };
}
