// Copyright (c) 2018-2021, Firas Khalil Khana
// Distributed under the terms of the ISC License

// * All constants are prefixed with `RADULA_`
// * All constants are ordered alphabetically
// * The `'static` is needed: https://doc.rust-lang.org/std/primitive.str.html

//
// Architectures
//

pub const RADULA_ARCHITECTURE_CERATA: &'static str = "--with-gcc-arch=";
pub const RADULA_ARCHITECTURE_TUPLE_TARGET: &'static str = "-glaucus-linux-musl";

// aarch64
pub const RADULA_ARCHITECTURE_AARCH64: &'static str = "aarch64";
pub const RADULA_ARCHITECTURE_AARCH64_CERATA: &'static str = "armv8-a";
pub const RADULA_ARCHITECTURE_AARCH64_FLAGS: &'static str =
    "-mabi=lp64 -mfix-cortex-a53-835769 -mfix-cortex-a53-843419 -march=armv8-a -mtune=generic";
pub const RADULA_ARCHITECTURE_AARCH64_GCC_CONFIGURATION: &'static str = "--with-arch=armv8-a --with-abi=lp64 --enable-fix-cortex-a53-835769 --enable-fix-cortex-a53-843419";
pub const RADULA_ARCHITECTURE_AARCH64_LINUX: &'static str = "arm64";
pub const RADULA_ARCHITECTURE_AARCH64_LINUX_IMAGE: &'static str = "arch/arm64/boot/Image";

// armv6zk
pub const RADULA_ARCHITECTURE_ARMV6ZK: &'static str = "armv6zk";
pub const RADULA_ARCHITECTURE_ARMV6ZK_FLAGS: &'static str = "-mabi=aapcs-linux -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s -mcpu=arm1176jzf-s -mfpu=vfpv2";
pub const RADULA_ARCHITECTURE_ARMV6ZK_GCC_CONFIGURATION: &'static str = "--with-arch=armv6zk --with-tune=arm1176jzf-s --with-abi=aapcs-linux --with-fpu=vfpv2 --with-float=hard";
pub const RADULA_ARCHITECTURE_ARMV6ZK_LINUX: &'static str = "arm";
pub const RADULA_ARCHITECTURE_ARMV6ZK_LINUX_CONFIGURATION: &'static str = "bcm2835_";
pub const RADULA_ARCHITECTURE_ARMV6ZK_LINUX_IMAGE: &'static str = "arch/arm/boot/zImage";

// i686
pub const RADULA_ARCHITECTURE_I686: &'static str = "i686";
pub const RADULA_ARCHITECTURE_I686_FLAGS: &'static str = "-march=i686 -mtune=generic -mabi=sysv";
pub const RADULA_ARCHITECTURE_I686_GCC_CONFIGURATION: &'static str =
    "--with-arch=i686 --with-tune=generic";
pub const RADULA_ARCHITECTURE_I686_LINUX: &'static str = "i386";
pub const RADULA_ARCHITECTURE_I686_LINUX_CONFIGURATION: &'static str = "i386_";
pub const RADULA_ARCHITECTURE_I686_LINUX_IMAGE: &'static str = "arch/x86/boot/bzImage";

// x86-64
pub const RADULA_ARCHITECTURE_X86_64: &'static str = "x86-64";
pub const RADULA_ARCHITECTURE_X86_64_FLAGS: &'static str =
    "-march=x86-64 -mtune=generic -mabi=sysv";
pub const RADULA_ARCHITECTURE_X86_64_GCC_CONFIGURATION: &'static str =
    "--with-arch=x86-64 --with-tune=generic";
pub const RADULA_ARCHITECTURE_X86_64_LINUX: &'static str = "x86_64";
pub const RADULA_ARCHITECTURE_X86_64_LINUX_CONFIGURATION: &'static str = "x86_64_";

//
// Directories
//

pub const RADULA_DIRECTORY_BACKUPS: &'static str = "bak";
pub const RADULA_DIRECTORY_BUILDS: &'static str = "bld";
pub const RADULA_DIRECTORY_CERATA: &'static str = "cerata";
pub const RADULA_DIRECTORY_CROSS: &'static str = "cross";
pub const RADULA_DIRECTORY_LOGS: &'static str = "log";
pub const RADULA_DIRECTORY_SOURCES: &'static str = "src";
pub const RADULA_DIRECTORY_TEMPORARY: &'static str = "tmp";
pub const RADULA_DIRECTORY_TOOLCHAIN: &'static str = "toolchain";

//
// Environment Variables
//

// Architecture
pub const RADULA_ENVIRONMENT_ARCHITECTURE: &'static str = "ARCH";
pub const RADULA_ENVIRONMENT_ARCHITECTURE_CERATA: &'static str = "CARCH";
pub const RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS: &'static str = "FARCH";
pub const RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION: &'static str = "GCARCH";
pub const RADULA_ENVIRONMENT_ARCHITECTURE_LINUX: &'static str = "LARCH";
pub const RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION: &'static str = "LCARCH";
pub const RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE: &'static str = "LIARCH";
pub const RADULA_ENVIRONMENT_ARCHITECTURE_MUSL: &'static str = "MARCH";

// Directories
pub const RADULA_ENVIRONMENT_DIRECTORY_BACKUPS: &'static str = "BAKD";
pub const RADULA_ENVIRONMENT_DIRECTORY_CERATA: &'static str = "CERD";

pub const RADULA_ENVIRONMENT_DIRECTORY_CROSS: &'static str = "CRSD";
pub const RADULA_ENVIRONMENT_DIRECTORY_CROSS_BUILDS: &'static str = "XBLD";
pub const RADULA_ENVIRONMENT_DIRECTORY_CROSS_LOGS: &'static str = "XLOG";
pub const RADULA_ENVIRONMENT_DIRECTORY_CROSS_SOURCES: &'static str = "XSRC";
pub const RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY: &'static str = "XTMP";

pub const RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS: &'static str = "GLAD";
pub const RADULA_ENVIRONMENT_DIRECTORY_LOGS: &'static str = "LOGD";
pub const RADULA_ENVIRONMENT_DIRECTORY_SOURCES: &'static str = "SRCD";
pub const RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY: &'static str = "TMPD";

pub const RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN: &'static str = "TLCD";
pub const RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_BUILDS: &'static str = "TBLD";
pub const RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_LOGS: &'static str = "TLOG";
pub const RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_SOURCES: &'static str = "TSRC";
pub const RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY: &'static str = "TTMP";

// Paths
pub const RADULA_ENVIRONMENT_PATH: &'static str = "PATH";

// pkg-config
pub const RADULA_ENVIRONMENT_PKG_CONFIG_LIBDIR: &'static str = "PKG_CONFIG_LIBDIR";
pub const RADULA_ENVIRONMENT_PKG_CONFIG_PATH: &'static str = "PKG_CONFIG_PATH";
pub const RADULA_ENVIRONMENT_PKG_CONFIG_SYSROOT_DIR: &'static str = "PKG_CONFIG_SYSROOT_DIR";
pub const RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_INCLUDE_PATH: &'static str =
    "PKG_CONFIG_SYSTEM_INCLUDE_PATH";
pub const RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_LIBRARY_PATH: &'static str =
    "PKG_CONFIG_SYSTEM_LIBRARY_PATH";

// Tuples
pub const RADULA_ENVIRONMENT_TUPLE_BUILD: &'static str = "BLD";
pub const RADULA_ENVIRONMENT_TUPLE_TARGET: &'static str = "TGT";

//
// Help Messages
//

pub const RADULA_HELP: &'static str = "USAGE:
\tradula [ OPTIONS ]

OPTIONS:
\tb, -b, --behave \tPerform any of the following behaviors:
\t                \tbinary, bootstrap, envenomate

\tc, -c, --ceras  \tDisplay ceras information

\th, -h, --help   \tDisplay this help message

\tv, -v, --version\tDisplay current version number";

pub const RADULA_HELP_BEHAVE: &'static str = "USAGE:
\tradula [ b | -b | --behave ] [ OPTIONS ]

OPTIONS:
\tb, bootstrap \tPerform bootstrap behavior
\te, envenomate\tPerform envenomate behavior

\th, -h, --help\tDisplay this help message

\ti, binary    \tPerform binary behavior";

pub const RADULA_HELP_BEHAVE_BINARY: &'static str = "USAGE:
\tradula [ b | -b | --behave ] [ i | binary ] [ OPTIONS ] [ cerata ]

OPTIONS:
\td, decyst    \tRemove binary cerata without preserving their cysts

\th, -h, --help\tDisplay this help message

\ti, install   \tInstall binary cerata (default)
\tr, remove    \tRemove binary cerata while preserving their cyst(s)
\ts, search    \tSearch for binary cerata within the remote repositories
\tu, upgrade   \tUpgrade binary cerata";

pub const RADULA_HELP_BEHAVE_BOOTSTRAP: &'static str = "USAGE:
\tradula [ b | -b | --behave ] [ b | bootstrap ] [ OPTIONS ]

OPTIONS:
\tc, clean     \tClean up while preserving sources and backups
\td, distclean \tClean up everything

\th, -h, --help\tDisplay this help message

\ti, image     \tCreate a .img file of the glaucus system
\tl, list      \tList supported genomes
\tr, require   \tCheck if host has all required packages
\ts, release   \tRelease a compressed tarball of the toolchain
\tt, toolchain \tBootstrap a cross compiler toolchain
\tx, cross     \tBootstrap a cross compiled glaucus system";

pub const RADULA_HELP_BEHAVE_BOOTSTRAP_LIST: &'static str = "GENOMES (ARCHITECTURES):
\taarch64,       arm64,        armv8-a
\tarm,           armv6zk,      bcm2835
\ti386,          i686,         x86
\tx86-64,        x86_64";

pub const RADULA_HELP_BEHAVE_ENVENOMATE: &'static str = "USAGE:
\tradula [ b | -b | --behave ] [ e | envenomate ] [ OPTIONS ] [ cerata ]

OPTIONS:
\td, decyst    \tRemove cerata without preserving their cyst(s)

\th, -h, --help\tDisplay this help message

\ti, install   \tInstall cerata from source (default)
\tr, remove    \tRemove cerata while preserving their cyst(s)
\ts, search    \tSearch for cerata within the cerata directory
\tu, upgrade   \tUpgrade cerata";

pub const RADULA_HELP_CERAS: &'static str = "USAGE:
\tradula [ c | -c | --ceras ] [ OPTIONS ] [ cerata ]

OPTIONS:
\tc, cnt, concentrate, concentrates\tDisplay cerata concentrate(s)

\th, -h, --help                    \tDisplay this help message

\tn, nom, name                     \tDisplay cerata name(s)
\ts, sum, checksum, sha512sum      \tDisplay cerata sha512sum(s)
\tu, url, source                   \tDisplay cerata source(s)
\tv, ver, version                  \tDisplay cerata version(s)
\ty, cys, cyst, cysts              \tDisplay cerata cyst(s)";

pub const RADULA_HELP_VERSION: &'static str = "radula version 3.4.1

Copyright (c) 2018-2021, Firas Khalil Khana
Distributed under the terms of the ISC License";

//
// Paths
//

// Default `ccache` directories on both Arch and Fedora
pub const RADULA_PATH_CCACHE: &'static str = "/usr/lib/ccache/bin:/usr/lib64/ccache";

pub const RADULA_PATH_CONFIG_GUESS: &'static str = "binutils/config.guess";
pub const RADULA_PATH_GLAUCUS_IMAGE: &'static str = "glaucus.img";

// `pkgconf` and `pkg-config` don't respect the provided sysroot (it doesn't get
// automatically prefixed to PATH and LIBDIR)
pub const RADULA_PATH_PKG_CONFIG_LIBDIR_PATH: &'static str = "/usr/lib/pkgconfig";
pub const RADULA_PATH_PKG_CONFIG_SYSROOT_DIR: &'static str = "/";
pub const RADULA_PATH_PKG_CONFIG_SYSTEM_INCLUDE_PATH: &'static str = "/usr/include";
pub const RADULA_PATH_PKG_CONFIG_SYSTEM_LIBRARY_PATH: &'static str = "/usr/lib";

//
// Tools
//

// A tooth is a tool available on the host system
pub const RADULA_TOOTH_AUTORECONF: &'static str = "autoreconf";
pub const RADULA_TOOTH_AUTORECONF_FLAGS: &'static str = "-fvis";

// `bash` is only required for building some packages, and isn't actually used
// for running shell scripts (we're using `dash` for that)
pub const RADULA_TOOTH_BASH: &'static str = "bash";

// We're not going to use any hashing crate/library
pub const RADULA_TOOTH_CHECKSUM: &'static str = "sha512sum";
pub const RADULA_TOOTH_CHECKSUM_FLAGS: &'static str = "-c";

// Both `chmod` and `chown` use the same flags
pub const RADULA_TOOTH_CHMOD: &'static str = "chmod";
pub const RADULA_TOOTH_CHMOD_CHOWN_FLAGS: &'static str = "-Rv";
pub const RADULA_TOOTH_CHOWN: &'static str = "chown";

// Prefer `curl` over `wget`
pub const RADULA_TOOTH_CURL: &'static str = "curl";
pub const RADULA_TOOTH_CURL_FLAGS: &'static str = "-Lo";

// No default flags were included for `git` because there are many
pub const RADULA_TOOTH_GIT: &'static str = "git";

pub const RADULA_TOOTH_LN: &'static str = "ln";
pub const RADULA_TOOTH_LN_FLAGS: &'static str = "-fnsv";

pub const RADULA_TOOTH_MAKE: &'static str = "make";
pub const RADULA_TOOTH_MAKE_FLAGS: &'static str = "V=1";

// Full path is being used here to not overlap with the install() function in
// .ceras files
pub const RADULA_TOOTH_MKDIR: &'static str = "/usr/bin/install";
pub const RADULA_TOOTH_MKDIR_FLAGS: &'static str = "-dv";

pub const RADULA_TOOTH_MV: &'static str = "mv";
pub const RADULA_TOOTH_MV_FLAGS: &'static str = "-v";

// We're not going to use the `num_cpus` crate
pub const RADULA_TOOTH_NPROC: &'static str = "nproc";

pub const RADULA_TOOTH_PATCH: &'static str = "patch";
pub const RADULA_TOOTH_PATCH_FLAGS: &'static str = "-v";

pub const RADULA_TOOTH_RM: &'static str = "rm";
pub const RADULA_TOOTH_RM_FLAGS: &'static str = "-frv";

pub const RADULA_TOOTH_RSYNC: &'static str = "rsync";
pub const RADULA_TOOTH_RSYNC_FLAGS: &'static str = "-vaHAXSx";

// The main shell used for running shell scripts
pub const RADULA_TOOTH_SHELL: &'static str = "dash";
pub const RADULA_TOOTH_SHELL_FLAGS: &'static str = "-c";

pub const RADULA_TOOTH_TAR: &'static str = "tar";

pub const RADULA_TOOTH_UMOUNT: &'static str = "umount";
pub const RADULA_TOOTH_UMOUNT_FLAGS: &'static str = "-fqRv";
