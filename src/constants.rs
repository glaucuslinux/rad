// All constants are prefixed with `RADULA_` and ordered alphabetically
// The `'static` is not necessary?

pub const RADULA_DIRECTORY_BACKUP: &'static str = "bak";
pub const RADULA_DIRECTORY_BUILDS: &'static str = "bld";
pub const RADULA_DIRECTORY_CERATA: &'static str = "cerata";

// Default `ccache` directories on both Arch and Fedora
pub const RADULA_DIRECTORY_CCACHE: &'static str = "/usr/lib/ccache/bin:/usr/lib64/ccache";

pub const RADULA_DIRECTORY_CROSS: &'static str = "cross";
pub const RADULA_DIRECTORY_LOG: &'static str = "log";
pub const RADULA_DIRECTORY_SOURCES: &'static str = "src";
pub const RADULA_DIRECTORY_TEMPORARY: &'static str = "tmp";
pub const RADULA_DIRECTORY_TOOLCHAIN: &'static str = "toolchain";

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

// `pkgconf` and `pkg-config` don't respect the provided sysroot (it doesn't get
// automatically prefixed to PATH and LIBDIR)
pub const RADULA_PKG_CONFIG_LIBDIR_PATH: &'static str = "/usr/lib/pkgconfig";
pub const RADULA_PKG_CONFIG_SYSROOT_DIR: &'static str = "/";
pub const RADULA_PKG_CONFIG_SYSTEM_INCLUDE_PATH: &'static str = "/usr/include";
pub const RADULA_PKG_CONFIG_SYSTEM_LIBRARY_PATH: &'static str = "/usr/lib";

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
