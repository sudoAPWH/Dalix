# MakeISO

The ```makeiso.sh``` script is responsible for creating a bootable ISO of dalixOS. Before running
it, one needs to run ```makebase.sh``` to generate the base debian image that ```makeiso.sh``` builds
ontop of. The sequence of steps that ```makeiso.sh``` should complete are outlined below.

Sequence:
 - Prepare loop devices and mounts to prepare the disk image at ```mnt/``` and 	```mnt/boot```
 - Extract base system into ```mnt/```
 - Install ```dalixos-base``` package into ```mnt/```
 - Add user
 - Set password for root and user to ```1234```
 - Install GRUB
 - Generate ```/etc/fstab```
 - Prepare ```base``` package with all packages in the base system
	- Find all packages to be installed
	- Install them
 - Remove all loop devices and mounts, and other cleanup.