#!/bin/bash
set -eu
sudo -v

echo "This script creates disk images for amd64 architecture."
echo "If you need to install for another architecture, then make a github issue for that, and we will look into it."

read -p 'Press [Enter] to continue'

rm -rf disk.img

# 4GB
dd if=/dev/zero of=disk.img bs=4096 count=1048576 status=progress

handle_error() {
	sudo umount mnt/* || echo ""
	sudo umount mnt || echo ""
	sudo losetup -d /dev/$dev
	sudo rm -Rf mnt
}

trap handle_error 0

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

# echo "Please enter your sudo password"
# sudo -k
# sudo -v

sudo mkfs.vfat -F 32 ${dev}p1
sudo mkfs.ext4 ${dev}p2

mkdir -p mnt
sudo mount ${dev}p2 mnt
sudo mkdir mnt/boot
sudo mount ${dev}p1 mnt/boot

# Now we have the file hiearchy in mnt/

# sudo debootstrap unstable mnt http://deb.debian.org/debian/
sudo tar -xvf Resources/base.tar.xz
sudo mv base/* mnt/
sudo rm -Rf base
ls mnt
sudo cp Resources/dalixos-base.deb mnt/root/dalixos-base.deb

sudo arch-chroot mnt <<EOF
ls root
apt install -y /root/dalixos-base.deb
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
apt install -y linux-image-amd64 firmware-linux-free linux-headers-amd64
apt install -y grub-efi-amd64
apt install -y network-manager ifupdown isc-dhcp-client pppoeconf wpasupplication wpagui wireless-tools iw iproute2 
apt install -y iptables nftables iputils-ping iputils-arping iputils-tracepath ethtool mtr nmap
apt install -y tcptrace ntopng dnsutils dlint dnstracer

/sbin/grub-install --target=x86_64-efi --efi-directory=/boot --removable
/sbin/grub-mkconfig -o /boot/grub/grub.cfg
EOF

sleep 4
sudo umount mnt/* || echo ""
sudo umount mnt || echo ""
sudo losetup -d $dev || echo ""

sudo rm -Rf mnt
