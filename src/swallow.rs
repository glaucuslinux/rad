// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use super::ceras;
use super::constants;

use colored::Colorize;
use git2::Repository;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;

// Fetch, verify and extract ceras's source
pub fn radula_behave_swallow(x: &'static str) {
    // Receive the variables from the `ceras` file
    let y = ceras::radula_behave_ceras_parse(x);

    println!("{} swallow", "::".bold());

    // Get the path of the source directory
    let z = &String::from(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_SOURCES).unwrap())
            .join(&y.nom)
            .to_str()
            .unwrap(),
    );

    let w = String::from(
        Path::new(z)
            .join(Path::new(&y.url.as_ref().unwrap()).file_name().unwrap())
            .to_str()
            .unwrap(),
    );

    // Verify ceras's source tarball
    let radula_behave_verify = || {
        write!(
            Command::new(constants::RADULA_TOOTH_CHECKSUM)
                .arg(constants::RADULA_TOOTH_CHECKSUM_FLAGS)
                .stdin(Stdio::piped())
                .stdout(Stdio::null())
                .spawn()
                .unwrap()
                .stdin
                .unwrap(),
            "{}",
            [&y.sum.unwrap(), " ", &w].concat()
        )
        .unwrap();
    };

    if !Path::is_dir(Path::new(z)) {
        if y.ver.unwrap() == constants::RADULA_TOOTH_GIT {
            // Clone ceras's source `git` repository and checkout the freshly cloned `git`
            // repository at the specified commit number
            let repo = Repository::clone(&y.url.unwrap(), z).unwrap();
            repo.checkout_tree(&repo.revparse_ext(&y.cmt.unwrap()).unwrap().0, None)
                .unwrap();
        } else {
            fs::create_dir(z).unwrap();

            radula_behave_verify();

            // Extract ceras's source tarball
            Command::new(constants::RADULA_TOOTH_TAR)
                .args(&["xpvf", &w, "-C", z])
                .stdout(Stdio::null())
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }
    } else if y.ver.unwrap() != constants::RADULA_TOOTH_GIT {
        // Only verify existing ceras's source tarballs without extracting them again
        radula_behave_verify();
    }
}
