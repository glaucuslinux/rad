// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::error::Error;
use std::process::exit;

use super::architecture;
use super::ccache;
use super::ceras;
use super::clean;
use super::constants;
use super::flags;
use super::functions;
use super::help;
use super::pkg_config;
// use super::image;

pub async fn radula_options() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);

    if args.len() < 1 {
        help::radula_help(constants::RADULA_HELP)?;
        exit(1);
    }

    while let Some(y) = args.next().as_deref() {
        match y {
            // `.unwrap_or_default()` actually returns an empty string literal which gets matched
            // to `_` in the below switch (if we had `.unwrap_or("h")` then that'll return the help
            // message without exiting with an error status of `1`).
            "-b" | "--behave" => match args.next().as_deref().unwrap_or_default() {
                "b" | "bootstrap" => {
                    match args.next().as_deref().unwrap_or_default() {
                        "c" | "clean" => {
                            functions::radula_behave_bootstrap_environment().await?;

                            functions::radula_behave_bootstrap_toolchain_environment();

                            functions::radula_behave_bootstrap_cross_environment_directories();

                            clean::radula_behave_bootstrap_clean().await?;

                            println!("clean complete");
                        }
                        "d" | "distclean" => {
                            functions::radula_behave_bootstrap_environment().await?;

                            functions::radula_behave_bootstrap_toolchain_environment();

                            functions::radula_behave_bootstrap_cross_environment_directories();

                            clean::radula_behave_bootstrap_distclean().await?;

                            println!("distclean complete");
                        }
                        "h" | "help" => help::radula_help(constants::RADULA_HELP_BEHAVE_BOOTSTRAP)?,
                        "i" | "image" => {
                            functions::radula_behave_bootstrap_environment().await?;

                            // image::radula_behave_bootstrap_cross_image();

                            println!("image complete");
                        }
                        "l" | "list" => {
                            help::radula_help(constants::RADULA_HELP_BEHAVE_BOOTSTRAP_LIST)?
                        }
                        "r" | "require" => {
                            println!("Checking if host has all required packages...")
                        }
                        "s" | "release" => {
                            functions::radula_behave_bootstrap_environment().await?;

                            functions::radula_behave_bootstrap_toolchain_release();

                            println!("release complete");
                        }
                        "t" | "toolchain" => {
                            functions::radula_behave_bootstrap_environment().await?;

                            functions::radula_behave_teeth_environment();

                            ccache::radula_behave_ccache_environment();

                            architecture::radula_behave_bootstrap_architecture_environment(
                                constants::RADULA_ARCHITECTURE_X86_64_V3,
                            )
                            .await?;

                            functions::radula_behave_bootstrap_toolchain_environment();

                            // Needed for clean to work...
                            functions::radula_behave_bootstrap_cross_environment_directories();

                            clean::radula_behave_bootstrap_clean().await?;

                            functions::radula_behave_bootstrap_initialize().await?;

                            functions::radula_behave_bootstrap_toolchain_prepare();
                            functions::radula_behave_bootstrap_toolchain_construct();
                            functions::radula_behave_bootstrap_toolchain_backup();

                            println!("toolchain complete");
                        }
                        "x" | "cross" => {
                            functions::radula_behave_bootstrap_environment().await?;

                            functions::radula_behave_teeth_environment();

                            pkg_config::radula_behave_pkg_config_environment()?;

                            architecture::radula_behave_bootstrap_architecture_environment(
                                constants::RADULA_ARCHITECTURE_X86_64_V3,
                            )
                            .await?;

                            flags::radula_behave_flags_environment();

                            functions::radula_behave_bootstrap_cross_environment_directories();
                            functions::radula_behave_bootstrap_cross_environment_teeth();

                            functions::radula_behave_bootstrap_cross_prepare().await?;
                            functions::radula_behave_bootstrap_cross_construct();
                            //radula_behave_bootstrap_cross_strip();

                            println!("cross complete");
                        }
                        _ => {
                            help::radula_help(constants::RADULA_HELP_BEHAVE_BOOTSTRAP)?;
                            exit(1);
                        }
                    }
                    exit(0);
                }
                // `exit` should be removed to allow dealing with multiple cerata simultaneously
                "e" | "envenomate" => {
                    match args.next().as_deref().unwrap_or_default() {
                        "h" | "help" => {
                            help::radula_help(constants::RADULA_HELP_BEHAVE_ENVENOMATE)?
                        }
                        _ => {
                            help::radula_help(constants::RADULA_HELP_BEHAVE_ENVENOMATE)?;
                            exit(1);
                        }
                    }
                    exit(0);
                }
                // `exit` should be removed to allow dealing with multiple cerata simultaneously
                "i" | "binary" => {
                    match args.next().as_deref().unwrap_or_default() {
                        "h" | "help" => help::radula_help(constants::RADULA_HELP_BEHAVE_BINARY)?,
                        _ => {
                            help::radula_help(constants::RADULA_HELP_BEHAVE_BINARY)?;
                            exit(1);
                        }
                    }
                    exit(0);
                }
                "h" | "help" => {
                    help::radula_help(constants::RADULA_HELP_BEHAVE)?;
                    exit(0);
                }
                _ => {
                    help::radula_help(constants::RADULA_HELP_BEHAVE)?;
                    exit(1);
                }
            },
            "-c" | "--ceras" => {
                while let Some(z) = args.next().as_deref() {
                    println!("{}", ceras::radula_behave_ceras_parse(&z).await?);
                }
                exit(0);
            }
            "-h" | "--help" => {
                help::radula_help(constants::RADULA_HELP)?;
                exit(0);
            }
            "-v" | "--version" => {
                println!("{}", constants::RADULA_HELP_VERSION);
                exit(0);
            }
            _ => {
                help::radula_help(constants::RADULA_HELP)?;
                exit(1);
            }
        }
    }

    Ok(())
}
