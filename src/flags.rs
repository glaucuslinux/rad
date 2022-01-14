// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;

use super::constants;

// This will be used whether we're bootstrapping or not so don't add _bootstrap_ to its name
pub fn radula_behave_flags_environment() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var(
        constants::RADULA_ENVIRONMENT_FLAGS_C_COMPILER,
        [
            constants::RADULA_FLAGS_C_COMPILER,
            " ",
            &env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS)?,
        ]
        .concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_FLAGS_CXX_COMPILER,
        [
            &env::var(constants::RADULA_ENVIRONMENT_FLAGS_C_COMPILER)?,
            " ",
            constants::RADULA_FLAGS_CXX_COMPILER,
        ]
        .concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_FLAGS_LINKER,
        [
            constants::RADULA_FLAGS_LINKER,
            " ",
            &env::var(constants::RADULA_ENVIRONMENT_FLAGS_C_COMPILER)?,
        ]
        .concat(),
    );

    Ok(())
}
