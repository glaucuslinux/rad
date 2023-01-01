// Copyright (c) 2018-2023, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::error::Error;
use std::path::Path;

use super::constants;

use tokio::process::Command;

//
// Architecture Functions
//

// These will be used whether we're bootstrapping or not so don't add _bootstrap_ to its name
// Get canonical system tuple using the `config.guess` file
async fn radula_behave_architecture_tuple() -> Result<String, Box<dyn Error>> {
    let tuple = String::from(
        String::from_utf8_lossy(
            &Command::new(
                Path::new(constants::RADULA_PATH_RADULA_CLUSTERS)
                    .join(constants::RADULA_DIRECTORY_GLAUCUS)
                    .join(constants::RADULA_CERAS_BINUTILS)
                    .join(constants::RADULA_FILE_CONFIG_GUESS),
            )
            .output()
            .await?
            .stdout,
        )
        .trim(),
    );

    Ok(tuple)
}

pub async fn radula_behave_architecture_environment(
    architecture: &'static str,
) -> Result<(), Box<dyn Error>> {
    env::set_var(
        constants::RADULA_ENVIRONMENT_TUPLE_BUILD,
        radula_behave_architecture_tuple().await?,
    );

    env::set_var(constants::RADULA_ENVIRONMENT_ARCHITECTURE, architecture);
    env::set_var(
        constants::RADULA_ENVIRONMENT_ARCHITECTURE_CERATA,
        [constants::RADULA_ARCHITECTURE_CERATA, architecture].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TUPLE_TARGET,
        [architecture, constants::RADULA_ARCHITECTURE_TUPLE_TARGET].concat(),
    );

    match architecture {
        constants::RADULA_ARCHITECTURE_AARCH64 => {
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_CERATA,
                [
                    constants::RADULA_ARCHITECTURE_CERATA,
                    constants::RADULA_ARCHITECTURE_AARCH64_CERATA,
                ]
                .concat(),
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS,
                constants::RADULA_ARCHITECTURE_AARCH64_FLAGS,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION,
                constants::RADULA_ARCHITECTURE_AARCH64_GCC_CONFIGURATION,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX,
                constants::RADULA_ARCHITECTURE_AARCH64_LINUX,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION,
                "",
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE,
                constants::RADULA_ARCHITECTURE_AARCH64_LINUX_IMAGE,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL,
                architecture,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL_LINKER,
                architecture,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_UCONTEXT,
                architecture,
            );
        }
        constants::RADULA_ARCHITECTURE_RISCV64 => {
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_CERATA,
                [
                    constants::RADULA_ARCHITECTURE_CERATA,
                    constants::RADULA_ARCHITECTURE_RISCV64_CERATA,
                ]
                .concat(),
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS,
                constants::RADULA_ARCHITECTURE_RISCV64_FLAGS,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION,
                constants::RADULA_ARCHITECTURE_RISCV64_GCC_CONFIGURATION,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX,
                constants::RADULA_ARCHITECTURE_RISCV64_LINUX,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION,
                "",
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE,
                constants::RADULA_ARCHITECTURE_RISCV64_LINUX_IMAGE,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL,
                architecture,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL_LINKER,
                architecture,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_UCONTEXT,
                architecture,
            );
        }
        constants::RADULA_ARCHITECTURE_X86_64_V3 => {
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS,
                constants::RADULA_ARCHITECTURE_X86_64_V3_FLAGS,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION,
                constants::RADULA_ARCHITECTURE_X86_64_V3_GCC_CONFIGURATION,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX,
                constants::RADULA_ARCHITECTURE_X86_64_V3_LINUX,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION,
                constants::RADULA_ARCHITECTURE_X86_64_V3_LINUX_CONFIGURATION,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE,
                constants::RADULA_ARCHITECTURE_X86_64_V3_LINUX_IMAGE,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL,
                constants::RADULA_ARCHITECTURE_X86_64_V3_LINUX,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL_LINKER,
                constants::RADULA_ARCHITECTURE_X86_64_V3_LINUX,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_TUPLE_TARGET,
                [
                    constants::RADULA_ARCHITECTURE_X86_64_V3_LINUX,
                    constants::RADULA_ARCHITECTURE_TUPLE_TARGET,
                ]
                .concat(),
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_UCONTEXT,
                constants::RADULA_ARCHITECTURE_X86_64_V3_LINUX,
            );
        }
        _ => {}
    }
    Ok(())
}

// To prevent a race condition between tests use `cargo test -- --test-threads 1`
#[tokio::test]
async fn test_radula_behave_architecture_tuple() -> Result<(), Box<dyn Error>> {
    println!("\n");

    assert_eq!(
        radula_behave_architecture_tuple().await?,
        "x86_64-pc-linux-gnu"
    );

    Ok(())
}

#[tokio::test]
async fn test_radula_behave_architecture_environment_aarch64() -> Result<(), Box<dyn Error>> {
    radula_behave_architecture_environment("aarch64").await?;

    // BLD
    //
    // Should be evaluated once and doesn't matter if it's first or last because
    // the build machine won't change, but should be after a call to
    // architecture_environment
    println!(
        "\nBLD    :: {}\n",
        env::var(constants::RADULA_ENVIRONMENT_TUPLE_BUILD)?
    );

    println!(
        "ARCH   :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE)?
    );
    println!(
        "CARCH  :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_CERATA)?
    );
    println!(
        "FARCH  :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS)?
    );
    println!(
        "GCARCH :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION)?
    );
    println!(
        "LARCH  :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX)?
    );
    println!(
        "LCARCH :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION)?
    );
    println!(
        "LIARCH :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE)?
    );
    println!(
        "MARCH  :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL)?
    );
    println!(
        "MLARCH :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL_LINKER)?
    );
    println!(
        "TGT    :: {}",
        env::var(constants::RADULA_ENVIRONMENT_TUPLE_TARGET)?
    );
    println!(
        "UARCH  :: {}\n",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_UCONTEXT)?
    );

    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_TUPLE_BUILD)?,
        radula_behave_architecture_tuple().await?
    );

    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE)?,
        "aarch64"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_CERATA)?,
        "--with-gcc-arch=armv8-a"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS)?,
        "-mabi=lp64 -mfix-cortex-a53-835769 -mfix-cortex-a53-843419 -march=armv8-a -mtune=generic"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION)?,
        "--with-arch=armv8-a --with-abi=lp64 --enable-fix-cortex-a53-835769 --enable-fix-cortex-a53-843419"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX)?,
        "arm64"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION)?,
        ""
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE)?,
        "arch/arm64/boot/Image"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL)?,
        "aarch64"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL_LINKER)?,
        "aarch64"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_TUPLE_TARGET)?,
        "aarch64-glaucus-linux-musl"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_UCONTEXT)?,
        "aarch64"
    );

    Ok(())
}

#[tokio::test]
async fn test_radula_behave_architecture_environment_riscv64() -> Result<(), Box<dyn Error>> {
    radula_behave_architecture_environment("riscv64").await?;

    println!(
        "\nARCH   :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE)?
    );
    println!(
        "CARCH  :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_CERATA)?
    );
    println!(
        "FARCH  :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS)?
    );
    println!(
        "GCARCH :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION)?
    );
    println!(
        "LARCH  :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX)?
    );
    println!(
        "LCARCH :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION)?
    );
    println!(
        "LIARCH :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE)?
    );
    println!(
        "MARCH  :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL)?
    );
    println!(
        "MLARCH :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL_LINKER)?
    );
    println!(
        "TGT    :: {}",
        env::var(constants::RADULA_ENVIRONMENT_TUPLE_TARGET)?
    );
    println!(
        "UARCH  :: {}\n",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_UCONTEXT)?
    );

    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE)?,
        "riscv64"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_CERATA)?,
        "--with-gcc-arch=rv64gc"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS)?,
        "-mabi=lp64d -march=rv64gc -mcpu=sifive-u74 -mtune=sifive-7-series -mcmodel=medany"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION)?,
        "--with-cpu=sifive-u74 --with-arch=rv64gc --with-tune=sifive-7-series --with-abi=lp64d"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX)?,
        "riscv"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION)?,
        ""
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE)?,
        "arch/riscv/boot/Image"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL)?,
        "riscv64"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL_LINKER)?,
        "riscv64"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_TUPLE_TARGET)?,
        "riscv64-glaucus-linux-musl"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_UCONTEXT)?,
        "riscv64"
    );

    Ok(())
}

#[tokio::test]
async fn test_radula_behave_architecture_environment_x86_64_v3() -> Result<(), Box<dyn Error>> {
    radula_behave_architecture_environment("x86-64").await?;

    println!(
        "\nARCH   :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE)?
    );
    println!(
        "CARCH  :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_CERATA)?
    );
    println!(
        "FARCH  :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS)?
    );
    println!(
        "GCARCH :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION)?
    );
    println!(
        "LARCH  :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX)?
    );
    println!(
        "LCARCH :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION)?
    );
    println!(
        "LIARCH :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE)?
    );
    println!(
        "MARCH  :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL)?
    );
    println!(
        "MLARCH :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL_LINKER)?
    );
    println!(
        "TGT    :: {}",
        env::var(constants::RADULA_ENVIRONMENT_TUPLE_TARGET)?
    );
    println!(
        "UARCH  :: {}\n",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_UCONTEXT)?
    );

    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE)?,
        "x86-64"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_CERATA)?,
        "--with-gcc-arch=x86-64"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS)?,
        "-march=x86-64-v3 -mtune=generic -mfpmath=sse -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION)?,
        "--with-arch=x86-64-v3 --with-tune=generic"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX)?,
        "x86_64"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION)?,
        "x86_64_"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE)?,
        "arch/x86/boot/bzImage"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL)?,
        "x86_64"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL_LINKER)?,
        "x86_64"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_TUPLE_TARGET)?,
        "x86_64-glaucus-linux-musl"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_UCONTEXT)?,
        "x86_64"
    );

    Ok(())
}
