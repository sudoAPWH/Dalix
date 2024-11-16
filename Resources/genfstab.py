#!/bin/python

import sys, subprocess
# args should have len(3)
assert len(sys.argv) == 3, "Not enough arguments"

UUID1 = subprocess.check_output(f"findmnt {sys.argv[1]} -o UUID", shell=True).decode("utf-8")
UUID2 = subprocess.check_output(f"findmnt {sys.argv[2]} -o UUID", shell=True).decode("utf-8")

UUID1 = UUID1.replace("UUID", "")
UUID2 = UUID2.replace("UUID", "")
UUID1 = UUID1.lstrip().rstrip()
UUID2 = UUID2.lstrip().rstrip()

# root partition
print(f"UUID={UUID1}    /   ext4    rw,relatime 0 1")
# boot partition
print(f"UUID={UUID2}    /boot   vfat    rw,relatime,fmask=0022,dmask=0022,codepage=437,iocharset=ascii,shortname=mixed,utf8,errors=remount-ro   0 2")

print(f"/ /System/Packages/base***0.1.0/chroot none nofail,x-systemd.device-timeout=2 0 0")
