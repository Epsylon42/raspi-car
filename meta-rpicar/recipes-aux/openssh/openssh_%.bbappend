FILESEXTRAPATHS:prepend := "${THISDIR}/${PN}:"
SRC_URI += "file://sshd_config"

do_install:append() {
    sed -i ${D}${sysconfdir}/ssh/sshd_config \
        -e 's|HostKey *|HostKey ${persistdir}|g'
}
