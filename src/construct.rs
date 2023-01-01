// Copyright (c) 2018-2023, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::error::Error;
use std::path::Path;
use std::process::Command;

use super::ceras;
use super::constants;
use super::swallow;

use colored::Colorize;

pub async fn radula_behave_construct(
    name: &'static str,
    stage: &'static str,
) -> Result<(), Box<dyn Error>> {
    // We only require `nom` and `ver` from the `ceras` file
    let ceras = ceras::radula_behave_ceras_parse(name).await?;

    let version = ceras.ver.unwrap_or_default();
    let commit = ceras.cmt.unwrap_or_default();

    println!(
        "{}{}",
        name.blue(),
        [" ", &version, " ", &commit].concat().trim_end()
    );

    // Perform swallow within construct for now (this may not be the best approach for parallelism)
    match name {
        constants::RADULA_CERAS_LIBELF => {
            swallow::radula_behave_swallow(constants::RADULA_CERAS_ELFUTILS).await?;
        }
        // `gcc` will be removed from the list below when dependency resolution is working
        constants::RADULA_CERAS_GCC => {
            swallow::radula_behave_swallow(constants::RADULA_CERAS_GMP).await?;
            swallow::radula_behave_swallow(constants::RADULA_CERAS_MPFR).await?;
            swallow::radula_behave_swallow(constants::RADULA_CERAS_MPC).await?;
            swallow::radula_behave_swallow(constants::RADULA_CERAS_ISL).await?;
            swallow::radula_behave_swallow(name).await?;
        }
        constants::RADULA_CERAS_HYDROSKELETON => {}
        constants::RADULA_CERAS_LIBGCC
        | constants::RADULA_CERAS_LIBGOMP
        | constants::RADULA_CERAS_LIBSTDCXX_V3 => {
            swallow::radula_behave_swallow(constants::RADULA_CERAS_GCC).await?;
        }
        constants::RADULA_CERAS_LINUX_HEADERS => {
            swallow::radula_behave_swallow(constants::RADULA_CERAS_LINUX).await?;
        }
        constants::RADULA_CERAS_LKSH => {
            swallow::radula_behave_swallow(constants::RADULA_CERAS_MKSH).await?;
        }
        constants::RADULA_CERAS_MUSL_HEADERS | constants::RADULA_CERAS_MUSL_UTILS => {
            swallow::radula_behave_swallow(constants::RADULA_CERAS_MUSL).await?;
        }
        _ => swallow::radula_behave_swallow(name).await?,
    }

    println!("{} construct", "::".bold());

    // The `.wait()` is needed to allow `Ctrl + C` to work...
    Command::new(constants::RADULA_CERAS_DASH)
        .args(&[
            constants::RADULA_TOOTH_SHELL_FLAGS,
            &format!(
                // `ceras` and stage files are only using `nom` and `ver`.
                //
                // All basic functions need to be called together to prevent the loss of the
                // current working directory, otherwise we'd have to store it and pass it or `cd`
                // into it whenever a basic function is called.
                "nom={} ver={} . {} && prepare {} && configure {3} && build {3} && check {3} && install {3}",
                name,
                version,
                Path::new(constants::RADULA_PATH_RADULA_CLUSTERS)
                    .join(constants::RADULA_DIRECTORY_GLAUCUS)
                    .join(name)
                    .join(stage)
                    .to_str()
                    .unwrap_or_default(),
                &format!(
                    ">> {} 2>&1",
                    env::var(if stage == constants::RADULA_DIRECTORY_CROSS {
                        constants::RADULA_ENVIRONMENT_FILE_CROSS_LOG
                    } else {
                        constants::RADULA_ENVIRONMENT_FILE_TOOLCHAIN_LOG
                    })?
                )
            )
        ])
        .spawn()?
        .wait()?;

    println!("");

    Ok(())
}
