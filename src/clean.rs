// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::fs;
use std::path::Path;

use super::constants;

pub fn radula_behave_bootstrap_clean() {
    fs::remove_dir_all(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS).unwrap());

    fs::remove_dir_all(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_LOGS).unwrap());

    fs::remove_dir_all(
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY_BUILDS).unwrap(),
    );

    fs::remove_dir_all(
        &env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY_BUILDS).unwrap(),
    );

    fs::remove_dir_all(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN).unwrap());
}

pub fn radula_behave_bootstrap_distclean() {
    fs::remove_dir_all(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_BACKUPS).unwrap());

    fs::remove_file(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS).unwrap())
            .join(constants::RADULA_FILE_GLAUCUS_IMAGE),
    );

    fs::remove_dir_all(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_SOURCES).unwrap());

    radula_behave_bootstrap_clean();

    // Only remove `RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY` completely after
    // `RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_BUILDS` and
    // `RADULA_ENVIRONMENT_DIRECTORY_CROSS_BUILDS` are removed
    fs::remove_dir_all(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY).unwrap());
}
