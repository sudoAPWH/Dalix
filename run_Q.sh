#!/bin/bash

qemu-system-x86_64                                                \
-enable-kvm                                                       \
-drive if=pflash,format=raw,unit=0,readonly,file=/usr/share/OVMF/OVMF_CODE.4m.fd \
-drive if=pflash,format=raw,unit=1,file=OVMF_VARS.4m.fd           \
-m 8G                                                             \
-smp 4                                                            \
-drive format=raw,file=disk.img                                   \
-boot d                                                           \
-netdev user,id=net0,net=192.168.0.0/24,dhcpstart=192.168.0.9     \
-device virtio-net-pci,netdev=net0                                \
-vga qxl                                                          \
-device AC97
