use std::env;
use std::path::Path;
use std::process::Command;

use super::paths;

pub fn radula_behave_fetch() {}
pub fn radula_behave_swallow() {}

pub fn radula_behave_source(x: &str) {
    let ceras: [&str; 8] = [
        "Name",
        "Version",
        "Commit",
        "Source",
        "sha512sum",
        "Cysts",
        "Concentrates",
        "Licenses",
    ];

    // Magic
    // - Sources `ceras` file and prints its variables
    // - Enumerates over the output
    // - Formats the output
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
    .to_string()
    .trim()
    .split("~~")
    .into_iter()
    .enumerate()
    {
        println!("{:<12} :: {}", ceras[i], j.replace(" ", ", "));
    }
}
