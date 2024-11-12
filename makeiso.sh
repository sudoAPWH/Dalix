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

chmod +x build.sh
./build.sh

dev="$(sudo losetup -Pf --show disk.img)"

mkfs.fat -F	32 $(dev)p1
mkfs.ext4 $(dev)p2

mkdir mnt
sudo mount $(dev)p2 mnt
sudo mkdir mnt/boot
sudo mount $(dev)p1 mnt/boot

# Now we have the file hiearchy in mnt/

sudo debootstrap unstable mnt http://deb.debian.org/debian/
cp Resources/dalixos-base.deb mnt/root/

sudo arch-chroot mnt <<EOF
apt install ./root/dalixos-base.deb
/sbin/adduser --home /Users/user user <<EOD
1234
1234
User




EOD
/sbin/usermod -aG sudo user
EOF

sudo losetup -d $(dev)