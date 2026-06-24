SUMMARY = "wifi provisioning service"
DESCRIPTION = ""
LICENSE = "Apache-2.0"
VERSION = "4.11.84"

FILESEXTRAPATHS:append := ":${THISDIR}"

S = "${UNPACKDIR}/wifi-connect"

SRC_URI = " \
    git://github.com/balena-os/wifi-connect;protocol=https;nobranch=1;destsuffix=wifi-connect;name=wifi-connect \
    https://github.com/balena-os/wifi-connect/releases/download/v${VERSION}/wifi-connect-ui.tar.gz;subdir=wifi-connect-ui;name=ui \
    git://github.com/balena-io-modules/network-manager;protocol=https;nobranch=1;destsuffix=network-manager;name=network-manager \
    file://cargo-lock.patch \
    file://wifi-provisioning-check.service \
    file://balena-wifi-connect.service \
"
SRCREV_wifi-connect = "v${VERSION}"
SRCREV_network-manager = "4da2e6a57de16b6ae911f74321f929d78af8b1ba"
SRCREV_FORMAT = "wifi-connect_network-manager"
SRC_URI[wifi-connect.sha256sum] = "e611f9b9ad87c5a3de74de42647fd56e600264a51bcb8319665936567aed0558"
SRC_URI[ui.sha256sum] = "e57a3cec559729516decf892beb1e7f191b23e71b2e13bcd43d36b980034ffbe"
SRC_URI[network-manager.sha256sum] = ""
LIC_FILES_CHKSUM = "file://${S}/LICENSE;md5=3bfd34238ccc26128aef96796a8bbf97"

DEPENDS = "dbus"
RDEPENDS:${PN} = " \
    dbus \
    dnsmasq \
"

inherit cargo cargo-update-recipe-crates pkgconfig systemd
require ${BPN}-crates.inc

BALENA_WIFI_SSID ??= ""
BALENA_WIFI_GATEWAY ??= "192.168.42.1"
export BALENA_WIFI_SSID
export BALENA_WIFI_GATEWAY

do_install[vardeps] += "BALENA_WIFI_SSID"
do_install[vardeps] += "BALENA_WIFI_GATEWAY"

do_install:append() {
    install -d ${D}${datadir}/balena-wifi-connect/
    cp -r ${UNPACKDIR}/wifi-connect-ui/* ${D}${datadir}/balena-wifi-connect/

    install -d ${D}${systemd_system_unitdir}
    install -m 0644 ${UNPACKDIR}/wifi-provisioning-check.service ${D}${systemd_system_unitdir}/wifi-provisioning-check.service
    install -m 0644 ${UNPACKDIR}/balena-wifi-connect.service ${D}${systemd_system_unitdir}/balena-wifi-connect.service
    sed -i \
        -e 's|@sysconfdir@|${sysconfdir}|g' \
        -e 's|@bindir@|${bindir}|g' \
        ${D}${systemd_system_unitdir}/wifi-provisioning-check.service \
        ${D}${systemd_system_unitdir}/balena-wifi-connect.service

    if [ -z "$BALENA_WIFI_SSID" ]; then
        bbfatal "${PN} requires BALENA_WIFI_SSID variable to be set and non-empty"
    fi
    if [ -z "$BALENA_WIFI_GATEWAY" ]; then
        bbfatal "${PN} requires BALENA_WIFI_GATEWAY variable to be set and non-empty"
    fi

    install -d ${D}${sysconfdir}
    echo 'SSID=${BALENA_WIFI_SSID}' >> ${D}${sysconfdir}/balena-wifi-connect.env
    echo 'GATEWAY=${BALENA_WIFI_GATEWAY}' >> ${D}${sysconfdir}/balena-wifi-connect.env
    echo 'UI_DIRECTORY=${datadir}/balena-wifi-connect' >> ${D}${sysconfdir}/balena-wifi-connect.env
    chmod 0644 ${D}${sysconfdir}/balena-wifi-connect.env
}

SYSTEMD_SERVICE:${PN} = " \
    wifi-provisioning-check.service \
    balena-wifi-connect.service \
"
FILES:${PN} += " \
    ${datadir}/balena-wifi-connect \
    ${sysconfdir}/balena-wifi-connect.env \
"
