{
  v4l-utils,
  writeShellApplication
}:
writeShellApplication {
  name = "v4l2-setup";
  runtimeInputs = [ v4l-utils ];
  text = ''
    if [ -e /dev/video0 ]; then
      v4l2-ctl -c auto_exposure=1,exposure_time_absolute=1000,image_stabilization=1
    fi
  '';
}
