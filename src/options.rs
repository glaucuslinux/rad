// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::error::Error;
use std::process::exit;

use super::architecture;
use super::bootstrap;
use super::ccache;
use super::ceras;
use super::clean;
use super::constants;
use super::cross;
use super::flags;
use super::help;
use super::img;
use super::iso;
use super::pkg_config;
use super::teeth;
use super::toolchain;

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
                            bootstrap::radula_behave_bootstrap_environment().await?;

                            toolchain::radula_behave_bootstrap_toolchain_environment()?;

                            cross::radula_behave_bootstrap_cross_environment_directories()?;

                            clean::radula_behave_bootstrap_clean().await?;

                            println!("clean complete");
                        }
                        "d" | "distclean" => {
                            bootstrap::radula_behave_bootstrap_environment().await?;

                            toolchain::radula_behave_bootstrap_toolchain_environment()?;

                            cross::radula_behave_bootstrap_cross_environment_directories()?;

                            clean::radula_behave_bootstrap_distclean().await?;

                            println!("distclean complete");
                        }
                        "h" | "help" => help::radula_help(constants::RADULA_HELP_BEHAVE_BOOTSTRAP)?,
                        "i" | "img" => {
                            bootstrap::radula_behave_bootstrap_environment().await?;

                            img::radula_behave_bootstrap_cross_img().await?;

                            println!("img complete");
                        }
                        "l" | "list" => {
                            help::radula_help(constants::RADULA_HELP_BEHAVE_BOOTSTRAP_LIST)?
                        }
                        "r" | "require" => {
                            println!("Checking if host has all required packages...")
                        }
                        "s" | "release" => {
                            bootstrap::radula_behave_bootstrap_environment().await?;

                            toolchain::radula_behave_bootstrap_toolchain_release().await?;

                            println!("release complete");
                        }
                        "t" | "toolchain" => {
                            bootstrap::radula_behave_bootstrap_environment().await?;

                            teeth::radula_behave_teeth_environment()?;

                            architecture::radula_behave_architecture_environment(
                                constants::RADULA_ARCHITECTURE_X86_64_V3,
                            )
                            .await?;

                            toolchain::radula_behave_bootstrap_toolchain_environment()?;

                            // Needed for clean to work...
                            cross::radula_behave_bootstrap_cross_environment_directories()?;

                            clean::radula_behave_bootstrap_clean().await?;

                            bootstrap::radula_behave_bootstrap_initialize().await?;

                            ccache::radula_behave_bootstrap_toolchain_ccache().await?;

                            toolchain::radula_behave_bootstrap_toolchain_prepare().await?;
                            toolchain::radula_behave_bootstrap_toolchain_construct().await?;
                            toolchain::radula_behave_bootstrap_toolchain_backup().await?;

                            println!("toolchain complete");
                        }
                        "x" | "cross" => {
                            bootstrap::radula_behave_bootstrap_environment().await?;

                            teeth::radula_behave_teeth_environment()?;

                            architecture::radula_behave_architecture_environment(
                                constants::RADULA_ARCHITECTURE_X86_64_V3,
                            )
                            .await?;

                            flags::radula_behave_flags_environment()?;

                            cross::radula_behave_bootstrap_cross_environment_directories()?;
                            cross::radula_behave_bootstrap_cross_environment_teeth()?;

                            ccache::radula_behave_bootstrap_cross_ccache().await?;

                            pkg_config::radula_behave_pkg_config_environment()?;

                            cross::radula_behave_bootstrap_cross_prepare().await?;
                            cross::radula_behave_bootstrap_cross_construct().await?;

                            println!("cross complete");
                        }
                        "z" | "iso" => {
                            bootstrap::radula_behave_bootstrap_environment().await?;

                            iso::radula_behave_bootstrap_cross_iso().await?;

                            println!("iso complete");
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
