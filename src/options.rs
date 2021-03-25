use std::env;
use std::process;

use super::constants;
use super::functions;

pub fn radula_options() {
    let mut x = env::args().skip(1);

    if x.len() < 1 {
        functions::radula_open(constants::RADULA_HELP);
        process::exit(1);
    }

    while let Some(option) = x.next() {
        match option.as_str() {
            "b" | "-b" | "--behave" => match x.next().as_deref() {
                Some("b") | Some("bootstrap") => match x.next().as_deref() {
                    Some("c") | Some("clean") => println!("Do nothing"),
                    Some("d") | Some("distclean") => println!("Do nothing"),

                    Some("h") | Some("-h") | Some("--help") => {
                        functions::radula_open(constants::RADULA_BEHAVE_BOOTSTRAP_HELP)
                    }

                    Some("i") | Some("image") => println!("Do nothing"),
                    Some("l") | Some("list") => println!("Do nothing"),
                    Some("r") | Some("require") => println!("Do nothing"),
                    Some("s") | Some("release") => println!("Do nothing"),
                    Some("t") | Some("toolchain") => println!("Do nothing"),
                    Some("x") | Some("cross") => println!("Do nothing"),
                    _ => {
                        functions::radula_open(constants::RADULA_BEHAVE_BOOTSTRAP_HELP);
                        process::exit(1);
                    }
                },
                Some("e") | Some("envenomate") => match x.next().as_deref() {
                    Some("h") | Some("-h") | Some("--help") => {
                        functions::radula_open(constants::RADULA_BEHAVE_ENVENOMATE_HELP);
                    }

                    _ => {
                        functions::radula_open(constants::RADULA_BEHAVE_ENVENOMATE_HELP);
                        process::exit(1);
                    }
                },
                Some("i") | Some("binary") => match x.next().as_deref() {
                    Some("h") | Some("-h") | Some("--help") => {
                        functions::radula_open(constants::RADULA_BEHAVE_BINARY_HELP);
                    }

                    _ => {
                        functions::radula_open(constants::RADULA_BEHAVE_BINARY_HELP);
                        process::exit(1);
                    }
                },

                Some("h") | Some("-h") | Some("--help") => {
                    functions::radula_open(constants::RADULA_BEHAVE_HELP)
                }

                _ => {
                    functions::radula_open(constants::RADULA_BEHAVE_HELP);
                    process::exit(1);
                }
            },
            "c" | "-c" | "--ceras" => match x.next().as_deref() {
                Some("n") | Some("nom") | Some("name") => println!("Do nothing"),

                Some("h") | Some("-h") | Some("--help") => {
                    functions::radula_open(constants::RADULA_CERAS_HELP)
                }

                Some("v") | Some("ver") | Some("version") => println!("Do nothing"),
                Some("u") | Some("url") | Some("source") => println!("Do nothing"),
                Some("s") | Some("sum") | Some("checksum") | Some("sha512sum") => {
                    println!("Do nothing")
                }
                Some("y") | Some("cys") | Some("cyst") | Some("cysts") => println!("Do nothing"),
                Some("c") | Some("cnt") | Some("concentrate") | Some("concentrates") => {
                    println!("Do nothing")
                }
                _ => {
                    functions::radula_open(constants::RADULA_CERAS_HELP);
                    process::exit(1);
                }
            },
            "h" | "-h" | "--help" => functions::radula_open(constants::RADULA_HELP),

            "v" | "-v" | "--version" => println!("{}", constants::RADULA_VERSION),
            _ => {
                functions::radula_open(constants::RADULA_HELP);
                process::exit(1);
            }
        }
    }
}
