SUMMARY = "systemd target for car control services"
LICENSE = "CLOSED"
VERSION = "1.0"

inherit systemd

do_install() {
    install -d ${D}${systemd_system_unitdir}
    cat >${D}${systemd_system_unitdir}/rc-control.target <<EOF
[Unit]
Description=target for car control services
Requires=network.target
After=wifi-provisioning-check.service

[Install]
WantedBy=default.target
EOF

    cat >${D}${systemd_system_unitdir}/v4l-setup.service <<EOF
[Unit]
Description=configure camera
Before=rc-control.target

[Service]
ExecStart=${bindir}/v4l2-ctl -c auto_exposure=1,exposure_time_absolute=1000,image_stabilization=1
Type=oneshot
RemainAfterExit=true

[Install]
WantedBy=rc-control.target
EOF
}

SYSTEMD_SERVICE:${PN} = " \
    rc-control.target \
    v4l-setup.service \
"
