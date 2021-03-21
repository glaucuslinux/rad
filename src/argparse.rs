use std::env;
use std::process;

use super::help;

pub fn radula_argparse(radula_genome: &'static str) {
    let mut options = env::args().skip(1);

    if options.len() < 1 {
        help::radula_open(help::RADULA_HELP);
        process::exit(1);
    }

    while let Some(option) = options.next() {
        match option.as_str() {
            "b" | "-b" | "--behave" => match options.next().as_deref() {
                Some("b") | Some("bootstrap") => match options.next().as_deref() {
                    Some("c") | Some("clean") => println!("Do nothing"),
                    Some("d") | Some("distclean") => println!("Do nothing"),

                    Some("h") | Some("-h") | Some("--help") => {
                        help::radula_open(help::RADULA_BEHAVE_BOOTSTRAP_HELP)
                    }

                    Some("i") | Some("image") => println!("Do nothing"),
                    Some("l") | Some("list") => println!("Do nothing"),
                    Some("r") | Some("require") => println!("Do nothing"),
                    Some("s") | Some("release") => println!("Do nothing"),
                    Some("t") | Some("toolchain") => println!("Do nothing"),
                    Some("x") | Some("cross") => println!("Do nothing"),
                    _ => {
                        help::radula_open(help::RADULA_BEHAVE_BOOTSTRAP_HELP);
                        process::exit(1);
                    }
                },
                Some("e") | Some("envenomate") => match options.next().as_deref() {
                    Some("h") | Some("-h") | Some("--help") => {
                        help::radula_open(help::RADULA_BEHAVE_ENVENOMATE_HELP);
                    }

                    _ => {
                        help::radula_open(help::RADULA_BEHAVE_ENVENOMATE_HELP);
                        process::exit(1);
                    }
                },
                Some("i") | Some("binary") => match options.next().as_deref() {
                    Some("h") | Some("-h") | Some("--help") => {
                        help::radula_open(help::RADULA_BEHAVE_BINARY_HELP);
                    }

                    _ => {
                        help::radula_open(help::RADULA_BEHAVE_BINARY_HELP);
                        process::exit(1);
                    }
                },

                Some("h") | Some("-h") | Some("--help") => {
                    help::radula_open(help::RADULA_BEHAVE_HELP)
                }

                _ => {
                    help::radula_open(help::RADULA_BEHAVE_HELP);
                    process::exit(1);
                }
            },
            "c" | "-c" | "--ceras" => match options.next().as_deref() {
                Some("n") | Some("nom") | Some("name") => println!("Do nothing"),

                Some("h") | Some("-h") | Some("--help") => {
                    help::radula_open(help::RADULA_CERAS_HELP)
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
                Some("l") | Some("lic") | Some("license") | Some("licenses") => {
                    println!("Do nothing")
                }
                _ => {
                    help::radula_open(help::RADULA_CERAS_HELP);
                    process::exit(1);
                }
            },
            "g" | "-g" | "--genome" => println!("{}", radula_genome),
            "h" | "-h" | "--help" => help::radula_open(help::RADULA_HELP),

            "v" | "-v" | "--version" => println!("{}", help::RADULA_VERSION),
            _ => {
                help::radula_open(help::RADULA_HELP);
                process::exit(1);
            }
        }
    }
}
