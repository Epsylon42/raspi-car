SUMMARY = "reverse proxy server for rccar"
LICENSE = "CLOSED"
VERSION = "1.0"

SRC_URI = " \
    file://caddy \
    file://caddy.service \
"

RDEPENDS:${PN} = "caddy rccontrol-systemd-target"

inherit systemd

do_install() {
    install -d ${D}${cardatadir}/http
    cp -r ${UNPACKDIR}/caddy/* ${D}${cardatadir}/http
    chmod -R 0644 ${D}${cardatadir}/http

    install -d ${D}${systemd_system_unitdir}
    install -m 0644 ${UNPACKDIR}/caddy.service ${D}${systemd_system_unitdir}/caddy.service
    sed -i \
        -e 's|@@bindir@@|${bindir}|g' \
        -e 's|@@workdir@@|${cardatadir}/http|g' \
        ${D}${systemd_system_unitdir}/caddy.service
}

SYSTEMD_SERVICE:${PN} = "caddy.service"
FILES:${PN} = " \
    ${cardatadir}/http \
"
