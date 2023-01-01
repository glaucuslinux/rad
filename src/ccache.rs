// Copyright (c) 2018-2023, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::error::Error;
use std::path::Path;

use super::constants;

use tokio::fs;

#[cfg(test)]
use super::cross;
#[cfg(test)]
use super::toolchain;

//
// Ccache Functions
//

pub async fn radula_behave_bootstrap_cross_ccache() -> Result<(), Box<dyn Error>> {
    env::set_var(
        constants::RADULA_ENVIRONMENT_CCACHE_CONFIGURATION,
        Path::new(constants::RADULA_PATH_RADULA_CLUSTERS)
            .join(constants::RADULA_DIRECTORY_GLAUCUS)
            .join(constants::RADULA_CERAS_CCACHE)
            .join(constants::RADULA_FILE_CCACHE_CONFIGURATION),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CCACHE_DIRECTORY,
        Path::new(&env::var(
            constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY,
        )?)
        .join(constants::RADULA_CERAS_CCACHE),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_PATH,
        env::join_paths(
            [Path::new(&env::var(
                constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN,
            )?)
            .join(constants::RADULA_PATH_USR)
            .join(constants::RADULA_PATH_LIB)
            .join(constants::RADULA_CERAS_CCACHE)]
            .into_iter()
            .chain(env::split_paths(&env::var(
                constants::RADULA_ENVIRONMENT_PATH,
            )?)),
        )?,
    );

    fs::create_dir_all(env::var(constants::RADULA_ENVIRONMENT_CCACHE_DIRECTORY)?).await?;

    Ok(())
}

pub async fn radula_behave_bootstrap_toolchain_ccache() -> Result<(), Box<dyn Error>> {
    env::set_var(
        constants::RADULA_ENVIRONMENT_CCACHE_CONFIGURATION,
        Path::new(constants::RADULA_PATH_RADULA_CLUSTERS)
            .join(constants::RADULA_DIRECTORY_GLAUCUS)
            .join(constants::RADULA_CERAS_CCACHE)
            .join(constants::RADULA_FILE_CCACHE_CONFIGURATION),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_CCACHE_DIRECTORY,
        Path::new(&env::var(
            constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY,
        )?)
        .join(constants::RADULA_CERAS_CCACHE),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_PATH,
        env::join_paths(
            env::split_paths(constants::RADULA_PATH_CCACHE).chain(env::split_paths(&env::var(
                constants::RADULA_ENVIRONMENT_PATH,
            )?)),
        )?,
    );

    fs::create_dir_all(env::var(constants::RADULA_ENVIRONMENT_CCACHE_DIRECTORY)?).await?;

    Ok(())
}

#[tokio::test]
async fn test_radula_behave_bootstrap_cross_ccache() -> Result<(), Box<dyn Error>> {
    cross::radula_behave_bootstrap_cross_environment_directories()?;

    radula_behave_bootstrap_cross_ccache().await?;

    println!(
        "\nCCACHE_CONFIGPATH :: {}",
        env::var(constants::RADULA_ENVIRONMENT_CCACHE_CONFIGURATION)?
    );
    println!(
        "CCACHE_DIR        :: {}",
        env::var(constants::RADULA_ENVIRONMENT_CCACHE_DIRECTORY)?
    );
    println!(
        "PATH              :: {}\n",
        env::var(constants::RADULA_ENVIRONMENT_PATH)?
    );

    assert!(env::var(constants::RADULA_ENVIRONMENT_CCACHE_CONFIGURATION)?.ends_with("ccache.conf"));
    assert!(env::var(constants::RADULA_ENVIRONMENT_CCACHE_DIRECTORY)?.ends_with("tmp/cross/ccache"));
    // Don't check with ".ends_with()" against "/toolchain/lib/ccache" as the
    // first forward slash won't be matched
    assert!(
        env::split_paths(&env::var(constants::RADULA_ENVIRONMENT_PATH)?)
            .nth(0)
            .unwrap()
            .ends_with("toolchain/usr/lib/ccache")
    );

    Ok(())
}

// Expected to have an additional "toolchain/usr/lib/ccache" before "toolchain/usr/bin"
// as a side-effect for using async/await (which is technically wrong but we'll
// let it slide because that path will be empty until ccache, the last toolchain
// package, is built...
#[tokio::test]
async fn test_radula_behave_bootstrap_toolchain_ccache() -> Result<(), Box<dyn Error>> {
    toolchain::radula_behave_bootstrap_toolchain_environment()?;

    radula_behave_bootstrap_toolchain_ccache().await?;

    println!(
        "\nCCACHE_CONFIGPATH :: {}",
        env::var(constants::RADULA_ENVIRONMENT_CCACHE_CONFIGURATION)?
    );
    println!(
        "CCACHE_DIR        :: {}",
        env::var(constants::RADULA_ENVIRONMENT_CCACHE_DIRECTORY)?
    );
    println!(
        "PATH              :: {}\n",
        env::var(constants::RADULA_ENVIRONMENT_PATH)?
    );

    assert!(env::var(constants::RADULA_ENVIRONMENT_CCACHE_CONFIGURATION)?.ends_with("ccache.conf"));
    assert!(
        env::var(constants::RADULA_ENVIRONMENT_CCACHE_DIRECTORY)?.ends_with("tmp/toolchain/ccache")
    );
    assert!(env::var(constants::RADULA_ENVIRONMENT_PATH)?
        .starts_with("/usr/lib/ccache:/usr/lib/ccache/bin:/usr/lib64/ccache"));

    Ok(())
}
