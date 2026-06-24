DESCRIPTION = "pigpio "
SECTION = "devtools"
LICENSE = "CLOSED"
LIC_FILES_CHKSUM = "file://UNLICENSED"

SRC_URI = "git://github.com/joan2937/pigpio.git;protocol=https;nobranch=1;tag=${PV}"

DEPENDS = "curl glib-2.0 openssl attr gpgme libxml2"

EXTRA_OECMAKE = "-DCMAKE_POLICY_VERSION_MINIMUM=3.10"
CFLAGS:append = " -std=gnu17"

inherit pkgconfig cmake

do_install:append() {
    rm -r ${D}/usr/bin
    rm -r ${D}/usr/man
}

SOLIBS = ".so"
FILES_SOLIBSDEV = ""
