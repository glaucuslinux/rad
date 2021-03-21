use std::path::Path;
use std::process::Command;
use std::string;
use std::{env, ffi::OsStr};

use super::paths;

pub fn radula_behave_fetch(x: &'static str) {
    let y: [string::String; 8] = radula_behave_source(x);
    if radula_behave_source(x)[1] == "git" {
        Command::new(paths::RADULA_GIT)
            .arg("clone")
            .arg(y[3].as_str())
            .spawn();
    }
}

pub fn radula_behave_swallow() {}

// Sources the `ceras` file and returns an array of strings representing the
// variables inside of it
pub fn radula_behave_source(x: &'static str) -> [string::String; 8] {
    // We can't have a `"".to_string()` copied 8 times, which is why we're using
    // a constant beforehand
    const S: String = String::new();
    let mut y: [string::String; 8] = [S; 8];

    // Magic
    for (i, j) in std::string::String::from_utf8_lossy(
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
        y[i] = j.to_string();
    }

    return y;
}
