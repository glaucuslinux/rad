// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::error::Error;
use std::path::Path;

use super::constants;
use super::bootstrap;

//
// pkg-config Function
//

pub fn radula_behave_pkg_config_environment() -> Result<(), Box<dyn Error>> {
    env::set_var(
        constants::RADULA_ENVIRONMENT_PKG_CONFIG_LIBDIR,
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS)?).join(
            constants::RADULA_PATH_PKG_CONFIG_LIBDIR_PATH
                .strip_prefix(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
                .unwrap(),
        ),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_PKG_CONFIG_PATH,
        env::var(constants::RADULA_ENVIRONMENT_PKG_CONFIG_LIBDIR)?,
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_PKG_CONFIG_SYSROOT_DIR,
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS)?).join(
            constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR
                .strip_prefix(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
                .unwrap(),
        ),
    );

    // These environment variables are only `pkgconf` specific, but setting them
    // won't do any harm...
    env::set_var(
        constants::RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_INCLUDE_PATH,
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS)?).join(
            constants::RADULA_PATH_PKG_CONFIG_SYSTEM_INCLUDE_PATH
                .strip_prefix(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
                .unwrap(),
        ),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_LIBRARY_PATH,
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS)?).join(
            constants::RADULA_PATH_PKG_CONFIG_SYSTEM_LIBRARY_PATH
                .strip_prefix(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
                .unwrap(),
        ),
    );

    Ok(())
}

#[tokio::test]
pub async fn test_radula_behave_pkg_config_environment() -> Result<(), Box<dyn Error>> {
    bootstrap::radula_behave_bootstrap_environment().await?;
    radula_behave_pkg_config_environment();

    println!(
        "\nPKG_CONFIG_LIBDIR              :: {}",
        env::var(constants::RADULA_ENVIRONMENT_PKG_CONFIG_LIBDIR)?
    );
    println!(
        "PKG_CONFIG_PATH                :: {}",
        env::var(constants::RADULA_ENVIRONMENT_PKG_CONFIG_PATH)?
    );
    println!(
        "PKG_CONFIG_SYSROOT_DIR         :: {}",
        env::var(constants::RADULA_ENVIRONMENT_PKG_CONFIG_SYSROOT_DIR)?
    );
    println!(
        "PKG_CONFIG_SYSTEM_INCLUDE_PATH :: {}",
        env::var(constants::RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_INCLUDE_PATH)?
    );
    println!(
        "PKG_CONFIG_SYSTEM_LIBRARY_PATH :: {}",
        env::var(constants::RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_LIBRARY_PATH)?
    );

    assert!(env::var(constants::RADULA_ENVIRONMENT_PKG_CONFIG_LIBDIR)?
        .ends_with(constants::RADULA_PATH_PKG_CONFIG_LIBDIR_PATH));
    assert!(env::var(constants::RADULA_ENVIRONMENT_PKG_CONFIG_PATH)?
        .ends_with(constants::RADULA_PATH_PKG_CONFIG_LIBDIR_PATH));
    assert!(
        env::var(constants::RADULA_ENVIRONMENT_PKG_CONFIG_SYSROOT_DIR)?
            .ends_with(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
    );
    assert!(
        env::var(constants::RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_INCLUDE_PATH)?
            .ends_with(constants::RADULA_PATH_PKG_CONFIG_SYSTEM_INCLUDE_PATH)
    );
    assert!(
        env::var(constants::RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_LIBRARY_PATH)?
            .ends_with(constants::RADULA_PATH_PKG_CONFIG_SYSTEM_LIBRARY_PATH)
    );

    Ok(())
}
