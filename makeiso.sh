#!/bin/bash

echo "This script creates disk images for amd64 architecture."
echo "If you need to install for another architecture, then make a github issue for that, and we will look into it."

read -p 'Press [Enter] to continue'

rm -rf disk.img

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

echo "Please enter your sudo password"

sudo mkfs.vfat -F 32 ${dev}p1
sudo mkfs.ext4 ${dev}p2

mkdir mnt
sudo mount ${dev}p2 mnt
sudo mkdir mnt/boot
sudo mount ${dev}p1 mnt/boot

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
passwd <<EOD
1234
1234
EOD

/sbin/usermod -aG sudo user
apt install linux linux-firmware linux-headers
apt install grub-efi-amd64

/sbin/grub-install --target=x86_64-efi --efi-directory=/boot --removable
EOF

sleep 4
sudo umount mnt/*
sudo umount mnt
sudo losetup -d $dev

sudo rm -Rf mnt