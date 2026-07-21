SUMMARY = "misc config"
DESCRIPTION = ""
LICENSE = "MIT"

do_install() {
    install -d ${D}${sysconfdir}/systemd/journald.conf.d
    cat > ${D}${sysconfdir}/systemd/journald.conf.d/persistent-log.conf <<EOF
[Journal]
Storage=persistent
EOF

    install -d ${D}${sysconfdir}/NetworkManager/conf.d
    cat > ${D}${sysconfdir}/NetworkManager/conf.d/autoconnect.conf <<EOF
[connection]
connection.autoconnect=true
EOF
}

FILES:${PN} += "${sysconfdir}/systemd/journald.conf.d/persistent-log.conf"
FILES:${PN} += "${sysconfdir}/NetworkManager/conf.d/autoconnect.conf"
