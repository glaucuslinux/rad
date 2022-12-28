// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::error::Error;
use std::path::Path;

use super::constants;

use tokio::fs;

//
// Bootstrap Functions
//

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
        env::join_paths(
            [Path::new(&env::var(
                constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN,
            )?)
            .join(constants::RADULA_PATH_USR)
            .join(constants::RADULA_PATH_BIN)]
            .into_iter()
            .chain(env::split_paths(&env::var(
                constants::RADULA_ENVIRONMENT_PATH,
            )?)),
        )?,
    );

    Ok(())
}

pub async fn radula_behave_bootstrap_initialize() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS)?).await?;
    fs::create_dir_all(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_LOGS)?).await?;
    fs::create_dir_all(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_SOURCES)?).await?;
    fs::create_dir_all(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY)?).await?;

    Ok(())
}

#[tokio::test]
async fn test_radula_behave_bootstrap_environment() -> Result<(), Box<dyn Error>> {
    radula_behave_bootstrap_environment().await?;

    println!(
        "\nGLAD :: {}\n",
        env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS)?
    );

    println!(
        "BAKD :: {}",
        env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS)?
    );
    println!(
        "CERD :: {}",
        env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CERATA)?
    );
    println!(
        "CRSD :: {}",
        env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS)?
    );
    println!(
        "LOGD :: {}",
        env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_LOGS)?
    );
    println!(
        "SRCD :: {}",
        env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_SOURCES)?
    );
    println!(
        "TMPD :: {}",
        env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY)?
    );
    println!(
        "TLCD :: {}",
        env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN)?
    );

    println!(
        "\nPATH :: {}\n",
        env::var(constants::RADULA_ENVIRONMENT_PATH)?
    );

    assert!(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS)?.ends_with("glaucus"));

    assert!(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS)?.ends_with("bak"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CERATA)?.ends_with("cerata"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS)?.ends_with("cross"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_LOGS)?.ends_with("log"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_SOURCES)?.ends_with("src"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY)?.ends_with("tmp"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN)?.ends_with("toolchain"));

    assert!(
        env::split_paths(&env::var(constants::RADULA_ENVIRONMENT_PATH)?)
            .next()
            .unwrap_or_default()
            .ends_with("toolchain/usr/bin")
    );

    Ok(())
}
