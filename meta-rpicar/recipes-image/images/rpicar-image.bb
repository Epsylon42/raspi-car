require recipes-core/images/core-image-minimal.bb

SUMMARY = "rpicar image"
LICENSE = "CLOSED"

WKS_FILE = "${@bb.utils.contains('DISTRO_FEATURES', 'rpicar-ota', \
                                 'sd-card-layout-ab.wks', \
                                 'sd-card-layout-default.wks', \
                                 d)}"

EXTRA_IMAGE_INSTALL:append:df-rpicar-ota = " \
    swupdate \
    u-boot-fw-utils \
"
