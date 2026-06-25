require recipes-core/images/core-image-minimal.bb

SUMMARY = "rpicar image"
LICENSE = "CLOSED"

WKS_FILE = "${@bb.utils.contains('DISTRO_FEATURES', 'rpicar-ota', \
                                 'sd-card-layout-ab.wks', \
                                 'sd-card-layout-default.wks', \
                                 d)}"

IMAGE_INSTALL:append:df-rpicar-ota = " \
    swupdate \
    u-boot-fw-utils \
"

ROOTFS_POSTPROCESS_COMMAND:append = " \
    ${@bb.utils.contains('DISTRO_FEATURES', 'rpicar-ota', 'seed_persistent;', '', d)} \
"

seed_persistent() {
    seed_path() {
        path=$1
        if [ -e "${IMAGE_ROOTFS}${path}" ]; then
            mkdir -p "${IMAGE_ROOTFS}/persistent$(dirname "${path}")"
            mv "${IMAGE_ROOTFS}${path}" "${IMAGE_ROOTFS}/persistent${path}"
            ln -s "/persistent${path}" "${IMAGE_ROOTFS}${path}"
        else
            bbfatal "${PN} seed_persistent requires ${path} to exist"
        fi
    }

    mkdir -p "${IMAGE_ROOTFS}/persistent"
    seed_path "/etc/NetworkManager/system-connections"
    seed_path "/etc/ssh"
}
