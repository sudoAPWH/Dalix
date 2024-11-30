# Applications

Applications for dalixOS will be custom **AppBundles** which are detailed in this document. An AppBundle has a seperate rootfs of it's own which may cause size issues, but will always work and be reliable. (Providing the developer isn't insane...) We reccomend application developers link with musl, so-as to avoid all of the compatability issues glibc has.

AppBundles are similar to self-extracting archives, but they are more like self-mounting archives as they are data appended to a dalixOS application runtime.
