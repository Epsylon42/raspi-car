SUMMARY = "mediamtx service and config"
LICENSE = "CLOSED"
VERSION = "1.0"

inherit systemd

do_install() {
    install -d ${D}${cardatadir}
    cat > ${D}${cardatadir}/mediamtx.yml <<'EOF'
paths:
  cam:
    runOnDemand: gst-launch-1.0 v4l2src ! video/x-h264, width=800, height=600, framerate=15/1 ! h264parse ! rtspclientsink location=rtsp://localhost:$RTSP_PORT/$MTX_PATH
    runOnDemandRestart: yes

rtmp: no
hls: no
srt: no
EOF

    install -d ${D}${systemd_system_unitdir}
    cat >${D}${systemd_system_unitdir}/mediamtx.service <<EOF
[Unit]
Description=mediamtx
After=rc-control.target

[Service]
ExecStart=${bindir}/mediamtx mediamtx.yml
WorkingDirectory=${cardatadir}
Restart=always

[Install]
WantedBy=rc-control.target
EOF
}

SYSTEMD_SERVICE:${PN} = "mediamtx.service"
FILES:${PN} = "${cardatadir}/mediamtx.yml"
