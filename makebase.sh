#!/bin/bash
set -eu
sudo rm -Rf Tests/base
sudo debootstrap stable Tests/base http://deb.debian.org/debian/
sudo rm -Rf Tests/base/boot
sudo arch-chroot Tests/base <<EOF
apt install -y sudo python3 python3-tomli-w python3-termcolor micro python3-pip bubblewrap
apt install -y network-manager ifupdown isc-dhcp-client pppoeconf wpasupplicant wireless-tools iw iproute2
apt install -y iptables nftables iputils-ping iputils-arping iputils-tracepath ethtool mtr nmap
apt install -y tcptrace ntopng dnsutils dlint dnstracer
apt install -y linux-image-amd64 firmware-linux-free linux-headers-amd64 grub-efi-amd64 arch-install-scripts locales

EOF
cd Tests && sudo tar -Jcvf ../Resources/base.tar.xz base && cd ..
