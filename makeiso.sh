#!/bin/bash
shopt -s extglob
set -eu
sudo -v

echo "This script creates disk images for amd64 architecture."
echo "If you need to install for another architecture, then make a github issue for that, and we will look into it."

read -p 'Press [Enter] to continue'

rm -rf disk.img

# 4GB
dd if=/dev/zero of=disk.img bs=4096 count=1048576 status=progress

handle_error() {
	sleep 4
	sudo umount mnt/* || echo ""
	sudo umount mnt || echo ""
	sleep 2
	sudo losetup -d $dev
	sleep 1
	sudo rm -Rf mnt
	exit 0
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
# Copy base system, and dalixos-base.deb to mnt
sudo rm -Rf base
echo "Extracting base system..."
echo "This may take a while..."
sudo tar -xkf Resources/base.tar.xz
sudo mv base/!(boot) mnt/
sudo mv base/boot/* mnt/boot/
sudo rm -Rf base
ls mnt
sudo cp Resources/dalixos-base.deb mnt/root/dalixos-base.deb


# Preform operations inside chroot
sudo arch-chroot mnt <<EOF
ls root
apt install -y /root/dalixos-base.deb


# ln -sfT / /System/Packages/base/chroot


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
dpkg-reconfigure locales

/sbin/grub-install --target=x86_64-efi --efi-directory=/boot --removable
/sbin/grub-mkconfig -o /boot/grub/grub.cfg
# genfstab / -U >> /etc/fstab
pkg install adduser adwaita-icon-theme apparmor apt-utils apt arch-install-scripts at-spi2-common at-spi2-core base-files base-passwd bash bind9-dnsutils bind9-host bind9-libs binutils-common binutils-x86-64-linux-gnu binutils bsdutils bubblewrap build-essential busybox bzip2 ca-certificates coreutils cpio cpp-14-x86-64-linux-gnu cpp-14 cpp-x86-64-linux-gnu cpp cron-daemon-common cron dalixos-base dash dbus-bin dbus-daemon dbus-session-bus-common dbus-system-bus-common dbus-user-session dbus dconf-gsettings-backend dconf-service debconf-i18n debconf debian-archive-keyring debianutils dhcpcd-base diffutils dirmngr dlint dmidecode dmsetup dns-root-data dnsmasq-base dnstracer dnsutils dpkg-dev dpkg dracut-install e2fsprogs efibootmgr ethtool fakeroot fdisk findutils firmware-ath9k-htc firmware-carl9170 firmware-linux-free fontconfig-config fontconfig fonts-dejavu-core fonts-dejavu-mono fonts-font-awesome fonts-glyphicons-halflings g++-14-x86-64-linux-gnu g++-14 g++-x86-64-linux-gnu g++ gcc-14-base gcc-14-x86-64-linux-gnu gcc-14 gcc-x86-64-linux-gnu gcc gettext-base gnupg-l10n gnupg-utils gnupg gpg-agent gpg-wks-client gpg gpgconf gpgsm gpgv grep grub-common grub-efi-amd64-bin grub-efi-amd64-signed grub-efi-amd64-unsigned grub-efi-amd64 grub2-common gsettings-desktop-schemas gtk-update-icon-cache gzip hicolor-icon-theme hostname ifupdown init-system-helpers init initramfs-tools-core initramfs-tools iproute2 iptables iputils-arping iputils-ping iputils-tracepath isc-dhcp-client isc-dhcp-common iw javascript-common klibc-utils kmod less libacl1 libalgorithm-diff-perl libalgorithm-diff-xs-perl libalgorithm-merge-perl libapparmor1 libapt-pkg6.0t64 libasan8 libassuan9 libatk-bridge2.0-0t64 libatk1.0-0t64 libatomic1 libatspi2.0-0t64 libattr1 libaudit-common libaudit1 libavahi-client3 libavahi-common-data libavahi-common3 libbinutils libblas3 libblkid1 libbluetooth3 libbpf1 libbrotli1 libbsd0 libbz2-1.0 libc-bin libc-dev-bin libc-l10n libc6-dev libc6 libcairo-gobject2 libcairo2 libcap-ng0 libcap2-bin libcap2 libcc1-0 libcloudproviders0 libcolord2 libcom-err2 libcrypt-dev libcrypt1 libctf-nobfd0 libctf0 libcups2t64 libcurl3t64-gnutls libdatrie1 libdav1d7 libdb5.3t64 libdbi1t64 libdbus-1-3 libdconf1 libdebconfclient0 libdeflate0 libdevmapper1.02.1 libdpkg-perl libduktape207 libedit2 libefiboot1t64 libefivar1t64 libelf1t64 libepoxy0 libexpat1-dev libexpat1 libext2fs2t64 libfakeroot libfdisk1 libffi8 libfile-fcntllock-perl libfontconfig1 libfreetype6 libfribidi0 libfstrm0 libfuse3-3 libgcc-14-dev libgcc-s1 libgcrypt20 libgdbm-compat4t64 libgdbm6t64 libgdk-pixbuf-2.0-0 libgdk-pixbuf2.0-bin libgdk-pixbuf2.0-common libglib2.0-0t64 libglib2.0-data libgmp10 libgnutls30t64 libgomp1 libgpg-error0 libgpm2 libgprofng0 libgraphite2-3 libgssapi-krb5-2 libgtk-3-0t64 libgtk-3-bin libgtk-3-common libgudev-1.0-0 libharfbuzz0b libhiredis1.1.0 libhogweed6t64 libhwasan0 libice6 libicu72 libidn2-0 libip4tc2 libip6tc2 libisl23 libitm1 libiw30t64 libjansson4 libjbig0 libjemalloc2 libjim0.83 libjpeg62-turbo libjs-bootstrap libjs-d3 libjs-jquery-form libjs-jquery-metadata libjs-jquery-tablesorter libjs-jquery-ui libjs-jquery libjs-rickshaw libjs-sphinxdoc libjs-underscore libjson-c5 libk5crypto3 libkeyutils1 libklibc libkmod2 libkrb5-3 libkrb5support0 libksba8 liblcms2-2 libldap-2.5-0 libldap-common liblerc4 liblinear4 liblmdb0 liblocale-gettext-perl liblsan0 liblua5.3-0 liblua5.4-0 liblz4-1 liblzf1 liblzma5 libmariadb3 libmaxminddb0 libmbim-glib4 libmbim-proxy libmbim-utils libmd0 libmm-glib0 libmnl0 libmount1 libmpc3 libmpfr6 libncurses6 libncursesw6 libndp0 libndpi4.2t64 libnetfilter-conntrack3 libnettle8t64 libnewt0.52 libnfnetlink0 libnftables1 libnftnl11 libnghttp2-14 libnghttp3-9 libngtcp2-16 libngtcp2-crypto-gnutls8 libnl-3-200 libnl-genl-3-200 libnl-route-3-200 libnm0 libnorm1t64 libnpth0t64 libnsl2 libp11-kit0 libpam-modules-bin libpam-modules libpam-runtime libpam-systemd libpam0g libpango-1.0-0 libpangocairo-1.0-0 libpangoft2-1.0-0 libpcap0.8t64 libpcre2-8-0 libpcsclite1 libperl5.40 libpgm-5.3-0t64 libpixman-1-0 libpng16-16t64 libpolkit-agent-1-0 libpolkit-gobject-1-0 libpopt0 libproc2-0 libprotobuf-c1 libpsl5t64 libpython3-dev libpython3-stdlib libpython3.12-dev libpython3.12-minimal libpython3.12-stdlib libpython3.12t64 libqmi-glib5 libqmi-proxy libqmi-utils libqrtr-glib0 libquadmath0 libreadline8t64 librrd8t64 librsvg2-2 librsvg2-common librtmp1 libsasl2-2 libsasl2-modules-db libsasl2-modules libseccomp2 libselinux1 libsemanage-common libsemanage2 libsepol2 libsframe1 libsharpyuv0 libslang2 libsm6 libsmartcols1 libsodium23 libsqlite3-0 libss2 libssh2-1t64 libssl3t64 libstdc++-14-dev libstdc++6 libsystemd-shared libsystemd0 libtasn1-6 libteamdctl0 libtext-charwidth-perl libtext-iconv-perl libtext-wrapi18n-perl libthai-data libthai0 libtiff6 libtinfo6 libtirpc-common libtirpc3t64 libtsan2 libubsan1 libudev1 libunistring5 liburcu8t64 libusb-1.0-0 libuuid1 libuv1t64 libwayland-client0 libwayland-cursor0 libwayland-egl1 libwebp7 libwireshark-data libx11-6 libx11-data libxau6 libxcb-render0 libxcb-shm0 libxcb1 libxcomposite1 libxcursor1 libxdamage1 libxdmcp6 libxext6 libxfixes3 libxi6 libxinerama1 libxkbcommon0 libxml2 libxmu6 libxmuu1 libxrandr2 libxrender1 libxt6t64 libxtables12 libxtst6 libxxhash0 libzmq5 libzstd1 linux-base linux-headers-6.11.7-amd64 linux-headers-6.11.7-common linux-headers-amd64 linux-image-6.11.7-amd64 linux-image-amd64 linux-kbuild-6.11.7 linux-libc-dev locales login.defs login logrotate logsave make manpages-dev manpages mariadb-common mawk media-types micro modemmanager mokutil mount mtr mysql-common nano ncurses-base ncurses-bin netbase network-manager nftables nmap-common nmap node-html5shiv ntopng-data ntopng openssl-provider-legacy openssl os-prober passwd patch perl-base perl-modules-5.40 perl pinentry-curses polkitd ppp pppoeconf procps publicsuffix python3-dev python3-minimal python3-pip python3-termcolor python3-tomli-w python3-wheel python3.12-dev python3.12-minimal python3.12 python3 readline-common redis-server redis-tools rpcsvc-proto sed sensible-utils sgml-base shared-mime-info shim-helpers-amd64-signed shim-signed-common shim-signed shim-unsigned sudo systemd-sysv systemd sysvinit-utils tar tcpdump tcptrace tzdata ucf udev usb-modeswitch-data usb-modeswitch usr-is-merged util-linux vim-common vim-tiny whiptail wireless-regdb wireless-tools wpasupplicant x11-common xauth xclip xdg-user-dirs xkb-data xml-core xplot-xplot.org xz-utils zlib1g-dev zlib1g zstd
EOF

# Generate fstab
sudo bash -c 'Resources/genfstab.py mnt mnt/boot > mnt/etc/fstab'

# Generate base package
# ln -sfT mnt/System/Packages/base\*\*\*0.1.0 mnt/System/Packages/base*
# mkdir -p 'mnt/System/Packages/base---0.1.0/chroot'
# sudo cp -r mnt/!(System|Users|Volumes|Applications|boot|dev|proc|sys|run) mnt/System/Packages/base\*\*\*0.1.0/chroot/

# sudo arch-chroot mnt <<EOF
# chown -R user:user '/System/Packages/base---0.1.0/chroot'
# EOF

unset nounset
unset errexit
sleep 2
sudo umount mnt/proc
sleep 2
sudo umount mnt/proc

sudo umount mnt/* mnt
# sudo umount mnt || echo ""
sudo losetup -d $dev || echo "Loop device is not around? Ignoring..."
sleep 1
#
sudo rm -Rf mnt || echo "Failiure in removing mnt, see error logs for more details..."
# Clean up afterward
# handle_error()
