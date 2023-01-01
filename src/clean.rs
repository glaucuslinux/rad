// Copyright (c) 2018-2023, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::error::Error;
use std::path::Path;

use super::constants;

use tokio::fs;

//
// Clean Functions
//

pub async fn radula_behave_remove_dir_all_force(path: &str) -> Result<(), Box<dyn Error>> {
    if Path::is_dir(Path::new(path)) {
        fs::remove_dir_all(path).await?;
    }

    Ok(())
}

pub async fn radula_behave_remove_file_force(path: &str) -> Result<(), Box<dyn Error>> {
    if Path::is_file(Path::new(path)) {
        fs::remove_file(path).await?;
    }

    Ok(())
}

pub async fn radula_behave_bootstrap_clean() -> Result<(), Box<dyn Error>> {
    radula_behave_remove_dir_all_force(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS)?)
        .await?;

    radula_behave_remove_dir_all_force(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_LOGS)?)
        .await?;

    radula_behave_remove_dir_all_force(&env::var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY_BUILDS,
    )?)
    .await?;

    radula_behave_remove_dir_all_force(&env::var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY_BUILDS,
    )?)
    .await?;

    radula_behave_remove_dir_all_force(&env::var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN,
    )?)
    .await?;

    Ok(())
}

pub async fn radula_behave_bootstrap_distclean() -> Result<(), Box<dyn Error>> {
    radula_behave_remove_dir_all_force(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS)?)
        .await?;

    radula_behave_remove_file_force(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS)?)
            .join(constants::RADULA_FILE_GLAUCUS_IMG)
            .to_str()
            .unwrap_or_default(),
    )
    .await?;

    radula_behave_remove_dir_all_force(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_SOURCES)?)
        .await?;

    radula_behave_bootstrap_clean().await?;

    // Only remove `RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY` completely after
    // `RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_BUILDS` and
    // `RADULA_ENVIRONMENT_DIRECTORY_CROSS_BUILDS` are removed
    radula_behave_remove_dir_all_force(&env::var(
        constants::RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY,
    )?)
    .await?;

    Ok(())
}
