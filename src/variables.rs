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

pub fn radula_behave_teeth_variables(radula_parallel: bool) {
    env::set_var("AUTORECONF", paths::RADULA_AUTORECONF);
    env::set_var("CHMOD", paths::RADULA_CHMOD);
    env::set_var("CHOWN", paths::RADULA_CHOWN);
    env::set_var("LN", paths::RADULA_LN);

    // `make` and its flags
    env::set_var("MAKE", paths::RADULA_MAKE);
    if radula_parallel {
        env::set_var(
            "MAKEFLAGS",
            ["-j", &(num_cpus::get_physical() * 3).to_string(), " V=1"].concat(),
        );
    } else {
        env::set_var("MAKEFLAGS", "-j1 V=1");
    }

    env::set_var("MKDIR", paths::RADULA_MKDIR);
    env::set_var("MV", paths::RADULA_MV);
    env::set_var("RM", paths::RADULA_RM);
    env::set_var("RSYNC", paths::RADULA_RSYNC);
    env::set_var("UMOUNT", paths::RADULA_UMOUNT);
}

pub fn radula_behave_arch_variables(radula_genome: &str) {
    env::set_var("ARCH", radula_genome);
    env::set_var("CARCH", ["--with-gcc-arch=", radula_genome].concat());

    match radula_genome {
        "aarch64" => {
            // CARCH for aarch64 requires a reset to `armv8-a`
            env::set_var("CARCH", "--with-gcc-arch=armv8-a");
        }
        "armv6zk" => {}
        "i686" => {}
        "x86-64" => {}
        _ => {}
    }
}
