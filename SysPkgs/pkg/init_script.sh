#!/bin/bash
# This is the init script for the system

mkdir /System
mkdir /System/Packages
mkdir /Applications
mkdir /Volumes
mkdir /Users

apt install sudo -y

/sbin/adduser user --home /Users/user <<EOF
1234
1234



EOF
/sbin/usermod -aG sudo user

chmod +x /usr/bin/pkg