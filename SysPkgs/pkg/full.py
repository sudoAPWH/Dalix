#!/bin/python3

import os
from tempfile import TemporaryDirectory
import tomllib
import tomli_w
# from packaging.version import Version
from collections import namedtuple
import argparse
from shutil import copytree, copyfile
import termcolor
from termcolor import colored
import sys
from collections import Counter
# from semver.version import Version

# Dependency = namedtuple('dep_info_t', ['name', 'comparison', 'version'])

root = ""

GOOD = 0
INFO = 1
WARNING = 2
ERROR = 3

def log(msg: str, level: int = 0):
	if level == GOOD:
		print(f"[{colored('+', 'green')}] {msg}", file=sys.stderr)
	elif level == INFO:
		print(f"[{colored('*', 'blue')}] {msg}", file=sys.stderr)
	elif level == WARNING:
		print(f"[{colored('!', 'yellow')}] {msg}", file=sys.stderr)
	elif level == ERROR:
		print(f"[{colored('X', 'red')}] {msg}", file=sys.stderr)

class Version:
	def __init__(self, version: str):
		self.version = version

	def __lt__(self, other):
		return os.system(f"dpkg --compare-versions {self.version} lt {other.version}") == 0
	def __gt__(self, other):
		return os.system(f"dpkg --compare-versions {self.version} gt {other.version}") == 0
	def __eq__(self, other):
		return os.system(f"dpkg --compare-versions {self.version} eq {other.version}") == 0
	def __ne__(self, other):
		return os.system(f"dpkg --compare-versions {self.version} ne {other.version}") == 0
	def __le__(self, other):
		return os.system(f"dpkg --compare-versions {self.version} le {other.version}") == 0
	def __ge__(self, other):
		return os.system(f"dpkg --compare-versions {self.version} ge {other.version}") == 0

	def __str__(self):
		return self.version

class Package:
	def __init__(self, name: str, version: Version, path: str):
		assert os.path.exists(path), "Package not found!"
		assert os.path.exists(path + "/pkg-info")
		self.name = name
		self.version = version
		self.path = path

	def __eq__(self, other):
		if type(other) != Package:
			return False
		return self.name == other.name and self.version == other.version and self.path == other.path

	def get_info(self) -> dict:
		"""
		Reads and returns package metadata from a pkg-info file.

		This function asserts the existence of a package directory and its pkg-info file,
		then loads the metadata from the pkg-info file using the TOML format and returns
		it as a dictionary.

		:return: A dictionary containing the package metadata.
		:raises AssertionError: If the package directory or pkg-info file does not exist.
		"""
		path = self.path
		assert os.path.exists(path), "Package not found!"
		assert os.path.exists(path + "/pkg-info")
		with open(path + "/pkg-info", "rb") as f:
			info = tomllib.load(f)
		return info

	def get_deps(self) -> list:
		"""
		:returns list(Packages):
		"""
		info = self.get_info()
		if info["Package"]["Dependencies"].rstrip().lstrip() == "":
			return []
		deps2 = info["Package"]["Dependencies"].split(",")
		deps = []
		for dep in deps2:
			if len(dep.split("|")) != 1:
				# We have an OR situation here...
				deps.append(OR([Dependency.parse(dep.lstrip().rstrip()) for dep in dep.split("|")]))
			else:
				deps.append(Dependency.parse(dep))
		pkgs = System.deps_to_pkgs(deps)

		assert type(pkgs) == list
		for pkg in pkgs:
			assert type(pkg) == Package
		return pkgs

	def __repr__(self):
		return f"Pkg({self.name}, {self.version}, {self.path})"

	def __hash__(self):
		return hash(str(self.name + self.version + self.path))

class Dependency:
	def __init__(self, name: str, comparisons: list, versions: list):
		self.name = name
		self.comparisons = comparisons
		self.versions = versions

	def satisfied_by(self, version: Version) -> bool:
		if self.versions == None:
			return True
		if self.comparisons == None:
			return True
		for comparison, needed_ver in zip(self.comparisons, self.versions):
			if comparison == "=":
				if Version(needed_ver) != Version(version):
					return False
			elif comparison == ">=":
				if Version(needed_ver) > Version(version):
					return False
			elif comparison == "<=":
				if Version(needed_ver) < Version(version):
					return False
			elif comparison in [">", ">>"]:
				if Version(needed_ver) >= Version(version):
					return False
			elif comparison in ["<", "<<"]:
				if Version(needed_ver) <= Version(version):
					return False
		return True

	def __repr__(self):
		return f"Dependency(\"{self.name}\", {self.comparisons}, {self.versions})"

	def parse(dep: str):
		"""
		Parses a dependency string and returns a dep_info_t representing the parsed dependency.

		The dep_info_t structure has the following fields:
			- name: The name of the package.
			- comparison: The comparison operator used in the dependency string.
			- version: The version of the package.

		For example, if given the input "pkg1 (>= 2.1.0)", the output will be:
			Dependency("pkg1", ">=", "2.1.0")

		:returns Dependency:
		"""
		assert type(dep) == str
		if "(" in dep:
			name, constraint = dep.split("(", 1)
			name = name.rstrip().lstrip()
			constraint = constraint.strip(" )")
			comparisons = []
			versions = []
			for part in constraint.split(","):
				part = part.strip()
				comparison, version = part.split(" ", 1)
				comparisons.append(comparison)
				versions.append(version)
			return Dependency(name, comparisons, versions)
		else:
			return Dependency(dep.lstrip().rstrip(), None, None)

	def parse_deps(deps: list) -> list:
		"""
		Parses a list of dependencies and returns a
		list of `Dependency` representing the parsed dependencies.

		The dep_info_t structure has the following fields:
			- name: The name of the package.
			- comparison: The comparison operator used in the dependency string.
			- version: The version of the package.

		For example, if given the input ["pkg1 (>= 2.1.0)", "pkg2 (= 3.2.1)", "pkg3 (<= 4.3.2)", "pkg4"],
		the output will be:
		[
			Dependency("pkg1", [">="], ["2.1.0"]),
			Dependency("pkg2", ["="], ["3.2.1"]),
			Dependency("pkg3", ["<="], ["4.3.2"]),
			Dependency("pkg4", None, None)
		]

		:param deps: A list of strings representing dependencies.
		:return list(Dependency):
		"""
		assert type(deps) == list
		for dep in deps:
			assert type(dep) == str
		deps_info = []
		for dep in deps:
			deps_info.append(Dependency.parse(dep))

		assert type(deps_info) == list
		return deps_info

class OR:
	def __init__(self, deps: list):
		self.deps = deps

	def satisfied_by(self, version: Version) -> bool:
		for dep in self.deps:
			if dep.satisfied_by(version):
				return True
		return False

	def __repr__(self):
		return f"OR({self.deps})"

class System:
	def symlink(src: str, dst: str):
		"""
		Creates a symbolic link from src to dst. dst is what gets created.

		Ensures that the directory of dest exists before creating the symlink.
		"""

		assert type(src) == str and type(dst) == str
		os.makedirs(src, exist_ok=True)

		dst_dir = os.path.dirname(dst)
		os.makedirs(dst_dir, exist_ok=True)

		# relative_path = os.path.relpath(source_path, target_dir)
		os.symlink(src, dst, target_is_directory=os.path.isdir(src))
	def get_pkg_list():
		"""
		Yields a list of all packages in the system
		:return list(Package):
		"""
		packages = os.listdir(f"{root}/System/Packages")
		for pkg in packages:
			pkg_name = pkg.split("---")
			if len(pkg_name) != 2:
				log(f"Corrupt package in system! \"{pkg}\" Skipping for now.", WARNING)
				continue
			pkg_ret = Package(
				pkg_name[0],
				Version(pkg_name[1]),
				f"{root}/System/Packages/{pkg}"
			)
			assert type(pkg_ret) == Package
			yield pkg_ret

	def search_pkg_list(name: str, strict=False):
		"""
		Searches for packages in the system package list by name.

		This function iterates over the list of packages installed in the system
		and yields packages that match the given name. If `strict` is True,
		it yields packages with an exact name match. If `strict` is False,
		it yields packages that contain the given name as a substring.

		:param name: The name or substring to search for in package names.
		:param strict: A boolean indicating whether to perform a strict name match.
		:return yield Packages:
		"""
		assert type(name) == str
		global root

		for pkg in System.get_pkg_list():
			if strict:
				if pkg.name == name:
					assert type(pkg) == Package
					yield pkg
			else:
				if name in pkg.name:
					assert type(pkg) == Package
					yield pkg

	def get_pkg(name: str):
		"""
		Returns the newest package matching the given name.

		This function iterates over the list of packages installed in the system
		and returns the newest package that matches the given name. If no packages
		match the given name, it returns None.

		:param name: The name of the package to search for.
		:return Package: The newest package matching the given name, or None if no packages match.
		"""
		assert type(name) == str
		candidates = list(System.search_pkg_list(name))
		if not candidates: return None
		newest = None
		for candidate in candidates:
			if newest == None:
				newest = candidate
			else:
				if newest.version < candidate.version:
					newest = candidate
		assert type(newest) == Package
		return newest

	def list_directory_tree(path: str) -> list:
		"""
		Returns a recursive list of all files and directories in the given path.

		:param path: The path to list the directory tree of.
		:return: A list of all files and directories (recursively) in the given path.
		"""
		assert type(path) == str
		files = []
		for root, dirs, filenames in os.walk(path):
			files.append(root)
			files.extend(os.path.join(root, f) for f in filenames)
		files.remove(path)
		assert type(files) == list
		return files

	def deps_to_pkgs(deps_info: list) -> list:
		"""
		Takes a list of Dependency objects and returns a list of Package objects representing
		the newest version of each dependency that satisfies the dependency's comparison operators.

		:param deps_info: A list of Dependency objects.
		:return list(Package):
		"""
		assert type(deps_info) == list
		for d in deps_info:
			assert type(d) == Dependency or type(d) == OR
		pkg_deps = []

		for dep in deps_info:
			if type(dep) == Dependency:
				pkgs = list(System.search_pkg_list(dep.name, strict=True))
			elif type(dep) == OR:
				pkgs = []
				for d in dep.deps:
					pkgs.extend(list(System.search_pkg_list(d.name, strict=True)))
			if not pkgs:
				raise ValueError(f"Could not find dependency \"{dep}\"")

			best = None
			for pkg in pkgs:
				if best == None and dep.satisfied_by(pkg.version):
					best = pkg
				elif best == None:
					continue
				elif pkg.version > best.version and dep.satisfied_by(pkg.version):
					best = pkg
			if best == None:
				raise ValueError(f"Could not find dependency \"{dep.name}\"")
			pkg_deps.append(best)

		assert type(pkg_deps) == list
		return pkg_deps

	def get_files_and_directories_from_pkgs(deps: list):
		"""
		:param deps: A list of `Package` objects.
		:return list(item_t): A list of `item_t` objects representing the
		files and directories in the given packages.
		"""
		assert type(deps) == list
		for dep in deps:
			assert type(dep) == Package

		class item_t:
			def __init__(self, bwrap_loc, fullpath, pkg, occurences):
				self.bwrap_loc = bwrap_loc
				self.fullpath = fullpath
				self.pkg = pkg
				self.occurences = occurences
			def __repr__(self):
				return f"Item({self.bwrap_loc}, {self.fullpath}, {self.pkg.name}, {self.occurences})"

		# generate trees of all of them, and merge them together
		directories = []
		files = []
		for dep in deps:
			# for each Package...
			dep_root = os.path.join(dep.path, "root")
			dep_contents = System.list_directory_tree(dep_root)
			for dep_item in dep_contents:
				if os.path.isdir(dep_item):
					directories.append(item_t(
						dep_item[len(dep_root):], # removes down the common prefix
						dep_item,
						dep,
						None
					))
				elif os.path.isfile(dep_item):
					files.append(item_t(
						dep_item[len(dep_root):], # removes down the common prefix
						dep_item,
						dep,
						None
					))

		# So now we have a list of all the files and directories that need to be included
		# This list is massive but we must not panic...

		# files, and directories are all of type item_t
		# so we need to convert them to their bwrap location

		directories_bwrap_locations = list([x.bwrap_loc for x in directories])
		files_bwrap_locations = list([x.bwrap_loc for x in files])

		dirs_occ = occurence_count(directories_bwrap_locations)
		files_occ = occurence_count(files_bwrap_locations)

		for i in range(len(directories)):
			directories[i].occurences = dirs_occ[directories[i].bwrap_loc]
			if directories[i].occurences == None:
				log(f"No occurences for {directories[i].bwrap_loc}", WARNING)

		for i in range(len(files)):
			files[i].occurences = files_occ[files[i].bwrap_loc]

		assert type(directories) == list
		assert type(files) == list
		return files, directories

	def fill_dep_tree(pkg, ignore_list=[]) -> list:
		"""
		Returns a list of all dependencies required for the container from a list of all the
		dependenecies that are specified by the caller.

		:param deps: A Package object
		:return list(Package): A list of all dependencies required for the dependency
		"""
		assert type(pkg) == Package
		assert type(ignore_list) == list

		output = []

		if pkg in ignore_list:
			return []
		# Package
		output.append(pkg)

		for child in pkg.get_deps(): # Packages
			if child not in ignore_list:
				output.extend(System.fill_dep_tree(child, ignore_list=output))

		assert type(output) == list
		for out in output:
			assert type(out) == Package
		return output

	def fill_dep_tree_from_list(pkgs: list) -> list:
		"""
		Fills a tree of needed packages from a list of packages.
		Also, includes the base package in this result.
		:param deps: A list of Packages
		:return list(Package):
		"""
		assert type(pkgs) == list
		pkgs.append(System.get_pkg("base"))
		output = []
		for pkg in pkgs:
			output.extend(System.fill_dep_tree(pkg, ignore_list=output))
		assert type(output) == list
		for out in output:
			assert type(out) == Package

		output = list(set(output)) # remove duplicates
		return output

	def mkdir(path: str):
		os.system(f"mkdir -p {path}")

	def rm(path: str):
		os.system(f"rm -rf {path}")

	def cp(src: str, dst: str):
		os.system(f"cp -r {src} {dst}")

	def touch(path: str):
		os.system(f"touch {path}")

class DebianUtils:
	def extract_deb_full(deb: str, out: str) -> str:
		os.system(f"dpkg-deb -R {deb} {out}")
	def get_deb_info(path: str) -> dict:
		"""
		Extracts metadata from a .deb package.

		This function extracts all the metadata from a .deb package, and returns it as a dictionary.

		:raises ValueError: The package's control file is invalid.
		:raises FileNotFoundError: The package does not exist.
		:returns dict:
		"""
		assert os.path.exists(path), "Supplied path does not exist!"
		with TemporaryDirectory() as tmpdir:
			DebianUtils.extract_deb_full(path, tmpdir)
			assert os.path.exists(tmpdir + "/DEBIAN/control"), "No control file found in package!"
			info_list = [] # contains tuples of (key: str, value: str)
			with open(tmpdir + "/DEBIAN/control") as control:
				file = control.read().rstrip()
				entry_list = file.split("\n")
				for entry in entry_list:
					if entry[0] == " " or entry[0] == "\t":
						if info_list:
							info_list[-1] = (info_list[-1][0], info_list[-1][1] + "\n"  + entry)
						else:
							raise ValueError("Invalid control file in package!")
					else:
						entry_split = entry.split(":")
						entry_split[1] = ''.join(entry_split[1:])

						key,value = entry_split[0:2]
						key = key.lstrip().rstrip()
						value = value.lstrip().rstrip()
						info_list.append((key, value))
		info = {}
		for entry_list in info_list:
			info[entry_list[0]] = entry_list[1]
		return info

	def extract_deb(path: str, out: str):
		os.system(f"dpkg-deb -x {path} {out}")

	def install_deb(path: str, fetch_dependencies: bool = True, make_symlinks: bool = False):
		"""
		Installs a .deb package into the system.

		This function will install a .deb package into the system. The package will be extracted and
		installed into a directory under /System/Packages, and the metadata will be extracted and stored
		in a file called pkg-info in the same directory.

		All symlinks will be created to point to the correct location.

		:raises ValueError: The package's control file is invalid.
		:raises FileNotFoundError: The package does not exist.
		:raises AssertionError: The package's path does not end with .deb.
		"""
		assert type(path) == str and type(fetch_dependencies) == bool
		global root

		assert os.path.exists(path), "Supplied path does not exist!"
		assert path.lower()[-4:] == ".deb", "Path does not point to a .deb file!"
		# create pkg directory
		with TemporaryDirectory() as tmpdir:
			# extract pkg
			log(f"Extracting {path}...")
			DebianUtils.extract_deb(path, tmpdir)
			# extract metadata
			info = DebianUtils.get_deb_info(path)
			info = DebianUtils.deb_to_pkg_info(info)
			info["Other"] = {}
			info["Other"]["source"] = "deb"
			# install package

			log(f"Installing {path}...")

			inst_dir = f"{root}/System/Packages/{info['Package']['Name']}---{info['Package']['Version']}"

			if os.path.exists(inst_dir):
				System.rm(inst_dir)
			System.mkdir(inst_dir)

			# create symlinks
			root = inst_dir + "/root"
			if make_symlinks:
				pass # Deprecated behaviour
			# System.symlink(f"{root}/usr/bin", f"{root}/bin")
			# System.symlink(f"{root}/usr/bin", f"{root}/usr/local/bin")
			# System.symlink(f"{root}/usr/sbin", f"{root}/sbin")
			# System.symlink(f"{root}/usr/lib", f"{root}/lib")
			# System.symlink(f"{root}/usr/lib64", f"{root}/lib64")
			# System.symlink(f"{root}/usr/etc", f"{root}/etc")
			# System.symlink(f"{root}/usr/var", f"{root}/var")

			# os.system(f"cp -Ra {tmpdir}/. {inst_dir}/root")
			copytree(
				tmpdir,
				f"{inst_dir}/root",
				symlinks=True,
				dirs_exist_ok=True
			)

			# install config
			System.touch(f"{inst_dir}/pkg-info")
			with open(f"{inst_dir}/pkg-info", "w") as info_f:
				info_f.write(
					tomli_w.dumps(
						info,
						multiline_strings=True
					),
				)
			pkg = Package(info["Package"]["Name"], Version(info["Package"]["Version"]), inst_dir)
			# install dependencies from debians servers.
			if "Dependencies" in info["Package"] and fetch_dependencies:
				DebianUtils.install_deps_for_pkg_from_online(pkg)
			return pkg

	def apt_satisfy(dep_string: str, outdir: str):
		cmd = "apt-get satisfy --download-only"
		os.system(f"{cmd} \"{dep_string}\" -o Dir::Cache::Archives={outdir} -y")

	def install_deps_from_online(dep_string: str):
		"""
		Downloads and installs all dependencies specified in dep_string.

		:param dep_string: A string of dependencies to install
		"""
		assert type(dep_string) == str
		with TemporaryDirectory() as tmpdir:
			DebianUtils.apt_satisfy(dep_string, tmpdir)
			deps = os.listdir(tmpdir)
			for dep in deps:
				if dep[-4:] != ".deb":
					continue
				log(f"Going to install {dep}...")
				try:
					DebianUtils.install_deb(f"{tmpdir}/{dep}", fetch_dependencies=False)
				except Exception as e:
					log(f"Failed to install {dep}! Skipping for now...", WARNING)
					log(e, WARNING)
					continue

	def apt_install(pkg: str, out: str):
		cmd = "apt-get install --download-only --reinstall"
		os.system(f"{cmd} {pkg} -o Dir::Cache::Archives={out} -y")

	def install_pkg_from_online(pkg: str, make_symlinks: bool = True, fetch_deps=True):
		"""
		:param str: A string repersenting the package to install
		"""
		assert type(pkg) == str
		with TemporaryDirectory() as tmpdir:
			DebianUtils.apt_install(pkg, tmpdir)
			pkgs = os.listdir(tmpdir)
			for pkg in pkgs:
				if pkg[-4:] != ".deb":
					continue
				log(f"Going to install {pkg}...")
				try:
					installed_pkg = DebianUtils.install_deb(f"{tmpdir}/{pkg}", fetch_dependencies=False, make_symlinks=make_symlinks)
					deps = installed_pkg.get_info()["Package"]["Dependencies"]
					if deps.rstrip().lstrip() == "":
						continue
					if fetch_deps:
						DebianUtils.install_deps_from_online(deps)
				except Exception as e:
					log(f"Failed to install {pkg}! Skipping for now...", WARNING)
					log(e, WARNING)
					continue
	def install_deps_for_pkg_from_online(pkg: Package):
		"""
		Fetches the dependencies of a package from debian's servers.
		We make apt do the heavy lifting
		:param Package: A package to install dependencies for.
		"""
		assert type(pkg) == Package
		deps = []
		info = pkg.get_info()
		if not "Dependencies" in info["Package"]:
			return
		dep_string = info["Package"]["Dependencies"]
		DebianUtils.install_deps_from_online(dep_string)

	def deb_to_pkg_info(info: dict) -> dict:
		"""
		Convert a debian control file info dict into a dalixOS pkg-info dict.

		This function takes a dict generated by get_deb_info and converts it into a dict
		that can be written to a pkg-info file.

		:raises ValueError: If the control file is invalid
		:returns dict:
		"""
		assert type(info) == dict
		pkg_info = {
			"InfoType": 1,
			"Package": {}
		}
		try:
			pkg_info["Package"]["Name"] = info["Package"]
			pkg_info["Package"]["Version"] = info["Version"]
			pkg_info["Package"]["Arch"] = info["Architecture"]
			pkg_info["Package"]["Maintainer"] = info["Maintainer"]
			pkg_info["Package"]["Description"] = info["Description"]
			if "Depends" in info:
				deps = info["Depends"]
				# deps = deps.replace(",", "\n").replace("(", "").replace(")", "").replace(" ", "")
				pkg_info["Package"]["Dependencies"] = deps
			else:
				pkg_info["Package"]["Dependencies"] = ""
		except KeyError:
			raise ValueError("Invalid debian control file in package!")
		assert type(pkg_info) == dict
		return pkg_info




def occurence_count(l: list) -> dict:
	"""
	Returns a dictionary where the keys are the elements of the list and the values are the
	number of occurances of each element in the list.

	Example:
		>>> occurence_count([1, 1, 2, 3, 4, 4, 5])
		{1: 2, 2: 1, 3: 1, 4: 2, 5: 1}
	"""
	assert type(l) == list
	r = dict(Counter(l))
	assert type(r) == dict
	return r

def generate_bwrap_args(deps: list, cmd: str, overlayfs=True) -> list:
	"""
	Generates a list of bubblewrap arguments for setting up the container environment.

	This function constructs bubblewrap arguments to set up the container with necessary
	overlays and bind mounts. The root of the system is mounted as an overlay for the source,
	and specific directories are bound to their respective locations.

	deps should be passed in a form of
	["pkg1==2.1.0", "pkg2>=3.2.1", "pkg3<=4.3.2"]

	:param deps: A list of dependency strings
	:return: A list of bubblewrap arguments for container setup.
	"""
	assert type(deps) == list
	assert type(cmd) == str
	global root
	args = []
	args.append(f"--new-session")
	# args.append(f"--overlay-src {os.path.join(root, "System/Packages/base---0.1.0/root")}")
	# args.append(f"--tmp-overlay /")
	args.append(f"--bind {root}/System /System")
	args.append(f"--bind {root}/Users /Users")
	args.append(f"--bind {root}/Volumes /Volumes")

	# generate list of packages that need to be included

	# Parse list of dependencies
	deps_parsed = Dependency.parse_deps(deps)

	# Get list of packages from list of dependencies
	deps = System.deps_to_pkgs(deps_parsed)
	deps = System.fill_dep_tree_from_list(deps)

	# item_t = namedtuple('item_t', ['bwrap_loc', 'fullpath', 'pkg', "occurences"])

	if not overlayfs: # Symlink Method
		# Get list of files and directories from list of dependencies
		files, directories = System.get_files_and_directories_from_pkgs(deps)

		# We need to follow the rules defined in Pkgs.md

		# for each directory...
		# if occurence count = 1, symlink
		# if occurence count > 1, create directory
		symlinked = []
		for dir in directories:
			if dir.pkg.name == "base": continue
			if dir.occurences == None:
				log(dir, ERROR)
				continue
			if dir.occurences == 1:
				# Everything is mounted within / so we don't have to worry about root inside the
				# container. But outside of the container we do. therfore, src_path must be to
				# inside /System/Packages not {root}/System/Package.
				for sym in symlinked:
					if dir.bwrap_loc.startswith(sym):
						break
				else:
					src_path = dir.fullpath[len(root):]
					if src_path.lstrip().rstrip() == "":
						log(src_path, ERROR)
					if dir.bwrap_loc.lstrip().rstrip() == "":
						log(dir.bwrap_loc, ERROR)
					args.append(f"--symlink {src_path} {dir.bwrap_loc}")
					symlinked.append(dir.bwrap_loc)
			elif dir.occurences > 1:
				pass
				# args.append(f"--dir {dir.bwrap_loc}")

		# for each file...
		# if occurence count = 1, symlink
		# if occurence count > 1, only the file closest to the main package will be symlinked

		for file in files:
			if file.pkg.name == "base": continue
			if True: # file.occurences == 1:
				# Everything is mounted within / so we don't have to worry about root inside the
				# container. But outside of the container we do. therfore, src_path must be to
				# inside /System/Packages not {root}/System/Package.
				for sym in symlinked:
					if file.bwrap_loc.startswith(sym):
						break
				else:
					src_path = file.fullpath[len(root):]
					args.append(f"--symlink {src_path} {file.bwrap_loc}")
	elif overlayfs: # Overlayfs Method
		for dep in deps:
			args.append(f"--overlay-src {dep.path}")
		args.append(f"--tmp-overlay /")
	args.append(cmd)
	return args



































if __name__ == "__main__":
	parser = argparse.ArgumentParser(description='dalixOS package manager')
	parser.add_argument(
		'-r',
		'--root',
		help='Root directory of the system',
		default=''
	)
	parser.add_argument(
		"command"
	)
	parser.add_argument(
		"args",
		nargs="+"
	)
	parser.add_argument(
		"-p",
		"--output-only",
		help="Only output the command to be run",
		action="store_true"
	)
	parser.add_argument(
		"-n",
		"--no-symlinks",
		help="Don't create FHS symlinks for the package.",
		action="store_true"
	)
	parser.add_argument(
		"-b",
		"--no-deps",
		help="Don't fetch dependencies",
		action="store_true"
	)

	args = parser.parse_args()

	root = args.root

	if args.command == "install":
		if os.getuid() != 0:
			log(f"Attempting to escalate privaleges to install {args.args[0]}...", WARNING)
			sys.exit(os.system(f"sudo {sys.argv[0]} install {args.args[0]}"))
		# for p in args.args:
			# if p.startswith("./"):
				# DebianUtils.install_deb(p, make_symlinks=not args.no_symlinks)
		a = ""
		for arg in args.args:
			a += arg + " "
		DebianUtils.install_pkg_from_online(a, make_symlinks=not args.no_symlinks, fetch_deps= not args.no_deps)
	elif args.command == "test":
		bargs = generate_bwrap_args([
			"neovim",
		])
		for barg in bargs:
			print(barg)

	elif args.command == "run":
		assert len(args.args) >= 2, "Not enough arguments! We need to know what package AND the command!"

		bargs = generate_bwrap_args(
			[
				args.args[0],
			],
			''.join([x+" " for x in args.args[1:]])
		)
		bargs = ''.join([x + " " for x in bargs])
		if args.output_only:
			print(bargs)
			sys.exit(0)
		else:
			sys.exit(os.system(f"bwrap {bargs}"))
