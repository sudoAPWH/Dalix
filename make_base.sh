#!/bin/bash
set -eu
sudo rm -Rf Tests/base
sudo debootstrap unstable Tests/base http://deb.debian.org/debian/
sudo rm -Rf Tests/base/boot
cd Tests && sudo tar -Jcvf ../Resources/base.tar.xz base && cd ..
