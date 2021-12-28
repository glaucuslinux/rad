use std::fs;
use std::path::Path;

use super::constants;

use serde::Deserialize;

// The variable `nom` is mandatory, and the rest are optional
#[derive(Deserialize)]
pub struct Ceras {
    pub nom: String,
    pub ver: Option<String>,
    pub cmt: Option<String>,
    pub url: Option<String>,
    pub sum: Option<String>,
    pub cnt: Option<String>,
    pub cys: Option<String>,
}

// Parses the `ceras` file and returns a Ceras struct holding the variables
pub fn radula_behave_ceras_parse(x: &str) -> Ceras {
    return toml::from_str(
        &fs::read_to_string(
            Path::new(constants::RADULA_PATH_CLUSTERS)
                .join(&x)
                .join(constants::RADULA_CERAS),
        )
        .unwrap(),
    )
    .unwrap();
}

// Checks if the `ceras` file exists
pub fn radula_behave_ceras_exists(x: &str) -> bool {
    return Path::new(constants::RADULA_PATH_CLUSTERS)
        .join(&x)
        .join(constants::RADULA_CERAS)
        .is_file();
}
