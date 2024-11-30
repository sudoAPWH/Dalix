# Applications

Applications for dalixOS will be custom **AppBundles** which are detailed in this document. An AppBundle has a seperate rootfs of it's own which may cause size issues, but will always work and be reliable. (Providing the developer isn't insane...) If you are an application developer, you can opt in to have your application overlayed over the users base system, which would decrease the size of your application significantly, losing some portability.

AppBundles are merely a modification of the AppDir spec, and as such, are AppImages. The basic root of an AppBundle's AppDir is:

[SymbolBank]: # ( │	├──	└── )

```
AppDir
├── AppRun
├── config.toml
├── app.desktop
├── icon.png
└── rootfs
	└── ...
```

We will detail each of these files/directories individually.

## Files
### AppRun
`AppRun` is an auto-generated script that parses configuration options in `config.toml`, bubblewraps the rootfs, and hands off control to the application.

### config.toml
`config.toml` is a toml file (also used in the AppBundle generation) of the format:
```toml
name = "Application Name"
version = "Application Version"
icon = "rootfs/usr/share/app/icon.png"
cmd = "command"

[build]
root = "root directory"
distro = "debian"
build_script = """
apt install something
apt satisfy whatever
"""
```

### app.desktop
`app.desktop` is auto-generated based off of `config.toml`

### icon.png
Basically just a symbolic link to the file specified in the `Icon` field in `config.toml`

### rootfs
`rootfs/` is the applications rootfs with the base distros root image along with all dependencies.