// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::string::String;

use super::clean;
use super::compress;
use super::constants;
use super::construct;
use super::rsync;

use chrono::Utc;
use tokio::{fs, process::Command};
use walkdir::WalkDir;

//
// Toolchain Functions
//

pub async fn radula_behave_bootstrap_toolchain_backup() -> Result<(), Box<dyn Error>> {
    rsync::radula_behave_rsync(
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS)?,
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS)?,
    )
    .await?;
    rsync::radula_behave_rsync(
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN)?,
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS)?,
    )
    .await?;

    // Backup toolchain log file
    rsync::radula_behave_rsync(
        &env::var(constants::RADULA_ENVIRONMENT_FILE_TOOLCHAIN_LOG)?,
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS)?,
    )
    .await?;

    Ok(())
}

pub async fn radula_behave_bootstrap_toolchain_construct() -> Result<(), Box<dyn Error>> {
    let radula_behave_construct_toolchain = |x: &'static str| async move {
        construct::radula_behave_construct(x, constants::RADULA_DIRECTORY_TOOLCHAIN)
            .await
            .unwrap();
    };

    radula_behave_construct_toolchain(constants::RADULA_CERAS_MUSL_HEADERS).await;
    radula_behave_construct_toolchain(constants::RADULA_CERAS_BINUTILS).await;
    radula_behave_construct_toolchain(constants::RADULA_CERAS_GCC).await;
    radula_behave_construct_toolchain(constants::RADULA_CERAS_MUSL).await;
    radula_behave_construct_toolchain(constants::RADULA_CERAS_LIBGCC).await;
    radula_behave_construct_toolchain(constants::RADULA_CERAS_LIBSTDCXX_V3).await;
    radula_behave_construct_toolchain(constants::RADULA_CERAS_LIBGOMP).await;
    radula_behave_construct_toolchain(constants::RADULA_CERAS_CCACHE).await;

    Ok(())
}

pub fn radula_behave_bootstrap_toolchain_environment() -> Result<(), Box<dyn Error>> {
    let path = &Path::new(&env::var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY,
    )?)
    .join(constants::RADULA_DIRECTORY_TOOLCHAIN);

    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY,
        path,
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY_BUILDS,
        path.join(constants::RADULA_DIRECTORY_BUILDS),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY_SOURCES,
        path.join(constants::RADULA_DIRECTORY_SOURCES),
    );

    // toolchain log file
    env::set_var(
        constants::RADULA_ENVIRONMENT_FILE_TOOLCHAIN_LOG,
        [
            Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_LOGS)?)
                .join(constants::RADULA_DIRECTORY_TOOLCHAIN)
                .to_str()
                .unwrap_or_default(),
            ".",
            constants::RADULA_DIRECTORY_LOGS,
        ]
        .concat(),
    );

    Ok(())
}

pub async fn radula_behave_bootstrap_toolchain_prepare() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS)?).await?;

    fs::create_dir_all(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN)?).await?;

    fs::create_dir_all(env::var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY,
    )?)
    .await?;
    fs::create_dir_all(env::var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY_BUILDS,
    )?)
    .await?;
    // Create the `src` directory if it doesn't exist, but don't remove it if it does exist!
    fs::create_dir_all(env::var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY_SOURCES,
    )?)
    .await?;

    Ok(())
}

pub async fn radula_behave_bootstrap_toolchain_release() -> Result<(), Box<dyn Error>> {
    let path = &String::from(
        Path::new(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
            .join(constants::RADULA_DIRECTORY_TEMPORARY)
            .join(constants::RADULA_DIRECTORY_TOOLCHAIN)
            .to_str()
            .unwrap_or_default(),
    );

    clean::radula_behave_remove_dir_all_force(path).await?;
    fs::create_dir_all(path).await?;

    rsync::radula_behave_rsync(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS)?)
            .join(constants::RADULA_DIRECTORY_CROSS)
            .to_str()
            .unwrap_or_default(),
        path,
    )
    .await?;
    rsync::radula_behave_rsync(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS)?)
            .join(constants::RADULA_DIRECTORY_TOOLCHAIN)
            .to_str()
            .unwrap_or_default(),
        path,
    )
    .await?;

    // Remove all `lib64` directories because glaucus is a pure 64-bit system
    clean::radula_behave_remove_dir_all_force(
        Path::new(path)
            .join(constants::RADULA_DIRECTORY_CROSS)
            .join(constants::RADULA_PATH_LIB64)
            .to_str()
            .unwrap_or_default(),
    )
    .await?;
    clean::radula_behave_remove_dir_all_force(
        Path::new(path)
            .join(constants::RADULA_DIRECTORY_CROSS)
            .join(constants::RADULA_PATH_USR)
            .join(constants::RADULA_PATH_LIB64)
            .to_str()
            .unwrap_or_default(),
    )
    .await?;
    clean::radula_behave_remove_dir_all_force(
        Path::new(path)
            .join(constants::RADULA_DIRECTORY_TOOLCHAIN)
            .join(constants::RADULA_PATH_LIB64)
            .to_str()
            .unwrap_or_default(),
    )
    .await?;

    // Remove libtool archive (.la) files
    WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name()
                .to_str()
                .map(|s| s.ends_with(".la"))
                .unwrap_or_default()
        })
        .for_each(|e| std::fs::remove_file(e.path()).unwrap());

    let radula_behave_bootstrap_toolchain_strip_libraries = |path: PathBuf| async move {
        Command::new(constants::RADULA_TOOTH_SHELL)
            .args(&[
                constants::RADULA_TOOTH_SHELL_FLAGS,
                &[
                    constants::RADULA_CROSS_STRIP,
                    &format!(" -gv {}/*", path.display()),
                ]
                .concat(),
            ])
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
            .unwrap()
            .wait()
            .await
            .unwrap();
    };

    radula_behave_bootstrap_toolchain_strip_libraries(
        Path::new(path)
            .join(constants::RADULA_DIRECTORY_CROSS)
            .join(constants::RADULA_PATH_USR)
            .join(constants::RADULA_PATH_LIB),
    )
    .await;
    radula_behave_bootstrap_toolchain_strip_libraries(
        Path::new(path)
            .join(constants::RADULA_DIRECTORY_TOOLCHAIN)
            .join(constants::RADULA_PATH_LIB),
    )
    .await;

    let radula_behave_bootstrap_toolchain_strip_binaries = |path: PathBuf| async move {
        Command::new(constants::RADULA_TOOTH_SHELL)
            .args(&[
                constants::RADULA_TOOTH_SHELL_FLAGS,
                &[
                    constants::RADULA_CROSS_STRIP,
                    &format!(" --strip-unneeded -v {}/*", path.display()),
                ]
                .concat(),
            ])
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
            .unwrap()
            .wait()
            .await
            .unwrap();
    };

    radula_behave_bootstrap_toolchain_strip_binaries(
        Path::new(path)
            .join(constants::RADULA_DIRECTORY_CROSS)
            .join(constants::RADULA_PATH_USR)
            .join(constants::RADULA_PATH_BIN),
    )
    .await;
    radula_behave_bootstrap_toolchain_strip_binaries(
        Path::new(path)
            .join(constants::RADULA_DIRECTORY_TOOLCHAIN)
            .join(constants::RADULA_PATH_BIN),
    )
    .await;

    // Remove toolchain manual pages
    clean::radula_behave_remove_dir_all_force(
        Path::new(path)
            .join(constants::RADULA_DIRECTORY_TOOLCHAIN)
            .join(constants::RADULA_PATH_SHARE)
            .join(constants::RADULA_PATH_INFO)
            .to_str()
            .unwrap_or_default(),
    )
    .await?;
    clean::radula_behave_remove_dir_all_force(
        Path::new(path)
            .join(constants::RADULA_DIRECTORY_TOOLCHAIN)
            .join(constants::RADULA_PATH_SHARE)
            .join(constants::RADULA_PATH_MAN)
            .to_str()
            .unwrap_or_default(),
    )
    .await?;

    compress::radula_behave_compress(
        &format!(
            "{}-{}",
            Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS)?)
                .join(constants::RADULA_DIRECTORY_TOOLCHAIN)
                .to_str()
                .unwrap_or_default(),
            Utc::now().format("%d%m%Y")
        ),
        path,
    )?;

    Ok(())
}

#[test]
fn test_radula_behave_bootstrap_toolchain_environment() -> Result<(), Box<dyn Error>> {
    radula_behave_bootstrap_toolchain_environment()?;

    println!(
        "\nTLCD :: {}",
        env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN)?
    );
    println!(
        "TTMP :: {}",
        env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY)?
    );
    println!(
        "TBLD :: {}",
        env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY_BUILDS)?
    );
    println!(
        "TSRC :: {}",
        env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY_SOURCES)?
    );
    println!(
        "TLOG :: {}\n",
        env::var(constants::RADULA_ENVIRONMENT_FILE_TOOLCHAIN_LOG)?
    );

    assert!(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_LOGS)?.ends_with("log"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_SOURCES)?.ends_with("src"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY)?.ends_with("tmp"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN)?.ends_with("toolchain"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_FILE_TOOLCHAIN_LOG)?.ends_with("toolchain.log"));

    Ok(())
}
