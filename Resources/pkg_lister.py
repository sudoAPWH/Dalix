"""
Creates dummy packages for packages that are already installed on the system
, but not in the System/Packages directory.
"""

import subprocess as sp
import os
import argparse

if __name__ == "__main__":
	parser = argparse.ArgumentParser(description="Lists packages that are installed on the system in a parsable manner")
	parser.add_argument("-n", "--name", help="Only output name", action="store_true")
	parser.add_argument("-v", "--version", help="Only output version", action="store_true")
	parser.add_argument("-a", "--arch", help="Only output arch", action="store_true")
	args = parser.parse_args()

	pkgs = sp.check_output("apt list --installed", shell=True).decode("utf-8").split("\n")
	for p in pkgs:
		pkg = p.replace("/", " ")
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
		if args.name:
			print(name)
		else:
			print(f"{name} {version} {arch}")