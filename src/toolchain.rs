// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::error::Error;
use std::path::Path;
use std::process::{Command, Stdio};
use std::string::String;

use super::constants;
use super::construct;
use super::rsync;

use tokio::fs;

//
// Toolchain Functions
//

pub fn radula_behave_bootstrap_toolchain_backup() -> Result<(), Box<dyn Error>> {
    rsync::radula_behave_rsync(
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS)?,
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS)?,
    );
    rsync::radula_behave_rsync(
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN)?,
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS)?,
    );

    // Backup toolchain log file
    rsync::radula_behave_rsync(
        &env::var(constants::RADULA_ENVIRONMENT_FILE_TOOLCHAIN_LOG)?,
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS)?,
    );

    Ok(())
}

pub fn radula_behave_bootstrap_toolchain_construct() -> Result<(), Box<dyn Error>> {
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

    Ok(())
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

pub fn radula_behave_bootstrap_toolchain_prepare() -> Result<(), Box<dyn Error>> {
    fs::create_dir(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS)?);

    fs::create_dir(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN)?);

    fs::create_dir(env::var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY,
    )?);
    fs::create_dir(env::var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY_BUILDS,
    )?);
    // Create the `src` directory if it doesn't exist, but don't remove it if it does exist!
    fs::create_dir(env::var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY_SOURCES,
    )?);

    Ok(())
}

pub fn radula_behave_bootstrap_toolchain_release() -> Result<(), Box<dyn Error>> {
    let x = &String::from(
        Path::new(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
            .join(constants::RADULA_DIRECTORY_TEMPORARY)
            .join(constants::RADULA_DIRECTORY_TOOLCHAIN)
            .to_str()
            .unwrap(),
    );

    fs::remove_dir_all(x);
    fs::create_dir(x);

    rsync::radula_behave_rsync(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS)?)
            .join(constants::RADULA_DIRECTORY_CROSS)
            .to_str()
            .unwrap(),
        x,
    );
    rsync::radula_behave_rsync(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS)?)
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
        .spawn()?
        .wait()?;

    let radula_behave_bootstrap_toolchain_strip_libraries = |x: &str| {
        Command::new(constants::RADULA_TOOTH_SHELL)
            .args(&[
                constants::RADULA_TOOTH_SHELL_FLAGS,
                &[constants::RADULA_CROSS_STRIP, &format!(" -gv {}/*", x)].concat(),
            ])
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .spawn()?
            .wait()?;
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
            .spawn()?
            .wait()?;
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
    //             Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS)?)
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
    //     .spawn()?
    //     .wait()?;

    Ok(())
}
