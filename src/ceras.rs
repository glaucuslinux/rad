// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

use super::constants;

use colored::Colorize;
use serde::Deserialize;
use tokio::fs;

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

impl fmt::Display for Ceras {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{:13}{:3}{}",
            "Name".bold(),
            "::".bold(),
            self.nom.blue()
        );
        writeln!(
            f,
            "{:13}{:3}{}",
            "Version".bold(),
            "::".bold(),
            [
                &self.ver.as_ref().unwrap_or(&"None".red().to_string()),
                " ",
                &self.cmt.as_ref().unwrap_or(&" ".to_string())
            ]
            .concat()
            .trim(),
        );
        writeln!(
            f,
            "{:13}{:3}{}",
            "URL".bold(),
            "::".bold(),
            self.url.as_ref().unwrap_or(&"None".red().to_string())
        );
        writeln!(
            f,
            "{:13}{:3}{}",
            "Checksum".bold(),
            "::".bold(),
            self.sum.as_ref().unwrap_or(&"None".red().to_string())
        );
        writeln!(
            f,
            "{:13}{:3}{}",
            "Concentrates".bold(),
            "::".bold(),
            self.cnt.as_ref().unwrap_or(&"None".red().to_string())
        );
        writeln!(
            f,
            "{:13}{:3}{}",
            "Cysts".bold(),
            "::".bold(),
            self.cys.as_ref().unwrap_or(&"None".red().to_string())
        )
    }
}

// Checks if the `ceras` file exists
pub async fn radula_behave_ceras_exist(name: &str) -> Result<bool, Box<dyn Error>> {
    let exist = radula_behave_ceras_path(&name).await?.is_file();

    Ok(exist)
}

// Parses the `ceras` file and returns a Ceras struct holding the variables
pub async fn radula_behave_ceras_parse(name: &str) -> Result<Ceras, Box<dyn Error>> {
    let parse = toml::from_str(&fs::read_to_string(radula_behave_ceras_path(&name).await?).await?)?;

    Ok(parse)
}

// Returns the full path to the `ceras` file
pub async fn radula_behave_ceras_path(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let path = Path::new(constants::RADULA_PATH_RADULA_CLUSTERS)
        .join(constants::RADULA_DIRECTORY_GLAUCUS)
        .join(&name)
        .join(constants::RADULA_CERAS);

    Ok(path)
}

// Return the full path to the `ceras` source tarball
pub async fn radula_behave_ceras_source(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let source = Path::new(constants::RADULA_PATH_RADULA_SOURCES).join(&name);

    Ok(source)
}
