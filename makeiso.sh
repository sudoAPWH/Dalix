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



sudo losetup -d $(dev)