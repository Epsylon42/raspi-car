DESCRIPTION = "Generating the update image for SWUpdate"
LICENSE = "CLOSED"

# local files to be added to the update image
SRC_URI = " \
    file://sw-description \
    "

# images to build before building update image
IMAGE_DEPENDS = "rpicar-image"

# images and files that will be included in the .swu image
SWUPDATE_IMAGES = "rpicar-image"

# the chosen format for the deployable image
SWUPDATE_IMAGES_FSTYPES[rpicar-image] = ".rootfs.ext4.gz"
SWUPDATE_IMAGES_FSTYPES[uImage] = ".bin"

inherit swupdate
