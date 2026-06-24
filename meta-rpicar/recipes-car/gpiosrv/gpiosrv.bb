SUMMARY = "web server for controlling gpio"
LICENSE = "CLOSED"
VERSION = "1.0"

FILESEXTRAPATHS:append = ":${THISDIR}"
SRC_URI = " \
    file://gpiosrv-1.0 \
    file://gpiosrv.service \
"

inherit cargo cargo-update-recipe-crates pkgconfig systemd
require ${BPN}-crates.inc

DEPENDS = "pigpio"
RDEPENDS:${PN} = "pigpio rccontrol-systemd-target"

do_install:append() {
    install -d ${D}${cardatadir}
    install ${S}/gpiosrv.json ${D}${cardatadir}

    install -d ${D}${systemd_system_unitdir}
    install -m 0644 ${UNPACKDIR}/gpiosrv.service ${D}${systemd_system_unitdir}/gpiosrv.service
    sed -i \
        -e 's|@@bindir@@|${bindir}|g' \
        -e 's|@@workdir@@|${cardatadir}|g' \
        ${D}${systemd_system_unitdir}/gpiosrv.service
}

SYSTEMD_SERVICE:${PN} = "gpiosrv.service"
FILES:${PN} += " \
    ${cardatadir}/gpiosrv.json \
"
