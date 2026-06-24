SUMMARY = "reverse proxy server for rccar"
LICENSE = "CLOSED"
VERSION = "1.0"

SRC_URI = " \
    file://caddy \
"
S = "${UNPACKDIR}"

RDEPENDS:${PN} = "caddy rccontrol-systemd"

inherit systemd

do_install() {
    install -d ${D}${cardatadir}/http
    cp -r ${UNPACKDIR}/caddy/* ${D}${cardatadir}/http
    chmod -R 0644 ${D}${cardatadir}/http

    install -d ${D}${systemd_system_unitdir}
    cat >${D}${systemd_system_unitdir}/caddy.service <<EOF
[Unit]
Description=caddy
After=rc-control.target

[Service]
ExecStart=${bindir}/caddy run
WorkingDirectory=${cardatadir}/http
Restart=always

[Install]
WantedBy=rc-control.target
EOF
}

SYSTEMD_SERVICE:${PN} = "caddy.service"
FILES:${PN} = " \
    ${cardatadir}/http \
"
