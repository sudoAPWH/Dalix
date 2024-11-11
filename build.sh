#!/bin/bash

# This is the build script for pkg
mkdir -p build/usr/bin
mkdir -p build/DEBIAN
mkdir -p build/System/Packages
mkdir -p build/Users
mkdir -p build/Volumes
mkdir -p build/Applications


cp SysPkgs/pkg/full.py build/usr/bin/pkg
cp control build/DEBIAN/control

dpkg -x Resources/bwrap.deb build

dpkg-deb --build build
mv build.deb dalixos-base.deb
mv dalixos-base.deb Resources/

rm -R build