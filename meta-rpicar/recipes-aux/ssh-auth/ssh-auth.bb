SUMMARY = "ssh pubkey auth"
DESCRIPTION = ""
LICENSE = "MIT"

SSH_AUTH_PUBKEY ?= ""
export SSH_AUTH_PUBKEY

RDEPENDS:${PN} = "openssh"

do_install[vardeps] += "SSH_AUTH_PUBKEY"

do_install() {
     if [ ! -z "$SSH_AUTH_PUBKEY" ]; then
        install -d ${D}${ROOT_HOME}/.ssh
        echo "${SSH_AUTH_PUBKEY}" >> ${D}${ROOT_HOME}/.ssh/authorized_keys
        chmod 0600 ${D}${ROOT_HOME}/.ssh/authorized_keys
    fi
}

ALLOW_EMPTY:${PN} = "1"
FILES:${PN} += "${ROOT_HOME}/.ssh/authorized_keys"
FILES:${PN} += "${sysconfdir}/ssh/sshd_config"
