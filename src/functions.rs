// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::error::Error;
use std::path::Path;
use std::process::{Command, Stdio};
use std::string::String;

use super::ceras;
use super::constants;
use super::construct;
use super::swallow;

use colored::Colorize;
use tokio::fs;

extern crate num_cpus;

//
// Behave Functions
//

pub fn radula_behave_bootstrap_cross_construct() {
    let radula_behave_construct_cross = |x: &'static str| {
        construct::radula_behave_construct(x, constants::RADULA_DIRECTORY_CROSS);
    };

    // Filesystem & Package Management
    radula_behave_construct_cross(constants::RADULA_CERAS_IANA_ETC);
    radula_behave_construct_cross(constants::RADULA_CERAS_HYDROSKELETON);
    radula_behave_construct_cross(constants::RADULA_CERAS_CERATA);
    radula_behave_construct_cross(constants::RADULA_CERAS_RADULA);

    // Headers
    radula_behave_construct_cross(constants::RADULA_CERAS_MUSL_UTILS);
    radula_behave_construct_cross(constants::RADULA_CERAS_LINUX_HEADERS);

    // Init
    radula_behave_construct_cross(constants::RADULA_CERAS_SKALIBS);
    radula_behave_construct_cross(constants::RADULA_CERAS_EXECLINE);
    radula_behave_construct_cross(constants::RADULA_CERAS_S6);
    radula_behave_construct_cross(constants::RADULA_CERAS_UTMPS);

    // Compatibility
    radula_behave_construct_cross(constants::RADULA_CERAS_MUSL_FTS);
    radula_behave_construct_cross(constants::RADULA_CERAS_MUSL_OBSTACK);
    radula_behave_construct_cross(constants::RADULA_CERAS_MUSL_RPMATCH);
    radula_behave_construct_cross(constants::RADULA_CERAS_LIBUCONTEXT);
    radula_behave_construct_cross(constants::RADULA_CERAS_GCOMPAT);

    // i18n & L10n
    radula_behave_construct_cross(constants::RADULA_CERAS_GETTEXT_TINY);
    radula_behave_construct_cross(constants::RADULA_CERAS_MUSL_LOCALES);

    // Permissions
    radula_behave_construct_cross(constants::RADULA_CERAS_ATTR);
    radula_behave_construct_cross(constants::RADULA_CERAS_ACL);
    radula_behave_construct_cross(constants::RADULA_CERAS_SHADOW);
    radula_behave_construct_cross(constants::RADULA_CERAS_LIBRESSL);

    // Userland
    radula_behave_construct_cross(constants::RADULA_CERAS_TOYBOX);
    radula_behave_construct_cross(constants::RADULA_CERAS_BC);
    radula_behave_construct_cross(constants::RADULA_CERAS_DIFFUTILS);
    radula_behave_construct_cross(constants::RADULA_CERAS_FILE);
    radula_behave_construct_cross(constants::RADULA_CERAS_FINDUTILS);
    radula_behave_construct_cross(constants::RADULA_CERAS_GREP);
    radula_behave_construct_cross(constants::RADULA_CERAS_HOSTNAME);
    radula_behave_construct_cross(constants::RADULA_CERAS_SED);
    radula_behave_construct_cross(constants::RADULA_CERAS_WHICH);

    // Development
    radula_behave_construct_cross(constants::RADULA_CERAS_EXPAT);

    // Compression
    radula_behave_construct_cross(constants::RADULA_CERAS_BZIP2);
    radula_behave_construct_cross(constants::RADULA_CERAS_LBZIP2);
    radula_behave_construct_cross(constants::RADULA_CERAS_LBZIP2_UTILS);
    radula_behave_construct_cross(constants::RADULA_CERAS_LZ4);
    radula_behave_construct_cross(constants::RADULA_CERAS_LZLIB);
    radula_behave_construct_cross(constants::RADULA_CERAS_PLZIP);
    radula_behave_construct_cross(constants::RADULA_CERAS_XZ);
    radula_behave_construct_cross(constants::RADULA_CERAS_ZLIB_NG);
    radula_behave_construct_cross(constants::RADULA_CERAS_PIGZ);
    radula_behave_construct_cross(constants::RADULA_CERAS_ZSTD);
    radula_behave_construct_cross(constants::RADULA_CERAS_LIBARCHIVE);

    // Development
    radula_behave_construct_cross(constants::RADULA_CERAS_BINUTILS);
    radula_behave_construct_cross(constants::RADULA_CERAS_GCC);

    // Synchronization
    radula_behave_construct_cross(constants::RADULA_CERAS_RSYNC);

    // Shell
    radula_behave_construct_cross(constants::RADULA_CERAS_NETBSD_CURSES);
    radula_behave_construct_cross(constants::RADULA_CERAS_OKSH);
    radula_behave_construct_cross(constants::RADULA_CERAS_DASH);

    // Editors & Pagers
    radula_behave_construct_cross(constants::RADULA_CERAS_LIBEDIT);
    radula_behave_construct_cross(constants::RADULA_CERAS_PCRE2);
    radula_behave_construct_cross(constants::RADULA_CERAS_LESS);
    radula_behave_construct_cross(constants::RADULA_CERAS_VIM);
    radula_behave_construct_cross(constants::RADULA_CERAS_MANDOC);

    // Userland
    radula_behave_construct_cross(constants::RADULA_CERAS_PLOCATE);

    // Networking
    radula_behave_construct_cross(constants::RADULA_CERAS_LIBCAP);
    radula_behave_construct_cross(constants::RADULA_CERAS_IPROUTE2);
    radula_behave_construct_cross(constants::RADULA_CERAS_IPUTILS);
    radula_behave_construct_cross(constants::RADULA_CERAS_SDHCP);
    radula_behave_construct_cross(constants::RADULA_CERAS_WGET2);

    // Utilities
    radula_behave_construct_cross(constants::RADULA_CERAS_KMOD);
    radula_behave_construct_cross(constants::RADULA_CERAS_EUDEV);
    radula_behave_construct_cross(constants::RADULA_CERAS_PSMISC);
    radula_behave_construct_cross(constants::RADULA_CERAS_PROCPS_NG);
    radula_behave_construct_cross(constants::RADULA_CERAS_UTIL_LINUX);
    radula_behave_construct_cross(constants::RADULA_CERAS_E2FSPROGS);
    radula_behave_construct_cross(constants::RADULA_CERAS_PCIUTILS);
    radula_behave_construct_cross(constants::RADULA_CERAS_HWIDS);

    // Services
    radula_behave_construct_cross(constants::RADULA_CERAS_S6_LINUX_INIT);
    radula_behave_construct_cross(constants::RADULA_CERAS_S6_RC);
    radula_behave_construct_cross(constants::RADULA_CERAS_S6_BOOT_SCRIPTS);

    // Kernel
    radula_behave_construct_cross(constants::RADULA_CERAS_LIBUARGP);
    radula_behave_construct_cross(constants::RADULA_CERAS_LIBELF);
    radula_behave_construct_cross(constants::RADULA_CERAS_LINUX);
}

pub fn radula_behave_bootstrap_cross_environment_directories() -> Result<(), Box<dyn Error>> {
    let path = &Path::new(&env::var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY,
    )?)
    .join(constants::RADULA_DIRECTORY_CROSS);

    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY,
        path,
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY_BUILDS,
        path.join(constants::RADULA_DIRECTORY_BUILDS),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY_SOURCES,
        path.join(constants::RADULA_DIRECTORY_SOURCES),
    );

    // cross log file
    env::set_var(
        constants::RADULA_ENVIRONMENT_FILE_CROSS_LOG,
        [
            Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_LOGS)?)
                .join(constants::RADULA_DIRECTORY_CROSS)
                .to_str()
                .unwrap(),
            ".",
            constants::RADULA_DIRECTORY_LOGS,
        ]
        .concat(),
    );

    Ok(())
}

pub fn radula_behave_bootstrap_cross_environment_teeth() {
    let x = &[
        &env::var(constants::RADULA_ENVIRONMENT_TUPLE_TARGET).unwrap(),
        "-",
    ]
    .concat();

    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_ARCHIVER,
        [x, constants::RADULA_CROSS_ARCHIVER].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_ASSEMBLER,
        [x, constants::RADULA_CROSS_ASSEMBLER].concat(),
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_BUILD_C_COMPILER,
        constants::RADULA_CROSS_C_COMPILER,
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_C_COMPILER,
        [x, constants::RADULA_CROSS_C_COMPILER].concat(),
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_C_COMPILER_LINKER,
        constants::RADULA_CROSS_C_CXX_COMPILER_LINKER,
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_C_PREPROCESSOR,
        [
            x,
            constants::RADULA_CROSS_C_COMPILER,
            " ",
            constants::RADULA_CROSS_C_PREPROCESSOR,
        ]
        .concat(),
    );

    env::set_var(constants::RADULA_ENVIRONMENT_CROSS_COMPILE, x);

    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_CXX_COMPILER,
        [x, constants::RADULA_CROSS_CXX_COMPILER].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_CXX_COMPILER_LINKER,
        constants::RADULA_CROSS_C_CXX_COMPILER_LINKER,
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_HOST_C_COMPILER,
        constants::RADULA_CROSS_C_COMPILER,
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_HOST_CXX_COMPILER,
        constants::RADULA_CROSS_CXX_COMPILER,
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_LINKER,
        [x, constants::RADULA_CROSS_LINKER].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_NAMES,
        [x, constants::RADULA_CROSS_NAMES].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_OBJECT_COPY,
        [x, constants::RADULA_CROSS_OBJECT_COPY].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_OBJECT_DUMP,
        [x, constants::RADULA_CROSS_OBJECT_DUMP].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_RANDOM_ACCESS_LIBRARY,
        [x, constants::RADULA_CROSS_RANDOM_ACCESS_LIBRARY].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_READ_ELF,
        [x, constants::RADULA_CROSS_READ_ELF].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_SIZE,
        [x, constants::RADULA_CROSS_SIZE].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_STRINGS,
        [x, constants::RADULA_CROSS_STRINGS].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CROSS_STRIP,
        [x, constants::RADULA_CROSS_STRIP].concat(),
    );
}

pub async fn radula_behave_bootstrap_cross_prepare() -> Result<(), Box<dyn Error>> {
    radula_behave_rsync(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS).unwrap())
            .join(constants::RADULA_DIRECTORY_CROSS)
            .to_str()
            .unwrap(),
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS).unwrap(),
    );
    radula_behave_rsync(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS).unwrap())
            .join(constants::RADULA_DIRECTORY_TOOLCHAIN)
            .to_str()
            .unwrap(),
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS).unwrap(),
    );

    fs::remove_dir_all(
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY_BUILDS).unwrap(),
    )
    .await?;
    fs::create_dir(
        env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY_BUILDS).unwrap(),
    )
    .await?;

    // Create the `src` directory if it doesn't exist, but don't remove it if it does exist!
    fs::create_dir(
        env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY_SOURCES).unwrap(),
    )
    .await?;

    // Create the `log` directory if it doesn't exist, but don't remove it if it does exist!
    fs::create_dir(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_LOGS).unwrap()).await?;

    // Remove cross log file if it exists
    fs::remove_file(env::var(constants::RADULA_ENVIRONMENT_FILE_CROSS_LOG).unwrap()).await?;

    Ok(())
}

// This function is not a mess anymore but it still breaks some libraries
fn radula_behave_bootstrap_cross_strip() {
    Command::new(constants::RADULA_TOOTH_FIND)
        .args(&[
            Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS).unwrap())
                .join(constants::RADULA_PATH_ETC)
                .to_str()
                .unwrap(),
            "-type",
            "d",
            "-empty",
            "-delete",
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    let x = &String::from(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS).unwrap())
            .join(constants::RADULA_PATH_USR)
            .to_str()
            .unwrap(),
    );

    Command::new(constants::RADULA_TOOTH_SHELL)
        .args(&[
            constants::RADULA_TOOTH_SHELL_FLAGS,
            &[
                constants::RADULA_TOOTH_FIND,
                " ",
                x,
                " -name *.a -type f -exec ",
                &[constants::RADULA_CROSS_STRIP, " -gv {} \\;"].concat(),
            ]
            .concat(),
        ])
        .stdout(Stdio::null())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    Command::new(constants::RADULA_TOOTH_SHELL)
        .args(&[
            constants::RADULA_TOOTH_SHELL_FLAGS,
            &[
                constants::RADULA_TOOTH_FIND,
                " ",
                x,
                " \\( -name *.so* -a ! -name *dbg \\) -type f -exec ",
                &[constants::RADULA_CROSS_STRIP, " --strip-unneeded -v {} \\;"].concat(),
            ]
            .concat(),
        ])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    Command::new(constants::RADULA_TOOTH_SHELL)
        .args(&[
            constants::RADULA_TOOTH_SHELL_FLAGS,
            &[
                constants::RADULA_TOOTH_FIND,
                " ",
                x,
                " -type f -exec ",
                &[constants::RADULA_CROSS_STRIP, " -sv {} \\;"].concat(),
            ]
            .concat(),
        ])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    Command::new(constants::RADULA_TOOTH_FIND)
        .args(&[x, "-name", "*.la", "-delete"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

pub async fn radula_behave_bootstrap_environment() -> Result<(), Box<dyn Error>> {
    let path = &fs::canonicalize("..").await?;

    env::set_var(constants::RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS, path);

    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS,
        path.join(constants::RADULA_DIRECTORY_BACKUPS),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_CERATA,
        path.join(constants::RADULA_DIRECTORY_CERATA),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS,
        path.join(constants::RADULA_DIRECTORY_CROSS),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_LOGS,
        path.join(constants::RADULA_DIRECTORY_LOGS),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_SOURCES,
        path.join(constants::RADULA_DIRECTORY_SOURCES),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY,
        path.join(constants::RADULA_DIRECTORY_TEMPORARY),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN,
        path.join(constants::RADULA_DIRECTORY_TOOLCHAIN),
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_PATH,
        Path::new(
            &[
                Path::new(&env::var(
                    constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN,
                )?)
                .join(constants::RADULA_PATH_BIN)
                .to_str()
                .unwrap(),
                ":",
            ]
            .concat(),
        )
        .join(
            env::var(constants::RADULA_ENVIRONMENT_PATH)?
                .strip_prefix(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
                .unwrap(),
        ),
    );

    Ok(())
}

pub async fn radula_behave_bootstrap_initialize() -> Result<(), Box<dyn Error>> {
    fs::create_dir(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS).unwrap()).await?;
    fs::create_dir(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_LOGS).unwrap()).await?;
    fs::create_dir(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_SOURCES).unwrap()).await?;
    fs::create_dir(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY).unwrap()).await?;

    Ok(())
}

pub fn radula_behave_bootstrap_toolchain_backup() {
    radula_behave_rsync(
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS).unwrap(),
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS).unwrap(),
    );
    radula_behave_rsync(
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN).unwrap(),
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS).unwrap(),
    );

    // Backup toolchain log file
    radula_behave_rsync(
        &env::var(constants::RADULA_ENVIRONMENT_FILE_TOOLCHAIN_LOG).unwrap(),
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS).unwrap(),
    );
}

pub fn radula_behave_bootstrap_toolchain_construct() {
    let radula_behave_construct_toolchain = |x: &'static str| {
        construct::radula_behave_construct(x, constants::RADULA_DIRECTORY_TOOLCHAIN);
    };

    radula_behave_construct_toolchain(constants::RADULA_CERAS_MUSL_HEADERS);
    radula_behave_construct_toolchain(constants::RADULA_CERAS_BINUTILS);
    radula_behave_construct_toolchain(constants::RADULA_CERAS_GCC);
    radula_behave_construct_toolchain(constants::RADULA_CERAS_MUSL);
    radula_behave_construct_toolchain(constants::RADULA_CERAS_LIBGCC);
    radula_behave_construct_toolchain(constants::RADULA_CERAS_LIBSTDCXX_V3);
    radula_behave_construct_toolchain(constants::RADULA_CERAS_LIBGOMP);
}

pub fn radula_behave_bootstrap_toolchain_environment() -> Result<(), Box<dyn Error>> {
    let x = &Path::new(&env::var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY,
    )?)
    .join(constants::RADULA_DIRECTORY_TOOLCHAIN);

    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY,
        x,
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY_BUILDS,
        x.join(constants::RADULA_DIRECTORY_BUILDS),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY_SOURCES,
        x.join(constants::RADULA_DIRECTORY_SOURCES),
    );

    // toolchain log file
    env::set_var(
        constants::RADULA_ENVIRONMENT_FILE_TOOLCHAIN_LOG,
        [
            Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_LOGS)?)
                .join(constants::RADULA_DIRECTORY_TOOLCHAIN)
                .to_str()
                .unwrap(),
            ".",
            constants::RADULA_DIRECTORY_LOGS,
        ]
        .concat(),
    );

    Ok(())
}

pub fn radula_behave_bootstrap_toolchain_prepare() {
    fs::create_dir(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS).unwrap());

    fs::create_dir(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN).unwrap());

    fs::create_dir(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY).unwrap());
    fs::create_dir(
        env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY_BUILDS).unwrap(),
    );
    // Create the `src` directory if it doesn't exist, but don't remove it if it does exist!
    fs::create_dir(
        env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY_SOURCES).unwrap(),
    );
}

pub fn radula_behave_bootstrap_toolchain_release() {
    let x = &String::from(
        Path::new(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
            .join(constants::RADULA_DIRECTORY_TEMPORARY)
            .join(constants::RADULA_DIRECTORY_TOOLCHAIN)
            .to_str()
            .unwrap(),
    );

    fs::remove_dir_all(x);
    fs::create_dir(x);

    radula_behave_rsync(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS).unwrap())
            .join(constants::RADULA_DIRECTORY_CROSS)
            .to_str()
            .unwrap(),
        x,
    );
    radula_behave_rsync(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS).unwrap())
            .join(constants::RADULA_DIRECTORY_TOOLCHAIN)
            .to_str()
            .unwrap(),
        x,
    );

    // Remove all `lib64` directories because glaucus is a pure 64-bit system
    fs::remove_dir_all(
        Path::new(x)
            .join(constants::RADULA_DIRECTORY_CROSS)
            .join(constants::RADULA_PATH_LIB64),
    );
    fs::remove_dir_all(
        Path::new(x)
            .join(constants::RADULA_DIRECTORY_CROSS)
            .join(constants::RADULA_PATH_USR)
            .join(constants::RADULA_PATH_LIB64),
    );
    fs::remove_dir_all(
        Path::new(x)
            .join(constants::RADULA_DIRECTORY_TOOLCHAIN)
            .join(constants::RADULA_PATH_LIB64),
    );

    Command::new(constants::RADULA_TOOTH_FIND)
        .args(&[x, "-name", "*.la", "-delete"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    let radula_behave_bootstrap_toolchain_strip_libraries = |x: &str| {
        Command::new(constants::RADULA_TOOTH_SHELL)
            .args(&[
                constants::RADULA_TOOTH_SHELL_FLAGS,
                &[constants::RADULA_CROSS_STRIP, &format!(" -gv {}/*", x)].concat(),
            ])
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    };

    radula_behave_bootstrap_toolchain_strip_libraries(
        Path::new(x)
            .join(constants::RADULA_DIRECTORY_CROSS)
            .join(constants::RADULA_PATH_USR)
            .join(constants::RADULA_PATH_LIB)
            .to_str()
            .unwrap(),
    );
    radula_behave_bootstrap_toolchain_strip_libraries(
        Path::new(x)
            .join(constants::RADULA_DIRECTORY_TOOLCHAIN)
            .join(constants::RADULA_PATH_LIB)
            .to_str()
            .unwrap(),
    );

    let radula_behave_bootstrap_toolchain_strip_binaries = |x: &str| {
        Command::new(constants::RADULA_TOOTH_SHELL)
            .args(&[
                constants::RADULA_TOOTH_SHELL_FLAGS,
                &[
                    constants::RADULA_CROSS_STRIP,
                    &format!(" --strip-unneeded -v {}/*", x),
                ]
                .concat(),
            ])
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    };

    radula_behave_bootstrap_toolchain_strip_binaries(
        Path::new(x)
            .join(constants::RADULA_DIRECTORY_CROSS)
            .join(constants::RADULA_PATH_USR)
            .join(constants::RADULA_PATH_BIN)
            .to_str()
            .unwrap(),
    );
    radula_behave_bootstrap_toolchain_strip_binaries(
        Path::new(x)
            .join(constants::RADULA_DIRECTORY_TOOLCHAIN)
            .join(constants::RADULA_PATH_BIN)
            .to_str()
            .unwrap(),
    );

    // Remove toolchain manual pages
    fs::remove_dir_all(
        Path::new(x)
            .join(constants::RADULA_DIRECTORY_TOOLCHAIN)
            .join(constants::RADULA_PATH_SHARE)
            .join(constants::RADULA_PATH_INFO),
    );
    fs::remove_dir_all(
        Path::new(x)
            .join(constants::RADULA_DIRECTORY_TOOLCHAIN)
            .join(constants::RADULA_PATH_SHARE)
            .join(constants::RADULA_PATH_MAN),
    );

    // Until we get the tar crate working
    // Command::new(constants::RADULA_TOOTH_TAR)
    //     .args(&[
    //         "cpvf",
    //         &format!(
    //             "{}-{}.tar.zst",
    //             Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS).unwrap())
    //                 .join(constants::RADULA_DIRECTORY_TOOLCHAIN)
    //                 .to_str()
    //                 .unwrap(),
    //             String::from_utf8_lossy(
    //                 &Command::new(constants::RADULA_TOOTH_DATE)
    //                     .arg("+%d%m%Y")
    //                     .output()
    //                     .unwrap()
    //                     .stdout
    //             )
    //             .trim(),
    //         ),
    //         "-I",
    //         "zstd -22 --ultra --long=31 -T0",
    //         ".",
    //     ])
    //     .current_dir(x)
    //     .stdout(Stdio::null())
    //     .spawn()
    //     .unwrap()
    //     .wait()
    //     .unwrap();
}

pub fn radula_behave_pkg_config_environment() {
    env::set_var(
        constants::RADULA_ENVIRONMENT_PKG_CONFIG_LIBDIR,
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS).unwrap()).join(
            constants::RADULA_PATH_PKG_CONFIG_LIBDIR_PATH
                .strip_prefix(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
                .unwrap(),
        ),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_PKG_CONFIG_PATH,
        env::var(constants::RADULA_ENVIRONMENT_PKG_CONFIG_LIBDIR).unwrap(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_PKG_CONFIG_SYSROOT_DIR,
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS).unwrap()).join(
            constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR
                .strip_prefix(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
                .unwrap(),
        ),
    );

    // These environment variables are only `pkgconf` specific, but setting them
    // won't do any harm...
    env::set_var(
        constants::RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_INCLUDE_PATH,
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS).unwrap()).join(
            constants::RADULA_PATH_PKG_CONFIG_SYSTEM_INCLUDE_PATH
                .strip_prefix(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
                .unwrap(),
        ),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_LIBRARY_PATH,
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS).unwrap()).join(
            constants::RADULA_PATH_PKG_CONFIG_SYSTEM_LIBRARY_PATH
                .strip_prefix(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
                .unwrap(),
        ),
    );
}

fn radula_behave_rsync(x: &str, y: &str) {
    Command::new(constants::RADULA_TOOTH_RSYNC)
        .args(&[constants::RADULA_TOOTH_RSYNC_FLAGS, x, y, "--delete"])
        .stdout(Stdio::null())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

pub fn radula_behave_teeth_environment() {
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_AUTORECONF,
        [
            constants::RADULA_TOOTH_AUTORECONF,
            " ",
            constants::RADULA_TOOTH_AUTORECONF_FLAGS,
        ]
        .concat(),
    );

    // Use `mawk` as the default AWK implementation
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_AWK,
        constants::RADULA_TOOTH_MAWK,
    );

    // Use `byacc` as the default YACC implementation
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_BISON,
        constants::RADULA_TOOTH_BYACC,
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_CHMOD,
        [
            constants::RADULA_TOOTH_CHMOD,
            " ",
            constants::RADULA_TOOTH_CHMOD_CHOWN_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_CHOWN,
        [
            constants::RADULA_TOOTH_CHOWN,
            " ",
            constants::RADULA_TOOTH_CHMOD_CHOWN_FLAGS,
        ]
        .concat(),
    );

    // Use `flex` as the default LEX implementation (will be replaced by `reflex` in the future)
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_FLEX,
        constants::RADULA_TOOTH_FLEX,
    );

    // Use `mawk` as the default AWK implementation
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_GAWK,
        constants::RADULA_TOOTH_MAWK,
    );

    // Use `flex` as the default LEX implementation (will be replaced by `reflex` in the future)
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_LEX,
        constants::RADULA_TOOTH_FLEX,
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_LN,
        [
            constants::RADULA_TOOTH_LN,
            " ",
            constants::RADULA_TOOTH_LN_FLAGS,
        ]
        .concat(),
    );

    // `make` and its flags
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_MAKE,
        constants::RADULA_TOOTH_MAKE,
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_MAKEFLAGS,
        [
            "-j",
            &(num_cpus::get() as f32 * 1.5).to_string(),
            " ",
            constants::RADULA_TOOTH_MAKEFLAGS,
        ]
        .concat(),
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_MKDIR,
        [
            constants::RADULA_TOOTH_MKDIR,
            " ",
            constants::RADULA_TOOTH_MKDIR_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_MKDIR_P,
        [
            constants::RADULA_TOOTH_MKDIR,
            " ",
            constants::RADULA_TOOTH_MKDIR_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_MV,
        [
            constants::RADULA_TOOTH_MV,
            " ",
            constants::RADULA_TOOTH_MV_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_PATCH,
        [
            constants::RADULA_TOOTH_PATCH,
            " ",
            constants::RADULA_TOOTH_PATCH_FLAGS,
        ]
        .concat(),
    );

    // Use `pkgconf` as the default PKG_CONFIG implementation
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_PKG_CONFIG,
        constants::RADULA_TOOTH_PKGCONF,
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_RM,
        [
            constants::RADULA_TOOTH_RM,
            " ",
            constants::RADULA_TOOTH_RM_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_RSYNC,
        [
            constants::RADULA_TOOTH_RSYNC,
            " ",
            constants::RADULA_TOOTH_RSYNC_FLAGS,
        ]
        .concat(),
    );

    // Use `byacc` as the default YACC implementation
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_YACC,
        constants::RADULA_TOOTH_BYACC,
    );
}
