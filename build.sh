#!/bin/bash

# This is the build script for pkg
mkdir -p build/usr/bin
mkdir -p build/DEBIAN
mkdir -p build/System/Packages
mkdir -p build/Users
mkdir -p build/Volumes
mkdir -p build/Applications

mkdir -p build/System/Packages/base
cat >> build/System/Packages/base/pkg-info <<EOF
InfoType = 1
[Package]
Name = base
Version = 0.1.0
Arch = all
Maintainer = dalixOS Team
Homepage = figsystems.github.io/dalixOS
Description = '''
A package which is included for all packages
'''
Dependencies = ""
EOF


cp SysPkgs/pkg/full.py build/usr/bin/pkg
cp control build/DEBIAN/control

# dpkg -x Resources/bwrap.deb build

dpkg-deb --build build
mv build.deb dalixos-base.deb
mv dalixos-base.deb Resources/

rm -R build