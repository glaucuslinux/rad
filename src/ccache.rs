// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::error::Error;
use std::path::Path;

use super::constants;

//
// Ccache Function
//

pub fn radula_behave_ccache_environment() -> Result<(), Box<dyn Error>> {
    env::set_var(
        constants::RADULA_ENVIRONMENT_PATH,
        Path::new(&[constants::RADULA_PATH_CCACHE, ":"].concat()).join(
            env::var(constants::RADULA_ENVIRONMENT_PATH)?
                .strip_prefix(constants::RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)
                .unwrap(),
        ),
    );

    Ok(())
}
