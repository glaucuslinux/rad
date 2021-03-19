// The `'static` is not necessary?

pub const RADULA_CCACHE_PATH: &'static str = "/usr/lib/ccache/bin:/usr/lib64/ccache";

pub const RADULA_PKG_CONFIG_PATH: &'static str = "/usr/lib/pkgconfig";
pub const RADULA_PKG_CONFIG_LIBDIR: &'static str = "/usr/lib/pkgconfig";
pub const RADULA_PKG_CONFIG_SYSROOT_DIR: &'static str = "/";
pub const RADULA_PKG_CONFIG_SYSTEM_INCLUDE_PATH: &'static str = "/usr/include";
pub const RADULA_PKG_CONFIG_SYSTEM_LIBRARY_PATH: &'static str = "/usr/lib";

pub const RADULA_BACKUP_DIRECTORY: &'static str = "bak";
pub const RADULA_BUILDS_DIRECTORY: &'static str = "builds";
pub const RADULA_CERATA_DIRECTORY: &'static str = "cerata";
pub const RADULA_CROSS_DIRECTORY: &'static str = "cross";
pub const RADULA_LOG_DIRECTORY: &'static str = "log";
pub const RADULA_SOURCES_DIRECTORY: &'static str = "src";
pub const RADULA_TEMPORARY_DIRECTORY: &'static str = "tmp";
pub const RADULA_TOOLCHAIN_DIRECTORY: &'static str = "toolchain";

pub const RADULA_AUTORECONF: &'static str = "autoreconf -fvis";
pub const RADULA_CHMOD: &'static str = "chmod -Rv";
pub const RADULA_CHOWN: &'static str = "chown -Rv";
pub const RADULA_LN: &'static str = "ln -fnsv";
pub const RADULA_MAKE: &'static str = "make";
pub const RADULA_MAKEFLAGS: &'static str = "V=1";
pub const RADULA_MKDIR: &'static str = "/usr/bin/install -dv";
pub const RADULA_MV: &'static str = "mv -v";
pub const RADULA_RM: &'static str = "rm -frv";
pub const RADULA_RSYNC: &'static str = "rsync -vaHAXSx";
pub const RADULA_SHELL: &'static str = "dash";
pub const RADULA_UMOUNT: &'static str = "umount -fqRv";
