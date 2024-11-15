#!/bin/bash

qemu-system-x86_64                                             \
-enable-kvm                                                    \
-bios /usr/share/ovmf/x64/OVMF.fd                              \
-m 8G                                                          \
-smp 4                                                         \
-drive format=raw,file=disk.img                                \
-boot d                                                        \
-netdev user,id=net0,net=192.168.0.0/24,dhcpstart=192.168.0.9  \
-device virtio-net-pci,netdev=net0                             \
-vga qxl                                                       \
-device AC97
