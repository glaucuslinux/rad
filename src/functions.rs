use std::env;
use std::path::Path;
use std::process::{Command, Stdio};
use std::string::String;

use super::paths;

pub fn radula_behave_fetch(x: &'static str) {
    // Receive the variables from the `ceras` file
    let y: [String; 8] = radula_behave_source(x);

    // Get the path of the source directory
    let z = String::from(
        Path::new(&env::var("SRCD").unwrap())
            .join(&y[0])
            .to_str()
            .unwrap(),
    );

    let w = String::from(
        Path::new(&z)
            .join(Path::new(&y[3]).file_name().unwrap())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
    );

    if radula_behave_source(x)[1] == paths::RADULA_GIT {
        // Clone the `git` repo
        Command::new(paths::RADULA_GIT)
            .args(&["clone", &y[3], &z])
            .spawn();
        // Checkout the freshly cloned `git` repo at the specified commit number
        Command::new(paths::RADULA_GIT)
            .args(&["-C", &z, "checkout", &y[2]])
            .spawn();
    } else {
        Command::new(paths::RADULA_MKDIR)
            .args(&[paths::RADULA_MKDIR_FLAGS, &z])
            .stdout(Stdio::null())
            .spawn();

        Command::new(paths::RADULA_CURL)
            .args(&["-Lo", &w, &y[3]])
            .spawn();

        Command::new(paths::RADULA_TAR)
            .args(&["xvf", &w, "-C", &z])
            .stdout(Stdio::null())
            .spawn();
    }
}

pub fn radula_behave_swallow() {}

// Sources the `ceras` file and returns an array of strings representing the
// variables inside of it
pub fn radula_behave_source(x: &'static str) -> [String; 8] {
    // We can't have a `"".to_string()` copied 8 times, which is why we're using
    // a constant beforehand
    const S: String = String::new();
    let mut y: [String; 8] = [S; 8];

    // Magic
    for (i, j) in String::from_utf8_lossy(
        &Command::new(paths::RADULA_SHELL)
            .args(&[
                paths::RADULA_SHELL_FLAGS,
                &format!(
                    ". {} && echo $nom~~$ver~~$cmt~~$url~~$sum~~$cys~~$cnt~~$lic",
                    Path::new(&env::var("CERD").unwrap())
                        .join(x)
                        .join("ceras")
                        .to_str()
                        .unwrap(),
                ),
            ])
            .output()
            .unwrap()
            .stdout,
    )
    .trim()
    .split("~~")
    .enumerate()
    {
        y[i] = j.to_owned();
    }

    return y;
}

pub fn radula_behave_verify() {}
