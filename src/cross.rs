// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::error::Error;
use std::path::Path;
use std::process::{Command, Stdio};
use std::string::String;

use super::clean;
use super::constants;
use super::construct;
use super::rsync;

use tokio::fs;

//
// Cross Functions
//

pub fn radula_behave_bootstrap_cross_construct() -> Result<(), Box<dyn Error>> {
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

    Ok(())
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

pub fn radula_behave_bootstrap_cross_environment_teeth() -> Result<(), Box<dyn Error>> {
    let x = &[&env::var(constants::RADULA_ENVIRONMENT_TUPLE_TARGET)?, "-"].concat();

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

    Ok(())
}

pub async fn radula_behave_bootstrap_cross_prepare() -> Result<(), Box<dyn Error>> {
    rsync::radula_behave_rsync(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS)?)
            .join(constants::RADULA_DIRECTORY_CROSS)
            .to_str()
            .unwrap(),
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS)?,
    )?;
    rsync::radula_behave_rsync(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS)?)
            .join(constants::RADULA_DIRECTORY_TOOLCHAIN)
            .to_str()
            .unwrap(),
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS)?,
    )?;

    clean::radula_behave_remove_dir_all_force(&env::var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY_BUILDS,
    )?)
    .await?;
    fs::create_dir(env::var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY_BUILDS,
    )?)
    .await?;

    // Create the `src` directory if it doesn't exist, but don't remove it if it does exist!
    fs::create_dir(env::var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY_SOURCES,
    )?)
    .await?;

    // Create the `log` directory if it doesn't exist, but don't remove it if it does exist!
    fs::create_dir(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_LOGS)?).await?;

    // Remove cross log file if it exists
    clean::radula_behave_remove_file_force(&env::var(constants::RADULA_ENVIRONMENT_FILE_CROSS_LOG)?)
        .await?;

    Ok(())
}

// This function is not a mess anymore but it still breaks some libraries
fn radula_behave_bootstrap_cross_strip() -> Result<(), Box<dyn Error>> {
    Command::new(constants::RADULA_TOOTH_FIND)
        .args(&[
            Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS)?)
                .join(constants::RADULA_PATH_ETC)
                .to_str()
                .unwrap(),
            "-type",
            "d",
            "-empty",
            "-delete",
        ])
        .spawn()?
        .wait()?;

    let x = &String::from(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS)?)
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
        .spawn()?
        .wait()?;
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
        .spawn()?
        .wait()?;
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
        .spawn()?
        .wait()?;
    Command::new(constants::RADULA_TOOTH_FIND)
        .args(&[x, "-name", "*.la", "-delete"])
        .spawn()?
        .wait()?;

    Ok(())
}

#[test]
fn test_radula_behave_bootstrap_cross_environment() -> Result<(), Box<dyn Error>> {
    radula_behave_bootstrap_cross_environment_directories()?;
    radula_behave_bootstrap_cross_environment_teeth()?;

    println!(
        "\nCRSD :: {}",
        env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS)?
    );
    println!(
        "XTMP :: {}",
        env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY)?
    );
    println!(
        "XBLD :: {}",
        env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY_BUILDS)?
    );
    println!(
        "XSRC :: {}",
        env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY_SOURCES)?
    );
    println!(
        "XLOG :: {}\n",
        env::var(constants::RADULA_ENVIRONMENT_FILE_CROSS_LOG)?
    );

    println!(
        "AR            :: {}",
        env::var(constants::RADULA_ENVIRONMENT_CROSS_ARCHIVER)?
    );
    println!(
        "AS            :: {}",
        env::var(constants::RADULA_ENVIRONMENT_CROSS_ASSEMBLER)?
    );
    println!(
        "BUILD_CC      :: {}",
        env::var(constants::RADULA_ENVIRONMENT_CROSS_BUILD_C_COMPILER)?
    );
    println!(
        "CROSS_COMPILE :: {}",
        env::var(constants::RADULA_ENVIRONMENT_CROSS_COMPILE)?
    );
    println!(
        "CC            :: {}",
        env::var(constants::RADULA_ENVIRONMENT_CROSS_C_COMPILER)?
    );
    println!(
        "CC_LD         :: {}",
        env::var(constants::RADULA_ENVIRONMENT_CROSS_C_COMPILER_LINKER)?
    );
    println!(
        "CPP           :: {}",
        env::var(constants::RADULA_ENVIRONMENT_CROSS_C_PREPROCESSOR)?
    );
    println!(
        "CXX           :: {}",
        env::var(constants::RADULA_ENVIRONMENT_CROSS_CXX_COMPILER)?
    );
    println!(
        "CXX_LD        :: {}",
        env::var(constants::RADULA_ENVIRONMENT_CROSS_CXX_COMPILER_LINKER)?
    );
    println!(
        "HOSTCC        :: {}",
        env::var(constants::RADULA_ENVIRONMENT_CROSS_HOST_C_COMPILER)?
    );
    println!(
        "HOSTCXX       :: {}",
        env::var(constants::RADULA_ENVIRONMENT_CROSS_HOST_CXX_COMPILER)?
    );
    println!(
        "LD            :: {}",
        env::var(constants::RADULA_ENVIRONMENT_CROSS_LINKER)?
    );
    println!(
        "NM            :: {}",
        env::var(constants::RADULA_ENVIRONMENT_CROSS_NAMES)?
    );
    println!(
        "OBJCOPY       :: {}",
        env::var(constants::RADULA_ENVIRONMENT_CROSS_OBJECT_COPY)?
    );
    println!(
        "OBJDUMP       :: {}",
        env::var(constants::RADULA_ENVIRONMENT_CROSS_OBJECT_DUMP)?
    );
    println!(
        "RANLIB        :: {}",
        env::var(constants::RADULA_ENVIRONMENT_CROSS_RANDOM_ACCESS_LIBRARY)?
    );
    println!(
        "READELF       :: {}",
        env::var(constants::RADULA_ENVIRONMENT_CROSS_READ_ELF)?
    );
    println!(
        "SIZE          :: {}",
        env::var(constants::RADULA_ENVIRONMENT_CROSS_SIZE)?
    );
    println!(
        "STRINGS       :: {}",
        env::var(constants::RADULA_ENVIRONMENT_CROSS_STRINGS)?
    );
    println!(
        "STRIP         :: {}\n",
        env::var(constants::RADULA_ENVIRONMENT_CROSS_STRIP)?
    );

    assert!(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_LOGS)?.ends_with("log"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_SOURCES)?.ends_with("src"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY)?.ends_with("tmp"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN)?.ends_with("toolchain"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_FILE_CROSS_LOG)?.ends_with("cross.log"));

    assert!(env::var(constants::RADULA_ENVIRONMENT_CROSS_ARCHIVER)?.ends_with("gcc-ar"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_CROSS_ASSEMBLER)?.ends_with("as"));
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_CROSS_BUILD_C_COMPILER)?,
        "gcc"
    );
    assert!(env::var(constants::RADULA_ENVIRONMENT_CROSS_COMPILE)?.ends_with("-"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_CROSS_C_COMPILER)?.ends_with("gcc"));
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_CROSS_C_COMPILER_LINKER)?,
        "bfd"
    );
    assert!(env::var(constants::RADULA_ENVIRONMENT_CROSS_C_PREPROCESSOR)?.ends_with("gcc -E"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_CROSS_CXX_COMPILER)?.ends_with("g++"));
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_CROSS_CXX_COMPILER_LINKER)?,
        "bfd"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_CROSS_HOST_C_COMPILER)?,
        "gcc"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_CROSS_HOST_CXX_COMPILER)?,
        "g++"
    );
    assert!(env::var(constants::RADULA_ENVIRONMENT_CROSS_LINKER)?.ends_with("ld.bfd"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_CROSS_NAMES)?.ends_with("gcc-nm"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_CROSS_OBJECT_COPY)?.ends_with("objcopy"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_CROSS_OBJECT_DUMP)?.ends_with("objdump"));
    assert!(
        env::var(constants::RADULA_ENVIRONMENT_CROSS_RANDOM_ACCESS_LIBRARY)?
            .ends_with("gcc-ranlib")
    );
    assert!(env::var(constants::RADULA_ENVIRONMENT_CROSS_READ_ELF)?.ends_with("readelf"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_CROSS_SIZE)?.ends_with("size"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_CROSS_STRINGS)?.ends_with("strings"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_CROSS_STRIP)?.ends_with("strip"));

    Ok(())
}
