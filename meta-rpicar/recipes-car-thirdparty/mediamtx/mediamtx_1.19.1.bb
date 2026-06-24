SUMMARY = "Ready-to-use SRT / WebRTC / RTSP / RTMP / LL-HLS media server and media proxy that allows to read, publish, proxy, record and playback video and audio streams."
DESCRIPTION = "${SUMMARY}"
HOMEPAGE = "https://github.com/bluenviron/mediamtx"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://${UNPACKDIR}/${BP}/LICENSE;md5=77fd2623bd5398430be5ce60489c2e81"

inherit bin_package

SRC_URI = "https://github.com/bluenviron/mediamtx/releases/download/v${PV}/mediamtx_v${PV}_linux_armv6.tar.gz;subdir=${BP}"
SRC_URI[sha256sum] = "7c6a59ea3e2344f0dbae018cae594a52999917c3eb8169b9f7daa9eef9545bc0"
COMPATIBLE_HOST = "arm-.*-linux.*"

do_install() {
    install -d ${D}${bindir}
    install ${S}/mediamtx ${D}${bindir}
}

FILES:${PN} = "${bindir}/mediamtx"
