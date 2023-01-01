// Copyright (c) 2018-2023, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::error::Error;
use std::path::Path;

use super::clean;
use super::constants;
use super::construct;
use super::rsync;

use tokio::fs;

//
// Cross Functions
//

pub async fn radula_behave_bootstrap_cross_construct() -> Result<(), Box<dyn Error>> {
    let radula_behave_construct_cross = |x: &'static str| async move {
        construct::radula_behave_construct(x, constants::RADULA_DIRECTORY_CROSS)
            .await
            .unwrap();
    };

    // Filesystem & Package Management
    radula_behave_construct_cross(constants::RADULA_CERAS_HYDROSKELETON).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_IANA_ETC).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_CERATA).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_RADULA).await;

    // Headers
    radula_behave_construct_cross(constants::RADULA_CERAS_MUSL_UTILS).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_LINUX_HEADERS).await;

    // Init
    radula_behave_construct_cross(constants::RADULA_CERAS_SKALIBS).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_NSSS).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_EXECLINE).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_S6).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_UTMPS).await;

    // Compatibility
    radula_behave_construct_cross(constants::RADULA_CERAS_MUSL_FTS).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_MUSL_OBSTACK).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_MUSL_RPMATCH).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_LIBUCONTEXT).await;

    // i18n & L10n
    radula_behave_construct_cross(constants::RADULA_CERAS_GETTEXT_TINY).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_MUSL_LOCALES).await;

    // Permissions
    radula_behave_construct_cross(constants::RADULA_CERAS_ATTR).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_ACL).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_SHADOW).await;

    // Hashing
    radula_behave_construct_cross(constants::RADULA_CERAS_LIBRESSL).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_XXHASH).await;

    // Userland
    radula_behave_construct_cross(constants::RADULA_CERAS_TOYBOX).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_DIFFUTILS).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_FILE).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_FINDUTILS).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_SED).await;

    // Development
    radula_behave_construct_cross(constants::RADULA_CERAS_EXPAT).await;

    // Compression
    radula_behave_construct_cross(constants::RADULA_CERAS_BZIP2).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_LBZIP2).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_LBZIP2_UTILS).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_LZ4).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_LZLIB).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_PLZIP).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_XZ).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_ZLIB_NG).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_PIGZ).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_ZSTD).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_LIBARCHIVE).await;

    // Development
    // radula_behave_construct_cross(constants::RADULA_CERAS_AUTOCONF).await;
    // radula_behave_construct_cross(constants::RADULA_CERAS_AUTOMAKE).await;
    // radula_behave_construct_cross(constants::RADULA_CERAS_BINUTILS).await;
    // radula_behave_construct_cross(constants::RADULA_CERAS_BYACC).await;
    // radula_behave_construct_cross(constants::RADULA_CERAS_CMAKE).await;
    // radula_behave_construct_cross(constants::RADULA_CERAS_FLEX).await;
    // radula_behave_construct_cross(constants::RADULA_CERAS_GCC).await;
    // radula_behave_construct_cross(constants::RADULA_CERAS_HELP2MAN).await;
    // radula_behave_construct_cross(constants::RADULA_CERAS_LIBTOOL).await;
    // radula_behave_construct_cross(constants::RADULA_CERAS_MAKE).await;
    // radula_behave_construct_cross(constants::RADULA_CERAS_MAWK).await;
    // radula_behave_construct_cross(constants::RADULA_CERAS_OM4).await;
    // radula_behave_construct_cross(constants::RADULA_CERAS_PATCH).await;
    // radula_behave_construct_cross(constants::RADULA_CERAS_PKGCONF).await;
    // radula_behave_construct_cross(constants::RADULA_CERAS_PYTHON).await;
    // radula_behave_construct_cross(constants::RADULA_CERAS_SAMURAI).await;

    // Synchronization
    // radula_behave_construct_cross(constants::RADULA_CERAS_RSYNC).await;

    // Editors, Pagers and Shells
    radula_behave_construct_cross(constants::RADULA_CERAS_NETBSD_CURSES).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_LIBEDIT).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_PCRE2).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_DASH).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_YASH).await;
    // radula_behave_construct_cross(constants::RADULA_CERAS_LESS).await;
    // radula_behave_construct_cross(constants::RADULA_CERAS_VIM).await;
    // radula_behave_construct_cross(constants::RADULA_CERAS_MANDOC).await;

    // Userland
    radula_behave_construct_cross(constants::RADULA_CERAS_BC).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_GREP).await;
    // radula_behave_construct_cross(constants::RADULA_CERAS_PLOCATE).await;

    // Networking
    radula_behave_construct_cross(constants::RADULA_CERAS_LIBCAP).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_LIBCAP_NG).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_IPROUTE2).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_IPUTILS).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_SDHCP).await;
    // radula_behave_construct_cross(constants::RADULA_CERAS_CURL).await;
    // radula_behave_construct_cross(constants::RADULA_CERAS_WGET2).await;

    // Time Zone
    radula_behave_construct_cross(constants::RADULA_CERAS_TZCODE).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_TZDATA).await;

    // Utilities
    radula_behave_construct_cross(constants::RADULA_CERAS_KMOD).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_EUDEV).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_PSMISC).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_PROCPS_NG).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_UTIL_LINUX).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_E2FSPROGS).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_PCIUTILS).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_HWDATA).await;

    // Services
    radula_behave_construct_cross(constants::RADULA_CERAS_S6_LINUX_INIT).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_S6_RC).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_S6_BOOT_SCRIPTS).await;

    // Kernel
    radula_behave_construct_cross(constants::RADULA_CERAS_LIBUARGP).await;
    radula_behave_construct_cross(constants::RADULA_CERAS_LIBELF).await;
    // radula_behave_construct_cross(constants::RADULA_CERAS_LINUX).await;

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
                .unwrap_or_default(),
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
            .unwrap_or_default(),
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS)?,
    )
    .await?;
    rsync::radula_behave_rsync(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS)?)
            .join(constants::RADULA_DIRECTORY_TOOLCHAIN)
            .to_str()
            .unwrap_or_default(),
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS)?,
    )
    .await?;

    clean::radula_behave_remove_dir_all_force(&env::var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY_BUILDS,
    )?)
    .await?;
    fs::create_dir_all(env::var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY_BUILDS,
    )?)
    .await?;

    // Create the `src` directory if it doesn't exist, but don't remove it if it does exist!
    fs::create_dir_all(env::var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY_SOURCES,
    )?)
    .await?;

    // Create the `log` directory if it doesn't exist, but don't remove it if it does exist!
    fs::create_dir_all(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_LOGS)?).await?;

    // Remove cross log file if it exists
    clean::radula_behave_remove_file_force(&env::var(
        constants::RADULA_ENVIRONMENT_FILE_CROSS_LOG,
    )?)
    .await?;

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
