# Applications

Applications for dalixOS will be custom Application bundles called **IcePak**s which are detailed in this document. An IcePak has a seperate rootfs of it's own but is also overlayed overtop of the rootfs. This results in lots of useful byproducts, for instance, Your app depends on the newest version of GLibC? Just add it to the rootfs and it will be used instead of the system installed libc as files in the `upperdir` (i.e. `rootfs/`) take precedence over files in the `rootdir` (i.e. `/`)

IcePaks are just self-extracting archives. The basic root of an IcePak's AppDir is:

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

### app.desktop
You only actually need to set the `Name` field for know.

### icon
The icon for the bundle. Can be a(n) svg, png, or jpg.

### rootfs
`rootfs/` is the applications rootfs with the base distros root image along with all dependencies.
