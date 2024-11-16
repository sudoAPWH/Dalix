#!/bin/bash
set -eu
sudo rm -Rf Tests/base
sudo debootstrap unstable Tests/base http://deb.debian.org/debian/
sudo rm -Rf Tests/base/boot
sudo arch-chroot Tests/base <<EOF

apt install -y network-manager ifupdown isc-dhcp-client pppoeconf wpasupplicant wireless-tools iw iproute2 
apt install -y iptables nftables iputils-ping iputils-arping iputils-tracepath ethtool mtr nmap
apt install -y tcptrace ntopng dnsutils dlint dnstracer

EOF
cd Tests && sudo tar -Jcvf ../Resources/base.tar.xz base && cd ..
