#!/bin/bash

# This is the build script for pkg
mkdir -p build/usr/bin
mkdir -p build/DEBIAN

cp SysPkgs/pkg/full.py build/usr/bin/pkg
cp control build/DEBIAN/control

dpkg-deb --build build
mv build.deb pkg.deb
mv pkg.deb ../../Resources

rm -R build