use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::string::String;

use crate::constants::{
    RADULA_ARCHITECTURE_X86_64_FLAGS, RADULA_ARCHITECTURE_X86_64_LINUX,
    RADULA_ENVIRONMENT_ARCHITECTURE_MUSL,
};

use super::constants;

pub fn radula_behave_bootstrap_arch_environment(x: &'static str) {
    env::set_var(constants::RADULA_ENVIRONMENT_ARCHITECTURE, x);

    env::set_var(
        constants::RADULA_ENVIRONMENT_TUPLE_BUILD,
        String::from_utf8_lossy(
            &Command::new(
                Path::new(&env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_CERATA).unwrap())
                    .join("binutils/config.guess"),
            )
            .output()
            .unwrap()
            .stdout,
        )
        .trim(),
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_ARCHITECTURE_CERATA,
        [constants::RADULA_ARCHITECTURE_CERATA, x].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TUPLE_TARGET,
        [x, constants::RADULA_ARCHITECTURE_TUPLE_TARGET].concat(),
    );

    match x {
        constants::RADULA_ARCHITECTURE_AARCH64 => {
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_CERATA,
                [
                    constants::RADULA_ARCHITECTURE_CERATA,
                    constants::RADULA_ARCHITECTURE_AARCH64_CERATA,
                ]
                .concat(),
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS,
                constants::RADULA_ARCHITECTURE_AARCH64_FLAGS,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION,
                constants::RADULA_ARCHITECTURE_AARCH64_GCC_CONFIGURATION,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX,
                constants::RADULA_ARCHITECTURE_AARCH64_LINUX,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION,
                "",
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE,
                constants::RADULA_ARCHITECTURE_AARCH64_LINUX_IMAGE,
            );
            env::set_var(RADULA_ENVIRONMENT_ARCHITECTURE_MUSL, x);
        }
        constants::RADULA_ARCHITECTURE_ARMV6ZK => {
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS,
                constants::RADULA_ARCHITECTURE_ARMV6ZK_FLAGS,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION,
                constants::RADULA_ARCHITECTURE_ARMV6ZK_GCC_CONFIGURATION,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX,
                constants::RADULA_ARCHITECTURE_ARMV6ZK_LINUX,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION,
                constants::RADULA_ARCHITECTURE_ARMV6ZK_LINUX_CONFIGURATION,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE,
                constants::RADULA_ARCHITECTURE_ARMV6ZK_LINUX_IMAGE,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL,
                constants::RADULA_ARCHITECTURE_ARMV6ZK_LINUX,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_TUPLE_TARGET,
                [x, constants::RADULA_ARCHITECTURE_CERATA, "eabihf"].concat(),
            );
        }
        constants::RADULA_ARCHITECTURE_I686 => {
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS,
                constants::RADULA_ARCHITECTURE_I686_FLAGS,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION,
                constants::RADULA_ARCHITECTURE_I686_GCC_CONFIGURATION,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX,
                constants::RADULA_ARCHITECTURE_I686_LINUX,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION,
                constants::RADULA_ARCHITECTURE_I686_LINUX_CONFIGURATION,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE,
                constants::RADULA_ARCHITECTURE_I686_LINUX_IMAGE,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL,
                constants::RADULA_ARCHITECTURE_I686_LINUX,
            );
        }
        constants::RADULA_ARCHITECTURE_X86_64 => {
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS,
                constants::RADULA_ARCHITECTURE_X86_64_FLAGS,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION,
                constants::RADULA_ARCHITECTURE_X86_64_GCC_CONFIGURATION,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX,
                constants::RADULA_ARCHITECTURE_X86_64_LINUX,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION,
                constants::RADULA_ARCHITECTURE_X86_64_LINUX_CONFIGURATION,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE,
                constants::RADULA_ARCHITECTURE_I686_LINUX_IMAGE,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL,
                constants::RADULA_ARCHITECTURE_X86_64_LINUX,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_TUPLE_TARGET,
                [
                    constants::RADULA_ARCHITECTURE_X86_64_LINUX,
                    constants::RADULA_ARCHITECTURE_TUPLE_TARGET,
                ]
                .concat(),
            );
        }
        _ => {}
    }
}

pub fn radula_behave_bootstrap_clean() {
    fs::remove_dir_all(env::var("CRSD").unwrap());
    fs::remove_dir_all(env::var("LOGD").unwrap());
    fs::remove_dir_all(env::var("TBLD").unwrap());
    fs::remove_dir_all(env::var("TLCD").unwrap());
    fs::remove_dir_all(env::var("XBLD").unwrap());
}

pub fn radula_behave_bootstrap_distclean() {
    fs::remove_dir_all(env::var("BAKD").unwrap());

    fs::remove_file(Path::new(&env::var("GLAD").unwrap()).join("glaucus.img"));

    fs::remove_dir_all(env::var("SRCD").unwrap());

    radula_behave_bootstrap_clean();

    // Only remove `$TMPD` completely after `$TBLD` and `$XBLD` are removed
    fs::remove_dir_all(env::var("TMPD").unwrap());
}

pub fn radula_behave_bootstrap_environment() {
    let x = fs::canonicalize("..").unwrap();

    env::set_var("GLAD", &x);

    env::set_var("BAKD", x.join(constants::RADULA_DIRECTORY_BACKUP));
    env::set_var("CERD", x.join(constants::RADULA_DIRECTORY_CERATA));
    env::set_var("CRSD", x.join(constants::RADULA_DIRECTORY_CROSS));
    env::set_var("LOGD", x.join(constants::RADULA_DIRECTORY_LOG));
    env::set_var("SRCD", x.join(constants::RADULA_DIRECTORY_SOURCES));
    env::set_var("TMPD", x.join(constants::RADULA_DIRECTORY_TEMPORARY));
    env::set_var("TLCD", x.join(constants::RADULA_DIRECTORY_TOOLCHAIN));

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

    radula_behave_teeth_environment();
}

pub fn radula_behave_bootstrap_initialize() {
    fs::create_dir_all(env::var("BAKD").unwrap());
    fs::create_dir_all(env::var("SRCD").unwrap());
}

pub fn radula_behave_bootstrap_toolchain() {
    radula_behave_bootstrap_toolchain_environment();
    radula_behave_bootstrap_toolchain_swallow();
    radula_behave_bootstrap_toolchain_prepare();

    radula_behave_bootstrap_toolchain_construct();
}

pub fn radula_behave_bootstrap_toolchain_backup() {
    let radula_behave_bootstrap_backup = |x: &'static str| {
        Command::new(constants::RADULA_TOOTH_RSYNC)
            .args(&[
                constants::RADULA_TOOTH_RSYNC_FLAGS,
                &env::var(x).unwrap(),
                &env::var("BAKD").unwrap(),
                "--delete",
            ])
            .stdout(Stdio::null())
            .spawn();
    };

    radula_behave_bootstrap_backup("CRSD");
    radula_behave_bootstrap_backup("TLCD");
}

pub fn radula_behave_bootstrap_toolchain_construct() {
    let x = "toolchain";

    radula_behave_construct("musl-headers", x);
    radula_behave_construct("binutils", x);
    radula_behave_construct("gcc", x);
    radula_behave_construct("musl", x);
    radula_behave_construct("libgcc", x);
    radula_behave_construct("libstdc++-v3", x);
    radula_behave_construct("libgomp", x);
}

pub fn radula_behave_bootstrap_toolchain_environment() {
    env::set_var(
        "TLOG",
        Path::new(&env::var("LOGD").unwrap()).join(constants::RADULA_DIRECTORY_TOOLCHAIN),
    );

    env::set_var(
        "TTMP",
        Path::new(&env::var("TMPD").unwrap()).join(constants::RADULA_DIRECTORY_TOOLCHAIN),
    );

    env::set_var(
        "TBLD",
        Path::new(&env::var("TTMP").unwrap()).join(constants::RADULA_DIRECTORY_BUILDS),
    );
    env::set_var(
        "TSRC",
        Path::new(&env::var("TTMP").unwrap()).join(constants::RADULA_DIRECTORY_SOURCES),
    );
}

pub fn radula_behave_bootstrap_toolchain_prepare() {
    fs::create_dir_all(env::var("CRSD").unwrap());
    fs::create_dir_all(env::var("TLOG").unwrap());
    fs::create_dir_all(env::var("TBLD").unwrap());
    fs::create_dir_all(env::var("TSRC").unwrap());
    fs::create_dir_all(env::var("TLCD").unwrap());
}

pub fn radula_behave_bootstrap_toolchain_swallow() {
    radula_behave_swallow("musl");
    radula_behave_swallow("binutils");
    radula_behave_swallow("gmp");
    radula_behave_swallow("mpfr");
    radula_behave_swallow("mpc");
    radula_behave_swallow("isl");
    radula_behave_swallow("gcc");
}

pub fn radula_behave_ccache_environment() {
    env::set_var(
        "PATH",
        Path::new(&[constants::RADULA_DIRECTORY_CCACHE, ":"].concat())
            .join(env::var("PATH").unwrap().strip_prefix("/").unwrap()),
    );
}

pub fn radula_behave_construct(x: &'static str, y: &'static str) {
    // We only require `nom` and `ver` from the `ceras` file
    let z: [String; 8] = radula_behave_source(x);

    let radula_behave_construct_stage_function = |w: &'static str| {
        Command::new(constants::RADULA_TOOTH_SHELL)
            .args(&[
                constants::RADULA_TOOTH_SHELL_FLAGS,
                &format!(
                    // `ceras` and `*.ceras` files are only using `nom` and `ver`
                    "nom={} ver={} . {} && {}",
                    z[0],
                    z[1],
                    Path::new(&env::var("CERD").unwrap())
                        .join(x)
                        .join([y, ".ceras"].concat())
                        .to_str()
                        .unwrap(),
                    w
                ),
            ])
            .spawn()
            .unwrap();
    };

    // prepare
    radula_behave_construct_stage_function("prepare");
    // configure
    radula_behave_construct_stage_function("configure");
    // build
    radula_behave_construct_stage_function("build");

    // check (disabled for now)
    //radula_behave_construct_stage_function("check");

    // install
    radula_behave_construct_stage_function("install");
}

pub fn radula_behave_bootstrap_cross_environment() {
    env::set_var(
        "XLOG",
        Path::new(&env::var("LOGD").unwrap()).join(constants::RADULA_DIRECTORY_CROSS),
    );

    env::set_var(
        "XTMP",
        Path::new(&env::var("TMPD").unwrap()).join(constants::RADULA_DIRECTORY_CROSS),
    );

    env::set_var(
        "XBLD",
        Path::new(&env::var("XTMP").unwrap()).join(constants::RADULA_DIRECTORY_BUILDS),
    );
    env::set_var(
        "XSRC",
        Path::new(&env::var("XTMP").unwrap()).join(constants::RADULA_DIRECTORY_SOURCES),
    );
}

pub fn radula_behave_pkg_config_environment() {
    env::set_var(
        "PKG_CONFIG_LIBDIR",
        constants::RADULA_PKG_CONFIG_LIBDIR_PATH,
    );
    env::set_var("PKG_CONFIG_PATH", constants::RADULA_PKG_CONFIG_LIBDIR_PATH);
    env::set_var(
        "PKG_CONFIG_SYSROOT_DIR",
        constants::RADULA_PKG_CONFIG_SYSROOT_DIR,
    );

    // These environment variables are only `pkgconf` specific, but setting them
    // won't do any harm...
    env::set_var(
        "PKG_CONFIG_SYSTEM_INCLUDE_PATH",
        constants::RADULA_PKG_CONFIG_SYSTEM_INCLUDE_PATH,
    );
    env::set_var(
        "PKG_CONFIG_SYSTEM_LIBRARY_PATH",
        constants::RADULA_PKG_CONFIG_SYSTEM_LIBRARY_PATH,
    );
}

// Sources the `ceras` file and returns an array of strings representing the
// variables inside of it
pub fn radula_behave_source(x: &'static str) -> [String; 8] {
    // We can't have a `"".to_string()` copied 8 times, which is why we're using
    // a constant beforehand
    const S: String = String::new();
    let mut y: [String; 8] = [S; 8];

    // Magic
    for (i, j) in String::from_utf8_lossy(
        &Command::new(constants::RADULA_TOOTH_SHELL)
            .args(&[
                constants::RADULA_TOOTH_SHELL_FLAGS,
                &format!(
                    ". {} && echo $nom~~$ver~~$cmt~~$url~~$sum~~$cys~~$cnt",
                    Path::new(&env::var("CERD").unwrap())
                        .join(x)
                        .join("ceras")
                        .to_str()
                        .unwrap(),
                ),
            ])
            .output()
            .unwrap()
            .stdout,
    )
    .trim()
    .split("~~")
    .enumerate()
    {
        y[i] = j.to_owned();
    }

    return y;
}

// Fetches and verifies ceras's source
pub fn radula_behave_swallow(x: &'static str) {
    // Receive the variables from the `ceras` file
    let y: [String; 8] = radula_behave_source(x);

    // Get the path of the source directory
    let z = String::from(
        Path::new(&env::var("SRCD").unwrap())
            .join(&y[0])
            .to_str()
            .unwrap(),
    );

    let w = String::from(
        Path::new(&z)
            .join(Path::new(&y[3]).file_name().unwrap())
            .to_str()
            .unwrap(),
    );

    // verify
    let radula_behave_verify = || {
        write!(
            Command::new(constants::RADULA_TOOTH_CHECKSUM)
                .arg(constants::RADULA_TOOTH_CHECKSUM_FLAGS)
                .stdin(Stdio::piped())
                .spawn()
                .unwrap()
                .stdin
                .unwrap(),
            "{}",
            [&y[4], " ", &w].concat()
        )
        .unwrap();
    };

    if !Path::is_dir(Path::new(&z)) {
        if y[1] == constants::RADULA_TOOTH_GIT {
            // Clone the `git` repo
            Command::new(constants::RADULA_TOOTH_GIT)
                .args(&["clone", &y[3], &z])
                .spawn();
            // Checkout the freshly cloned `git` repo at the specified commit number
            Command::new(constants::RADULA_TOOTH_GIT)
                .args(&["-C", &z, "checkout", &y[2]])
                .spawn();
        } else {
            fs::create_dir_all(&z);

            Command::new(constants::RADULA_TOOTH_CURL)
                .args(&[constants::RADULA_TOOTH_CURL_FLAGS, &w, &y[3]])
                .spawn();

            // verify
            radula_behave_verify();

            Command::new(constants::RADULA_TOOTH_TAR)
                .args(&["xvf", &w, "-C", &z])
                .stdout(Stdio::null())
                .spawn();
        }
    } else if y[1] != constants::RADULA_TOOTH_GIT {
        radula_behave_verify();
    }
}

pub fn radula_behave_teeth_environment() {
    env::set_var(
        "AUTORECONF",
        [
            constants::RADULA_TOOTH_AUTORECONF,
            " ",
            constants::RADULA_TOOTH_AUTORECONF_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        "CHMOD",
        [
            constants::RADULA_TOOTH_CHMOD,
            " ",
            constants::RADULA_TOOTH_CHMOD_CHOWN_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        "CHOWN",
        [
            constants::RADULA_TOOTH_CHOWN,
            " ",
            constants::RADULA_TOOTH_CHMOD_CHOWN_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        "LN",
        [
            constants::RADULA_TOOTH_LN,
            " ",
            constants::RADULA_TOOTH_LN_FLAGS,
        ]
        .concat(),
    );

    // `make` and its flags
    env::set_var("MAKE", constants::RADULA_TOOTH_MAKE);
    env::set_var(
        "MAKEFLAGS",
        [
            "-j",
            // We need to trim the output or parse won't work...
            &(String::from_utf8_lossy(
                &Command::new(constants::RADULA_TOOTH_NPROC)
                    .output()
                    .unwrap()
                    .stdout,
            )
            .trim()
            .parse::<f32>()
            .unwrap()
                * 1.5)
                .to_string(),
            " ",
            constants::RADULA_TOOTH_MAKE_FLAGS,
        ]
        .concat(),
    );

    env::set_var(
        "MKDIR",
        [
            constants::RADULA_TOOTH_MKDIR,
            " ",
            constants::RADULA_TOOTH_MKDIR_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        "MV",
        [
            constants::RADULA_TOOTH_MV,
            " ",
            constants::RADULA_TOOTH_MV_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        "RM",
        [
            constants::RADULA_TOOTH_RM,
            " ",
            constants::RADULA_TOOTH_RM_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        "RSYNC",
        [
            constants::RADULA_TOOTH_RSYNC,
            " ",
            constants::RADULA_TOOTH_RSYNC_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        "UMOUNT",
        [
            constants::RADULA_TOOTH_UMOUNT,
            " ",
            constants::RADULA_TOOTH_UMOUNT_FLAGS,
        ]
        .concat(),
    );
}

pub fn radula_open(x: &'static str) {
    println!("{}\n\n{}", constants::RADULA_HELP_VERSION, x);
}
