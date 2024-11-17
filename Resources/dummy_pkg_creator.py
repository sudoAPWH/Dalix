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

		pkg = name

		if pkg != "":
			# IDK
			pass
		os.system(f"mkdir -p System/Packages/{pkg}/chroot")
		os.system(f"touch System/Packages/{pkg}/pkg-info")
		with open(f"System/Packages/{pkg}/pkg-info", "w") as f:
			f.write("""
[Package]
Name = {0}
Version = 123.123.123
""".format(pkg))