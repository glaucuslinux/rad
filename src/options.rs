use std::env;
use std::process::exit;

use super::architecture;
use super::ceras;
use super::clean;
use super::constants;
use super::flags;
use super::functions;
use super::image;

use colored::Colorize;

pub fn radula_options() {
    let mut x = env::args().skip(1);

    if x.len() < 1 {
        functions::radula_help(constants::RADULA_HELP);
        exit(1);
    }

    while let Some(y) = x.next().as_deref() {
        match y {
            // `.unwrap_or_default()` actually returns an empty string literal which gets matched
            // to `_` in the below switch (if we had `.unwrap_or("h")` then that'll return the help
            // message without exiting with an error status of `1`).
            "-b" | "--behave" => match x.next().as_deref().unwrap_or_default() {
                "b" | "bootstrap" => {
                    match x.next().as_deref().unwrap_or_default() {
                        "c" | "clean" => {
                            functions::radula_behave_bootstrap_environment();

                            functions::radula_behave_bootstrap_toolchain_environment();

                            functions::radula_behave_bootstrap_cross_environment_directories();

                            clean::radula_behave_bootstrap_clean();

                            println!("clean complete");
                        }
                        "d" | "distclean" => {
                            functions::radula_behave_bootstrap_environment();

                            functions::radula_behave_bootstrap_toolchain_environment();

                            functions::radula_behave_bootstrap_cross_environment_directories();

                            clean::radula_behave_bootstrap_distclean();

                            println!("distclean complete");
                        }
                        "h" | "help" => {
                            functions::radula_help(constants::RADULA_HELP_BEHAVE_BOOTSTRAP)
                        }
                        "i" | "image" => {
                            functions::radula_behave_bootstrap_environment();

                            image::radula_behave_bootstrap_cross_image();

                            println!("image complete");
                        }
                        "l" | "list" => {
                            functions::radula_help(constants::RADULA_HELP_BEHAVE_BOOTSTRAP_LIST)
                        }
                        "r" | "require" => {
                            println!("Checking if host has all required packages...")
                        }
                        "s" | "release" => {
                            functions::radula_behave_bootstrap_environment();

                            functions::radula_behave_bootstrap_toolchain_release();

                            println!("release complete");
                        }
                        "t" | "toolchain" => {
                            functions::radula_behave_bootstrap_environment();

                            functions::radula_behave_teeth_environment();

                            functions::radula_behave_ccache_environment();

                            architecture::radula_behave_bootstrap_architecture_environment(
                                constants::RADULA_ARCHITECTURE_X86_64_V3,
                            );

                            functions::radula_behave_bootstrap_toolchain_environment();

                            // Needed for clean to work...
                            functions::radula_behave_bootstrap_cross_environment_directories();

                            clean::radula_behave_bootstrap_clean();

                            functions::radula_behave_bootstrap_initialize();

                            functions::radula_behave_bootstrap_toolchain_prepare();
                            functions::radula_behave_bootstrap_toolchain_construct();
                            functions::radula_behave_bootstrap_toolchain_backup();

                            println!("toolchain complete");
                        }
                        "x" | "cross" => {
                            functions::radula_behave_bootstrap_environment();

                            functions::radula_behave_teeth_environment();

                            functions::radula_behave_pkg_config_environment();

                            architecture::radula_behave_bootstrap_architecture_environment(
                                constants::RADULA_ARCHITECTURE_X86_64_V3,
                            );

                            flags::radula_behave_flags_environment();

                            functions::radula_behave_bootstrap_cross_environment_directories();
                            functions::radula_behave_bootstrap_cross_environment_teeth();

                            functions::radula_behave_bootstrap_cross_prepare();
                            functions::radula_behave_bootstrap_cross_construct();
                            //radula_behave_bootstrap_cross_strip();

                            println!("cross complete");
                        }
                        _ => {
                            functions::radula_help(constants::RADULA_HELP_BEHAVE_BOOTSTRAP);
                            exit(1);
                        }
                    }
                    return;
                }
                // `return` should be removed to allow dealing with multiple cerata simultaneously
                "e" | "envenomate" => {
                    match x.next().as_deref().unwrap_or_default() {
                        "h" | "help" => {
                            functions::radula_help(constants::RADULA_HELP_BEHAVE_ENVENOMATE)
                        }
                        _ => {
                            functions::radula_help(constants::RADULA_HELP_BEHAVE_ENVENOMATE);
                            exit(1);
                        }
                    }
                    return;
                }
                // `return` should be removed to allow dealing with multiple cerata simultaneously
                "i" | "binary" => {
                    match x.next().as_deref().unwrap_or_default() {
                        "h" | "help" => {
                            functions::radula_help(constants::RADULA_HELP_BEHAVE_BINARY)
                        }
                        _ => {
                            functions::radula_help(constants::RADULA_HELP_BEHAVE_BINARY);
                            exit(1);
                        }
                    }
                    return;
                }
                "h" | "help" => {
                    functions::radula_help(constants::RADULA_HELP_BEHAVE);
                    return;
                }
                _ => {
                    functions::radula_help(constants::RADULA_HELP_BEHAVE);
                    exit(1);
                }
            },
            "-c" | "--ceras" => {
                while let Some(z) = x.next().as_deref() {
                    let w = ceras::radula_behave_ceras_parse(&z);

                    println!("{:13}{:3}{}", "Name".bold(), "::".bold(), w.nom.blue());
                    println!(
                        "{:13}{:3}{}",
                        "Version".bold(),
                        "::".bold(),
                        [
                            &w.ver.unwrap_or("None".red().to_string().into()),
                            " ",
                            &w.cmt.unwrap_or_default()
                        ]
                        .concat()
                        .trim()
                    );
                    println!(
                        "{:13}{:3}{}",
                        "URL".bold(),
                        "::".bold(),
                        w.url.unwrap_or("None".red().to_string().into())
                    );
                    println!(
                        "{:13}{:3}{}",
                        "Checksum".bold(),
                        "::".bold(),
                        w.sum.unwrap_or("None".red().to_string().into())
                    );
                    println!(
                        "{:13}{:3}{}",
                        "Concentrates".bold(),
                        "::".bold(),
                        w.cnt.unwrap_or("None".red().to_string().into())
                    );
                    println!(
                        "{:13}{:3}{}",
                        "Cysts".bold(),
                        "::".bold(),
                        w.cys.unwrap_or("None".red().to_string().into())
                    );

                    println!("");
                }
                return;
            }
            "-h" | "--help" => {
                functions::radula_help(constants::RADULA_HELP);
                return;
            }
            "-v" | "--version" => {
                println!("{}", constants::RADULA_HELP_VERSION);
                return;
            }
            _ => {
                functions::radula_help(constants::RADULA_HELP);
                exit(1);
            }
        }
    }
}
