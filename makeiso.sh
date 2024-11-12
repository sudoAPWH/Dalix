#!/bin/bash

# 4GB
dd if=/dev/zero of=disk.img bs=4096 count=1048576 status=progress

# make neccesary partitions
fdisk disk.img <<EOF
g
n


+512M
t
1

n



w
EOF

dev="$(sudo losetup -Pf --show disk.img)"

mkdir mnt
sudo mount $(dev)p2 mnt
sudo mkdir mnt/boot
sudo mount $(dev)p1 mnt/boot

# Now we have the file hiearchy in mnt/

sudo debootstrap unstable mnt http://deb.debian.org/debian/

sudo losetup -d $(dev)