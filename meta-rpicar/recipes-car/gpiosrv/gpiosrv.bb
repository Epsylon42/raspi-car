SUMMARY = "web server for controlling gpio"
LICENSE = "CLOSED"
VERSION = "1.0"

FILESEXTRAPATHS:append = ":${THISDIR}"
SRC_URI = " \
    file://gpiosrv-1.0 \
"

inherit cargo cargo-update-recipe-crates pkgconfig systemd
require ${BPN}-crates.inc

DEPENDS = "pigpio"
RDEPENDS:${PN} = "pigpio rccontrol-systemd"

do_install:append() {
    install -d ${D}${cardatadir}
    install ${S}/gpiosrv.json ${D}${cardatadir}

    install -d ${D}${systemd_system_unitdir}
    cat >${D}${systemd_system_unitdir}/gpiosrv.service <<EOF
[Unit]
Description=server for car control via gpio
After=rc-control.target

[Service]
ExecStart=${bindir}/gpiosrv
WorkingDirectory=${cardatadir}
Environment="ROCKET_PORT=3000"

[Install]
WantedBy=rc-control.target
EOF
}

SYSTEMD_SERVICE:${PN} = "gpiosrv.service"
FILES:${PN} += " \
    ${cardatadir}/gpiosrv.json \
"
