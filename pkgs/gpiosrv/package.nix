{
  rustPlatform,
  pigpio,
}:
rustPlatform.buildRustPackage {
  pname = "gpiorsv";
  version = "0";
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  buildInputs = [
    pigpio
  ];

  passthru.config-file = ./gpiosrv.json;

  postInstallPhase = ''
    mkdir -p $out/share
    cp ${./gpiosrv.json} $out/share/gpiosrv.json
  '';
}
