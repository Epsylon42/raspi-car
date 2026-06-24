SUMMARY = "Fast, multi-platform web server with automatic HTTPS"
LICENSE = "Apache-2.0"
LIC_FILES_CHKSUM = "file://src/${GO_IMPORT}/LICENSE;md5=3b83ef96387f14655fc854ddc3c6bd57"

SRC_URI = "git://github.com/caddyserver/caddy;protocol=https;nobranch=1;destsuffix=${GO_SRCURI_DESTSUFFIX}"
SRCREV = "ffb6ab0644f24c5ee6542aca6bd59b7a1b0a8f91"

GO_IMPORT = "github.com/caddyserver/caddy/v2"
GO_INSTALL = "${GO_IMPORT}/cmd/caddy"
GO_DYNLINK:forcevariable = ""

inherit go-mod go-mod-update-modules
require ${BPN}-go-mods.inc
