use std::env;
use std::path::Path;
use std::process::Command;
use std::string::String;

use super::paths;

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
            [
                "-j",
                // We need to trim the output or parse won't work...
                &(String::from_utf8_lossy(
                    &Command::new(paths::RADULA_NPROC).output().unwrap().stdout,
                )
                .trim()
                .parse::<f32>()
                .unwrap()
                    * 1.5)
                    .to_string(),
                " ",
                paths::RADULA_MAKEFLAGS,
            ]
            .concat(),
        );
    } else {
        env::set_var("MAKEFLAGS", ["-j1", " ", paths::RADULA_MAKEFLAGS].concat());
    }

    env::set_var("MKDIR", paths::RADULA_MKDIR);
    env::set_var("MV", paths::RADULA_MV);
    env::set_var("RM", paths::RADULA_RM);
    env::set_var("RSYNC", paths::RADULA_RSYNC);
    env::set_var("UMOUNT", paths::RADULA_UMOUNT);
}

pub fn radula_behave_arch_variables(radula_genome: &'static str) {
    env::set_var("ARCH", radula_genome);

    env::set_var(
        "BLD",
        String::from_utf8_lossy(
            &Command::new(Path::new(&env::var("CERD").unwrap()).join("binutils/config.guess"))
                .output()
                .unwrap()
                .stdout,
        )
        .trim(),
    );

    env::set_var("CARCH", ["--with-gcc-arch=", radula_genome].concat());
    env::set_var("TGT", [radula_genome, "-glaucus-linux-musl"].concat());

    match radula_genome {
        "aarch64" => {
            env::set_var("CARCH", "--with-gcc-arch=armv8-a");
            env::set_var("FARCH", "-mabi=lp64 -mfix-cortex-a53-835769 -mfix-cortex-a53-843419 -march=armv8-a -mtune=generic");
            env::set_var("GCARCH", "--with-arch=armv8-a --with-abi=lp64 --enable-fix-cortex-a53-835769 --enable-fix-cortex-a53-843419");
            env::set_var("LARCH", "arm64");
            env::set_var("LCARCH", "");
            env::set_var("LIARCH", "arch/arm64/boot/Image");
            env::set_var("MARCH", radula_genome);
        }
        "armv6zk" => {
            env::set_var("FARCH", "-mabi=aapcs-linux -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s -mcpu=arm1176jzf-s -mfpu=vfpv2");
            env::set_var("GCARCH", "--with-arch=armv6zk --with-tune=arm1176jzf-s --with-abi=aapcs-linux --with-fpu=vfpv2 --with-float=hard");
            env::set_var("LARCH", "arm");
            env::set_var("LCARCH", "bcm2835_");
            env::set_var("LIARCH", "arch/arm/boot/zImage");
            env::set_var("MARCH", "arm");
            env::set_var("TGT", [radula_genome, "-glaucus-linux-musleabihf"].concat());
        }
        "i686" => {
            env::set_var("FARCH", "-march=i686 -mtune=generic -mabi=sysv");
            env::set_var("GCARCH", "--with-arch=i686 --with-tune=generic");
            env::set_var("LARCH", "i386");
            env::set_var("LCARCH", "i386_");
            env::set_var("LIARCH", "arch/x86/boot/bzImage");
            env::set_var("MARCH", "i386");
        }
        "x86-64" => {
            env::set_var("FARCH", "-march=x86-64 -mtune=generic -mabi=sysv");
            env::set_var("GCARCH", "--with-arch=x86-64 --with-tune=generic");
            env::set_var("LARCH", "x86_64");
            env::set_var("LCARCH", "x86_64_");
            env::set_var("LIARCH", "arch/x86/boot/bzImage");
            env::set_var("MARCH", "x86_64");
            env::set_var("TGT", "x86_64-glaucus-linux-musl");
        }
        _ => {}
    }
}

pub fn radula_behave_bootstrap_variables() {
    let radula_glaucus_directory = Path::new(&env::current_dir().unwrap()).join("..");
    env::set_var("GLAD", &radula_glaucus_directory);

    env::set_var(
        "BAKD",
        radula_glaucus_directory.join(paths::RADULA_BACKUP_DIRECTORY),
    );
    env::set_var(
        "CERD",
        radula_glaucus_directory.join(paths::RADULA_CERATA_DIRECTORY),
    );
    env::set_var(
        "CRSD",
        radula_glaucus_directory.join(paths::RADULA_CROSS_DIRECTORY),
    );
    env::set_var(
        "LOGD",
        radula_glaucus_directory.join(paths::RADULA_LOG_DIRECTORY),
    );
    env::set_var(
        "SRCD",
        radula_glaucus_directory.join(paths::RADULA_SOURCES_DIRECTORY),
    );
    env::set_var(
        "TMPD",
        radula_glaucus_directory.join(paths::RADULA_TEMPORARY_DIRECTORY),
    );
    env::set_var(
        "TLCD",
        radula_glaucus_directory.join(paths::RADULA_TOOLCHAIN_DIRECTORY),
    );

    env::set_var(
        "PATH",
        Path::new(
            &[
                Path::new(&env::var("TLCD").unwrap())
                    .join("bin")
                    .to_str()
                    .unwrap(),
                ":",
            ]
            .concat(),
        )
        .join(env::var("PATH").unwrap().strip_prefix("/").unwrap()),
    );
}

pub fn radula_behave_toolchain_variables() {
    env::set_var(
        "TLOG",
        Path::new(&env::var("LOGD").unwrap()).join(paths::RADULA_TOOLCHAIN_DIRECTORY),
    );

    env::set_var(
        "TTMP",
        Path::new(&env::var("TMPD").unwrap()).join(paths::RADULA_TOOLCHAIN_DIRECTORY),
    );

    env::set_var(
        "TBLD",
        Path::new(&env::var("TTMP").unwrap()).join(paths::RADULA_BUILDS_DIRECTORY),
    );
    env::set_var(
        "TSRC",
        Path::new(&env::var("TTMP").unwrap()).join(paths::RADULA_SOURCES_DIRECTORY),
    );
}

pub fn radula_behave_cross_variables() {
    env::set_var(
        "XLOG",
        Path::new(&env::var("LOGD").unwrap()).join(paths::RADULA_CROSS_DIRECTORY),
    );

    env::set_var(
        "XTMP",
        Path::new(&env::var("TMPD").unwrap()).join(paths::RADULA_CROSS_DIRECTORY),
    );

    env::set_var(
        "XBLD",
        Path::new(&env::var("XTMP").unwrap()).join(paths::RADULA_BUILDS_DIRECTORY),
    );
    env::set_var(
        "XSRC",
        Path::new(&env::var("XTMP").unwrap()).join(paths::RADULA_SOURCES_DIRECTORY),
    );
}
