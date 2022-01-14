// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::path::Path;

use super::constants;

use tokio::fs;

pub async fn radula_behave_bootstrap_clean() -> Result<(), Box<dyn std::error::Error>> {
    fs::remove_dir_all(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS).unwrap()).await?;

    fs::remove_dir_all(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_LOGS).unwrap()).await?;

    fs::remove_dir_all(
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY_BUILDS).unwrap(),
    )
    .await?;

    fs::remove_dir_all(
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY_BUILDS).unwrap(),
    )
    .await?;

    fs::remove_dir_all(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN).unwrap())
        .await?;

    Ok(())
}

pub async fn radula_behave_bootstrap_distclean() -> Result<(), Box<dyn std::error::Error>> {
    fs::remove_dir_all(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS).unwrap()).await?;

    fs::remove_file(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS).unwrap())
            .join(constants::RADULA_FILE_GLAUCUS_IMAGE),
    ).await?;

    fs::remove_dir_all(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_SOURCES).unwrap()).await?;

    radula_behave_bootstrap_clean().await?;

    // Only remove `RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY` completely after
    // `RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_BUILDS` and
    // `RADULA_ENVIRONMENT_DIRECTORY_CROSS_BUILDS` are removed
    fs::remove_dir_all(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY).unwrap()).await?;

    Ok(())
}
