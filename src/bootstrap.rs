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
    fs::create_dir(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS)?).await?;
    fs::create_dir(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_LOGS)?).await?;
    fs::create_dir(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_SOURCES)?).await?;
    fs::create_dir(env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY)?).await?;

    Ok(())
}
