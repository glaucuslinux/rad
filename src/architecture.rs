// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::error::Error;
use std::path::Path;

use super::constants;

use tokio::process::Command;

// Get canonical system tuple using the `config.guess` file
async fn radula_behave_bootstrap_architecture_tuple() -> Result<String, Box<dyn Error>> {
    let tuple = String::from(
        String::from_utf8_lossy(
            &Command::new(
                Path::new(constants::RADULA_PATH_RADULA_CLUSTERS)
                    .join(constants::RADULA_DIRECTORY_GLAUCUS)
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

pub async fn radula_behave_bootstrap_architecture_environment(
    x: &'static str,
) -> Result<(), Box<dyn Error>> {
    env::set_var(
        constants::RADULA_ENVIRONMENT_TUPLE_BUILD,
        radula_behave_bootstrap_architecture_tuple().await?,
    );

    env::set_var(constants::RADULA_ENVIRONMENT_ARCHITECTURE, x);
    env::set_var(
        constants::RADULA_ENVIRONMENT_ARCHITECTURE_CERATA,
        [constants::RADULA_ARCHITECTURE_CERATA, x].concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TUPLE_TARGET,
        [x, constants::RADULA_ARCHITECTURE_TUPLE_TARGET].concat(),
    );

    match x {
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
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCOMPAT,
                [
                    "-",
                    constants::RADULA_ARCHITECTURE_AARCH64,
                    constants::RADULA_ARCHITECTURE_AARCH64_GCOMPAT,
                ]
                .concat(),
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
            env::set_var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL, x);
            env::set_var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL_LINKER, x);
            env::set_var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_UCONTEXT, x);
        }
        constants::RADULA_ARCHITECTURE_ARMV6ZK => {
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS,
                constants::RADULA_ARCHITECTURE_ARMV6ZK_FLAGS,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION,
                constants::RADULA_ARCHITECTURE_ARMV6ZK_GCC_CONFIGURATION,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCOMPAT,
                [
                    "-",
                    constants::RADULA_ARCHITECTURE_ARMV6ZK_LINUX,
                    constants::RADULA_ARCHITECTURE_ARMV6ZK_MUSL_LINKER,
                    constants::RADULA_ARCHITECTURE_ARMV6ZK_GCOMPAT,
                ]
                .concat(),
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX,
                constants::RADULA_ARCHITECTURE_ARMV6ZK_LINUX,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION,
                constants::RADULA_ARCHITECTURE_ARMV6ZK_LINUX_CONFIGURATION,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE,
                constants::RADULA_ARCHITECTURE_ARMV6ZK_LINUX_IMAGE,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL,
                constants::RADULA_ARCHITECTURE_ARMV6ZK_LINUX,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL_LINKER,
                [
                    constants::RADULA_ARCHITECTURE_ARMV6ZK_LINUX,
                    constants::RADULA_ARCHITECTURE_ARMV6ZK_MUSL_LINKER,
                ]
                .concat(),
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_TUPLE_TARGET,
                [
                    &env::var(constants::RADULA_ENVIRONMENT_TUPLE_TARGET)?,
                    constants::RADULA_ARCHITECTURE_ARMV6ZK_TUPLE_TARGET,
                ]
                .concat(),
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_UCONTEXT,
                constants::RADULA_ARCHITECTURE_ARMV6ZK_LINUX,
            );
        }
        constants::RADULA_ARCHITECTURE_I686 => {
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS,
                constants::RADULA_ARCHITECTURE_I686_FLAGS,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION,
                constants::RADULA_ARCHITECTURE_I686_GCC_CONFIGURATION,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCOMPAT,
                constants::RADULA_ARCHITECTURE_I686_GCOMPAT,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX,
                constants::RADULA_ARCHITECTURE_I686_LINUX,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION,
                constants::RADULA_ARCHITECTURE_I686_LINUX_CONFIGURATION,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE,
                constants::RADULA_ARCHITECTURE_I686_LINUX_IMAGE,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL,
                constants::RADULA_ARCHITECTURE_I686_LINUX,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL_LINKER,
                constants::RADULA_ARCHITECTURE_I686_LINUX,
            );
            env::set_var(
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_UCONTEXT,
                constants::RADULA_ARCHITECTURE_I686_UCONTEXT,
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
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCOMPAT,
                [
                    "-",
                    constants::RADULA_ARCHITECTURE_RISCV64,
                    constants::RADULA_ARCHITECTURE_RISCV64_GCOMPAT,
                ]
                .concat(),
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
            env::set_var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL, x);
            env::set_var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL_LINKER, x);
            env::set_var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_UCONTEXT, x);
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
                constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCOMPAT,
                [
                    "-",
                    constants::RADULA_ARCHITECTURE_X86_64_V3,
                    constants::RADULA_ARCHITECTURE_I686_GCOMPAT,
                ]
                .concat(),
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
                constants::RADULA_ARCHITECTURE_I686_LINUX_IMAGE,
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

#[tokio::test]
async fn test_radula_behave_bootstrap_architecture_tuple() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        radula_behave_bootstrap_architecture_tuple().await?,
        "x86_64-pc-linux-gnu"
    );

    Ok(())
}

#[tokio::test]
async fn test_radula_behave_bootstrap_architecture_environment_aarch64(
) -> Result<(), Box<dyn Error>> {
    radula_behave_bootstrap_architecture_environment("aarch64").await?;

    // BLD
    //
    // Should be evaluated once and doesn't matter if it's first or last because
    // the build machine won't change, but should be after a call to
    // bootstrap_architecture_environment
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
        "GARCH  :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCOMPAT)?
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
        radula_behave_bootstrap_architecture_tuple().await?
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
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCOMPAT)?,
        "-aarch64.so.1"
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
async fn test_radula_behave_bootstrap_architecture_environment_armv6zk(
) -> Result<(), Box<dyn Error>> {
    radula_behave_bootstrap_architecture_environment("armv6zk").await?;

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
        "GARCH  :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCOMPAT)?
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
        "armv6zk"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_CERATA)?,
        "--with-gcc-arch=armv6zk"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS)?,
        "-mabi=aapcs-linux -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s -mcpu=arm1176jzf-s -mfpu=vfpv2 -mtls-dialect=gnu2"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION)?,
        "--with-arch=armv6zk --with-tune=arm1176jzf-s --with-abi=aapcs-linux --with-fpu=vfpv2 --with-float=hard"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCOMPAT)?,
        "-armhf.so.3"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX)?,
        "arm"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION)?,
        "bcm2835_"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE)?,
        "arch/arm/boot/zImage"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL)?,
        "arm"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL_LINKER)?,
        "armhf"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_TUPLE_TARGET)?,
        "armv6zk-glaucus-linux-musleabihf"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_UCONTEXT)?,
        "arm"
    );

    Ok(())
}

#[tokio::test]
async fn test_radula_behave_bootstrap_architecture_environment_i686() -> Result<(), Box<dyn Error>>
{
    radula_behave_bootstrap_architecture_environment("i686").await?;

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
        "GARCH  :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCOMPAT)?
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
        "i686"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_CERATA)?,
        "--with-gcc-arch=i686"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS)?,
        "-march=i686 -mtune=generic -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION)?,
        "--with-arch=i686 --with-tune=generic"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCOMPAT)?,
        ".so.2"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX)?,
        "i386"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION)?,
        "i386_"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE)?,
        "arch/x86/boot/bzImage"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL)?,
        "i386"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_MUSL_LINKER)?,
        "i386"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_TUPLE_TARGET)?,
        "i686-glaucus-linux-musl"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_UCONTEXT)?,
        "x86"
    );

    Ok(())
}

#[tokio::test]
async fn test_radula_behave_bootstrap_architecture_environment_riscv64(
) -> Result<(), Box<dyn Error>> {
    radula_behave_bootstrap_architecture_environment("riscv64").await?;

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
        "GARCH  :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCOMPAT)?
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
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCOMPAT)?,
        "-riscv64-lp64d.so.1"
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
async fn test_radula_behave_bootstrap_architecture_environment_x86_64_v3(
) -> Result<(), Box<dyn Error>> {
    radula_behave_bootstrap_architecture_environment("x86-64").await?;

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
        "GARCH  :: {}",
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCOMPAT)?
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
        env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_GCOMPAT)?,
        "-x86-64.so.2"
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
