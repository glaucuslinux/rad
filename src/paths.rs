// All constants are prefixed with `RADULA_`
// The `'static` is not necessary?

// Default `ccache` directories on both Arch and Fedora
pub const RADULA_CCACHE_PATH: &'static str = "/usr/lib/ccache/bin:/usr/lib64/ccache";

// `pkgconf` and `pkg-config` don't respect the provided sysroot (it doesn't get
// automatically prefixed to PATH and LIBDIR)
pub const RADULA_PKG_CONFIG_PATH: &'static str = "/usr/lib/pkgconfig";
pub const RADULA_PKG_CONFIG_LIBDIR: &'static str = "/usr/lib/pkgconfig";
pub const RADULA_PKG_CONFIG_SYSROOT_DIR: &'static str = "/";
pub const RADULA_PKG_CONFIG_SYSTEM_INCLUDE_PATH: &'static str = "/usr/include";
pub const RADULA_PKG_CONFIG_SYSTEM_LIBRARY_PATH: &'static str = "/usr/lib";

// glaucus related directories
pub const RADULA_BACKUP_DIRECTORY: &'static str = "bak";
pub const RADULA_BUILDS_DIRECTORY: &'static str = "bld";
pub const RADULA_CERATA_DIRECTORY: &'static str = "cerata";
pub const RADULA_CROSS_DIRECTORY: &'static str = "cross";
pub const RADULA_LOG_DIRECTORY: &'static str = "log";
pub const RADULA_SOURCES_DIRECTORY: &'static str = "src";
pub const RADULA_TEMPORARY_DIRECTORY: &'static str = "tmp";
pub const RADULA_TOOLCHAIN_DIRECTORY: &'static str = "toolchain";

// Tools available on the host system
pub const RADULA_AUTORECONF: &'static str = "autoreconf";
pub const RADULA_AUTORECONF_FLAGS: &'static str = "-fvis";

// `bash` is only required for building some packages, and isn't actually used
// for running shell scripts (we're using `dash` for that)
pub const RADULA_BASH: &'static str = "bash";

// Both `chmod` and `chown` use the same flags
pub const RADULA_CHMOD: &'static str = "chmod";
pub const RADULA_CHOWN: &'static str = "chown";
pub const RADULA_CHMOD_CHOWN_FLAGS: &'static str = "-Rv";

// Prefer `curl` over `wget`
pub const RADULA_CURL: &'static str = "curl";
pub const RADULA_CURL_FLAGS: &'static str = "-Lo";

// No default flags were included for `git` because there are many
pub const RADULA_GIT: &'static str = "git";

pub const RADULA_LN: &'static str = "ln";
pub const RADULA_LN_FLAGS: &'static str = "-fnsv";

// `make` flags are known as `MAKEFLAGS` (no underscore)
pub const RADULA_MAKE: &'static str = "make";
pub const RADULA_MAKEFLAGS: &'static str = "V=1";

// Full path is being used here to not overlap with the install() function in
// .ceras files
pub const RADULA_MKDIR: &'static str = "/usr/bin/install";
pub const RADULA_MKDIR_FLAGS: &'static str = "-dv";

pub const RADULA_MV: &'static str = "mv";
pub const RADULA_MV_FLAGS: &'static str = "-v";

// We're not going to use the `num_cpus` crate
pub const RADULA_NPROC: &'static str = "nproc";

pub const RADULA_RM: &'static str = "rm";
pub const RADULA_RM_FLAGS: &'static str = "-frv";

pub const RADULA_RSYNC: &'static str = "rsync";
pub const RADULA_RSYNC_FLAGS: &'static str = "-vaHAXSx";

// We're not going to use any hashing crate/library
pub const RADULA_CHECKSUM: &'static str = "sha512sum";
pub const RADULA_CHECKSUM_FLAGS: &'static str = "-c";

// The main shell used for running shell scripts
pub const RADULA_SHELL: &'static str = "dash";
pub const RADULA_SHELL_FLAGS: &'static str = "-c";

pub const RADULA_TAR: &'static str = "tar";

pub const RADULA_UMOUNT: &'static str = "umount";
pub const RADULA_UMOUNT_FLAGS: &'static str = "-fqRv";
