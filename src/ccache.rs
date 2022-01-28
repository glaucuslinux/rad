// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::error::Error;

use super::constants;

//
// Ccache Function
//

pub fn radula_behave_ccache_environment() -> Result<(), Box<dyn Error>> {
    env::set_var(
        constants::RADULA_ENVIRONMENT_PATH,
        env::join_paths(
            env::split_paths(constants::RADULA_PATH_CCACHE).chain(env::split_paths(&env::var(
                constants::RADULA_ENVIRONMENT_PATH,
            )?)),
        )
        .unwrap(),
    );

    Ok(())
}

#[test]
fn test_radula_behave_ccache_environment() -> Result<(), Box<dyn Error>> {
    radula_behave_ccache_environment();

    println!(
        "\nPATH :: {}\n",
        env::var(constants::RADULA_ENVIRONMENT_PATH)?
    );

    assert!(
        env::var(constants::RADULA_ENVIRONMENT_PATH)?.starts_with(constants::RADULA_PATH_CCACHE)
    );

    Ok(())
}
