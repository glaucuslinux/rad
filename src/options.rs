// Copyright (c) 2018-2021, Firas Khalil Khana
// Distributed under the terms of the ISC License

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

    while let Some(y) = x.next().as_deref() {
        match y {
            // `.unwrap_or_default()` actually returns an empty string literal which gets matched
            // to `_` in the below switch (if we had `.unwrap_or("h")` then that'll return the help
            // message without exiting with an error status of `1`).
            "b" | "-b" | "--behave" => match x.next().as_deref().unwrap_or_default() {
                "b" | "bootstrap" => match x.next().as_deref().unwrap_or_default() {
                    "c" | "clean" => {
                        functions::radula_behave_bootstrap_environment();

                        functions::radula_behave_bootstrap_toolchain_environment();

                        functions::radula_behave_bootstrap_cross_environment();

                        functions::radula_behave_bootstrap_clean();

                        println!("clean complete");
                    }
                    "d" | "distclean" => {
                        functions::radula_behave_bootstrap_environment();

                        functions::radula_behave_bootstrap_toolchain_environment();

                        functions::radula_behave_bootstrap_cross_environment();

                        functions::radula_behave_bootstrap_distclean();

                        println!("distclean complete");
                    }

                    "h" | "-h" | "--help" => {
                        functions::radula_open(constants::RADULA_HELP_BEHAVE_BOOTSTRAP)
                    }

                    "i" | "image" => println!("Do nothing"),
                    "l" | "list" => {
                        functions::radula_open(constants::RADULA_HELP_BEHAVE_BOOTSTRAP_LIST)
                    }
                    "r" | "require" => {
                        println!("Checking if host has all required packages...")
                    }
                    "s" | "release" => println!("release complete"),
                    "t" | "toolchain" => {
                        functions::radula_behave_bootstrap_environment();

                        functions::radula_behave_ccache_environment();

                        functions::radula_behave_bootstrap_initialize();

                        // Only including cross environment for clean to work
                        functions::radula_behave_bootstrap_cross_environment();

                        functions::radula_behave_bootstrap_toolchain_environment();

                        functions::radula_behave_bootstrap_clean();

                        functions::radula_behave_bootstrap_architecture_environment(
                            constants::RADULA_ARCHITECTURE_X86_64,
                        );

                        functions::radula_behave_bootstrap_toolchain_swallow();
                        functions::radula_behave_bootstrap_toolchain_prepare();
                        functions::radula_behave_bootstrap_toolchain_construct();
                        functions::radula_behave_bootstrap_toolchain_backup();
                    }
                    "x" | "cross" => {
                        functions::radula_behave_bootstrap_environment();

                        functions::radula_behave_pkg_config_environment();

                        functions::radula_behave_bootstrap_architecture_environment(
                            constants::RADULA_ARCHITECTURE_X86_64,
                        );

                        functions::radula_behave_flags_environment();

                        functions::radula_behave_bootstrap_cross_environment();
                        functions::radula_behave_bootstrap_cross_swallow();
                        functions::radula_behave_bootstrap_cross_prepare();

                        functions::radula_behave_bootstrap_cross_construct();

                        functions::radula_behave_bootstrap_cross_strip();
                    }
                    _ => {
                        functions::radula_open(constants::RADULA_HELP_BEHAVE_BOOTSTRAP);
                        process::exit(1);
                    }
                },
                "e" | "envenomate" => match x.next().as_deref().unwrap_or_default() {
                    "h" | "-h" | "--help" => {
                        functions::radula_open(constants::RADULA_HELP_BEHAVE_ENVENOMATE)
                    }

                    _ => {
                        functions::radula_open(constants::RADULA_HELP_BEHAVE_ENVENOMATE);
                        process::exit(1);
                    }
                },
                "i" | "binary" => match x.next().as_deref().unwrap_or_default() {
                    "h" | "-h" | "--help" => {
                        functions::radula_open(constants::RADULA_HELP_BEHAVE_BINARY)
                    }

                    _ => {
                        functions::radula_open(constants::RADULA_HELP_BEHAVE_BINARY);
                        process::exit(1);
                    }
                },

                "h" | "-h" | "--help" => functions::radula_open(constants::RADULA_HELP_BEHAVE),

                _ => {
                    functions::radula_open(constants::RADULA_HELP_BEHAVE);
                    process::exit(1);
                }
            },
            "c" | "-c" | "--ceras" => match x.next().as_deref().unwrap_or_default() {
                "n" | "nom" | "name" => println!("Do nothing"),

                "h" | "-h" | "--help" => functions::radula_open(constants::RADULA_HELP_CERAS),

                "v" | "ver" | "version" => println!("Do nothing"),
                "u" | "url" | "source" => println!("Do nothing"),
                "s" | "sum" | "checksum" | "sha512sum" => {
                    println!("Do nothing")
                }
                "y" | "cys" | "cyst" | "cysts" => println!("Do nothing"),
                "c" | "cnt" | "concentrate" | "concentrates" => {
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
