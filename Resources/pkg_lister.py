"""
Creates dummy packages for packages that are already installed on the system
, but not in the System/Packages directory.
"""

import subprocess as sp
import sys
import os

if __name__ == "__main__":
	assert len(sys.argv) == 2, "Invalid number of arguments"
	pkgs = sp.check_output("apt list --installed", shell=True).decode("utf-8").split("\n")
	for p in pkgs:
		pkg = p.replace("/", "")
		pkg = pkg.replace(",", " ")
		pkg = pkg.replace("[", "")
		pkg = pkg.replace("]", "")
		pkg = pkg.split(" ")
		name = pkg[0]
		version = pkg[3]
		arch = pkg[4]

		if pkg != "":
			# IDK
			pass
		print(f"{name} {version} {arch}")