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

    while let Some(y) = x.next() {
        match y.as_str() {
            "b" | "-b" | "--behave" => match x.next().as_deref() {
                Some("b") | Some("bootstrap") => match x.next().as_deref() {
                    Some("c") | Some("clean") => {
                        functions::radula_behave_bootstrap_environment();

                        functions::radula_behave_bootstrap_toolchain_environment();

                        functions::radula_behave_bootstrap_cross_environment();

                        functions::radula_behave_bootstrap_clean();
                    }
                    Some("d") | Some("distclean") => {
                        functions::radula_behave_bootstrap_environment();

                        functions::radula_behave_bootstrap_toolchain_environment();

                        functions::radula_behave_bootstrap_cross_environment();

                        functions::radula_behave_bootstrap_distclean();
                    }

                    Some("h") | Some("-h") | Some("--help") => {
                        functions::radula_open(constants::RADULA_HELP_BEHAVE_BOOTSTRAP)
                    }

                    Some("i") | Some("image") => println!("Do nothing"),
                    Some("l") | Some("list") => {
                        functions::radula_open(constants::RADULA_HELP_BEHAVE_BOOTSTRAP_LIST)
                    }
                    Some("r") | Some("require") => println!("Do nothing"),
                    Some("s") | Some("release") => println!("Do nothing"),
                    Some("t") | Some("toolchain") => {
                        functions::radula_behave_bootstrap_environment();

                        functions::radula_behave_ccache_environment();

                        functions::radula_behave_bootstrap_initialize();

                        functions::radula_behave_bootstrap_clean();

                        functions::radula_behave_bootstrap_arch_environment("x86-64");

                        functions::radula_behave_bootstrap_toolchain();

                        functions::radula_behave_bootstrap_toolchain_backup();
                    }
                    Some("x") | Some("cross") => println!("Do nothing"),
                    _ => {
                        functions::radula_open(constants::RADULA_HELP_BEHAVE_BOOTSTRAP);
                        process::exit(1);
                    }
                },
                Some("e") | Some("envenomate") => match x.next().as_deref() {
                    Some("h") | Some("-h") | Some("--help") => {
                        functions::radula_open(constants::RADULA_HELP_BEHAVE_ENVENOMATE)
                    }

                    _ => {
                        functions::radula_open(constants::RADULA_HELP_BEHAVE_ENVENOMATE);
                        process::exit(1);
                    }
                },
                Some("i") | Some("binary") => match x.next().as_deref() {
                    Some("h") | Some("-h") | Some("--help") => {
                        functions::radula_open(constants::RADULA_HELP_BEHAVE_BINARY)
                    }

                    _ => {
                        functions::radula_open(constants::RADULA_HELP_BEHAVE_BINARY);
                        process::exit(1);
                    }
                },

                Some("h") | Some("-h") | Some("--help") => {
                    functions::radula_open(constants::RADULA_HELP_BEHAVE)
                }

                _ => {
                    functions::radula_open(constants::RADULA_HELP_BEHAVE);
                    process::exit(1);
                }
            },
            "c" | "-c" | "--ceras" => match x.next().as_deref() {
                Some("n") | Some("nom") | Some("name") => println!("Do nothing"),

                Some("h") | Some("-h") | Some("--help") => {
                    functions::radula_open(constants::RADULA_HELP_CERAS)
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
                    functions::radula_open(constants::RADULA_HELP_CERAS);
                    process::exit(1);
                }
            },

            "h" | "-h" | "--help" => functions::radula_open(constants::RADULA_HELP),

            "v" | "-v" | "--version" => println!("{}", constants::RADULA_HELP_VERSION),
            _ => {
                functions::radula_open(constants::RADULA_HELP);
                process::exit(1);
            }
        }
    }
}
