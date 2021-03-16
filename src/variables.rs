use super::paths;
use std::env;
use std::path::Path;

pub fn radula_behave_ccache_variables() {
    env::set_var(
        "PATH",
        Path::new(&[paths::RADULA_CCACHE_PATH, ":"].concat())
            .join(env::var("PATH").unwrap().strip_prefix("/").unwrap()),
    );
}

pub fn radula_behave_pkg_config_variables() {
    env::set_var("PKG_CONFIG_PATH", paths::RADULA_PKG_CONFIG_PATH);
    env::set_var("PKG_CONFIG_LIBDIR", paths::RADULA_PKG_CONFIG_LIBDIR);
    env::set_var(
        "PKG_CONFIG_SYSROOT_DIR",
        paths::RADULA_PKG_CONFIG_SYSROOT_DIR,
    );

    // These environment variables are only `pkgconf` specific, but setting them
    // won't do us any harm...
    env::set_var(
        "PKG_CONFIG_SYSTEM_INCLUDE_PATH",
        paths::RADULA_PKG_CONFIG_SYSTEM_INCLUDE_PATH,
    );
    env::set_var(
        "PKG_CONFIG_SYSTEM_LIBRARY_PATH",
        paths::RADULA_PKG_CONFIG_SYSTEM_LIBRARY_PATH,
    );
}

pub fn radula_behave_teeth_variables() {
    env::set_var("AUTORECONF", paths::RADULA_AUTORECONF);
    env::set_var("CHMOD", paths::RADULA_CHMOD);
    env::set_var("CHOWN", paths::RADULA_CHOWN);
    env::set_var("LN", paths::RADULA_LN);
    env::set_var("MAKE", paths::RADULA_MAKE);
    // Do an if parallel here then
    env::set_var("MAKEFLAGS", paths::RADULA_MAKEFLAGS_PARALLEL);
    env::set_var("MAKEFLAGS", paths::RADULA_MAKEFLAGS);
    env::set_var("MKDIR", paths::RADULA_MKDIR);
    env::set_var("MV", paths::RADULA_MV);
    env::set_var("RM", paths::RADULA_RM);
    env::set_var("RSYNC", paths::RADULA_RSYNC);
    env::set_var("UMOUNT", paths::RADULA_UMOUNT);
}
