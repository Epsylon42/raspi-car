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
}

SYSTEMD_SERVICE:${PN} = "rc-control.target"
