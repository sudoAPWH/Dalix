#!/bin/bash
sudo rm -Rf Tests/base
sudo debootstrap unstable Tests/base http://deb.debian.org/debian/
cd Tests && sudo tar -Jcvf ../Resources/base.tar.xz base && cd ..
