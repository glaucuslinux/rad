// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::error::Error;

#[cfg(test)]
use super::architecture;
use super::constants;

// This will be used whether we're bootstrapping or not so don't add _bootstrap_ to its name
// Also, this must be used after `radula_behave_architecture_environment` is run
pub fn radula_behave_flags_environment() -> Result<(), Box<dyn Error>> {
    env::set_var(
        constants::RADULA_ENVIRONMENT_FLAGS_C_COMPILER,
        [
            constants::RADULA_FLAGS_C_COMPILER,
            " ",
            &env::var(constants::RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS)?,
        ]
        .concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_FLAGS_CXX_COMPILER,
        [
            &env::var(constants::RADULA_ENVIRONMENT_FLAGS_C_COMPILER)?,
            " ",
            constants::RADULA_FLAGS_CXX_COMPILER,
        ]
        .concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_FLAGS_LINKER,
        [
            constants::RADULA_FLAGS_LINKER,
            " ",
            &env::var(constants::RADULA_ENVIRONMENT_FLAGS_C_COMPILER)?,
        ]
        .concat(),
    );

    Ok(())
}

#[tokio::test]
async fn test_radula_behave_flags_environment_aarch64() -> Result<(), Box<dyn Error>> {
    architecture::radula_behave_architecture_environment("aarch64").await?;

    radula_behave_flags_environment()?;

    println!(
        "\nCFLAGS   :: {}",
        env::var(constants::RADULA_ENVIRONMENT_FLAGS_C_COMPILER)?
    );
    println!(
        "CXXFLAGS :: {}",
        env::var(constants::RADULA_ENVIRONMENT_FLAGS_CXX_COMPILER)?
    );
    println!(
        "LDFLAGS  :: {}\n",
        env::var(constants::RADULA_ENVIRONMENT_FLAGS_LINKER)?
    );

    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_FLAGS_C_COMPILER)?, "-pipe -g0 -Ofast -fomit-frame-pointer -fmerge-all-constants -fmodulo-sched -fmodulo-sched-allow-regmoves -fgcse-sm -fgcse-las -fdevirtualize-at-ltrans -fira-loop-pressure -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -floop-parallelize-all -fvariable-expansion-in-unroller -falign-functions=32 -flimit-function-alignment -flto=auto -flto-compression-level=19 -fuse-linker-plugin -ftracer -funroll-loops -ffunction-sections -fdata-sections -fno-stack-protector -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -mabi=lp64 -mfix-cortex-a53-835769 -mfix-cortex-a53-843419 -march=armv8-a -mtune=generic");
    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_FLAGS_CXX_COMPILER)?, "-pipe -g0 -Ofast -fomit-frame-pointer -fmerge-all-constants -fmodulo-sched -fmodulo-sched-allow-regmoves -fgcse-sm -fgcse-las -fdevirtualize-at-ltrans -fira-loop-pressure -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -floop-parallelize-all -fvariable-expansion-in-unroller -falign-functions=32 -flimit-function-alignment -flto=auto -flto-compression-level=19 -fuse-linker-plugin -ftracer -funroll-loops -ffunction-sections -fdata-sections -fno-stack-protector -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -mabi=lp64 -mfix-cortex-a53-835769 -mfix-cortex-a53-843419 -march=armv8-a -mtune=generic -fno-rtti -fvisibility-inlines-hidden -fvisibility=hidden");
    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_FLAGS_LINKER)?, "-Wl,--strip-all -Wl,-z,noexecstack,-z,now,-z,relro -Wl,--as-needed -Wl,--gc-sections -Wl,--sort-common -Wl,--hash-style=gnu -pipe -g0 -Ofast -fomit-frame-pointer -fmerge-all-constants -fmodulo-sched -fmodulo-sched-allow-regmoves -fgcse-sm -fgcse-las -fdevirtualize-at-ltrans -fira-loop-pressure -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -floop-parallelize-all -fvariable-expansion-in-unroller -falign-functions=32 -flimit-function-alignment -flto=auto -flto-compression-level=19 -fuse-linker-plugin -ftracer -funroll-loops -ffunction-sections -fdata-sections -fno-stack-protector -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -mabi=lp64 -mfix-cortex-a53-835769 -mfix-cortex-a53-843419 -march=armv8-a -mtune=generic");

    Ok(())
}

#[tokio::test]
async fn test_radula_behave_flags_environment_armv6zk() -> Result<(), Box<dyn Error>> {
    architecture::radula_behave_architecture_environment("armv6zk").await?;

    radula_behave_flags_environment()?;

    println!(
        "\nCFLAGS   :: {}",
        env::var(constants::RADULA_ENVIRONMENT_FLAGS_C_COMPILER)?
    );
    println!(
        "CXXFLAGS :: {}",
        env::var(constants::RADULA_ENVIRONMENT_FLAGS_CXX_COMPILER)?
    );
    println!(
        "LDFLAGS  :: {}\n",
        env::var(constants::RADULA_ENVIRONMENT_FLAGS_LINKER)?
    );

    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_FLAGS_C_COMPILER)?, "-pipe -g0 -Ofast -fomit-frame-pointer -fmerge-all-constants -fmodulo-sched -fmodulo-sched-allow-regmoves -fgcse-sm -fgcse-las -fdevirtualize-at-ltrans -fira-loop-pressure -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -floop-parallelize-all -fvariable-expansion-in-unroller -falign-functions=32 -flimit-function-alignment -flto=auto -flto-compression-level=19 -fuse-linker-plugin -ftracer -funroll-loops -ffunction-sections -fdata-sections -fno-stack-protector -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -mabi=aapcs-linux -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s -mcpu=arm1176jzf-s -mfpu=vfpv2 -mtls-dialect=gnu2");
    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_FLAGS_CXX_COMPILER)?, "-pipe -g0 -Ofast -fomit-frame-pointer -fmerge-all-constants -fmodulo-sched -fmodulo-sched-allow-regmoves -fgcse-sm -fgcse-las -fdevirtualize-at-ltrans -fira-loop-pressure -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -floop-parallelize-all -fvariable-expansion-in-unroller -falign-functions=32 -flimit-function-alignment -flto=auto -flto-compression-level=19 -fuse-linker-plugin -ftracer -funroll-loops -ffunction-sections -fdata-sections -fno-stack-protector -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -mabi=aapcs-linux -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s -mcpu=arm1176jzf-s -mfpu=vfpv2 -mtls-dialect=gnu2 -fno-rtti -fvisibility-inlines-hidden -fvisibility=hidden");
    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_FLAGS_LINKER)?, "-Wl,--strip-all -Wl,-z,noexecstack,-z,now,-z,relro -Wl,--as-needed -Wl,--gc-sections -Wl,--sort-common -Wl,--hash-style=gnu -pipe -g0 -Ofast -fomit-frame-pointer -fmerge-all-constants -fmodulo-sched -fmodulo-sched-allow-regmoves -fgcse-sm -fgcse-las -fdevirtualize-at-ltrans -fira-loop-pressure -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -floop-parallelize-all -fvariable-expansion-in-unroller -falign-functions=32 -flimit-function-alignment -flto=auto -flto-compression-level=19 -fuse-linker-plugin -ftracer -funroll-loops -ffunction-sections -fdata-sections -fno-stack-protector -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -mabi=aapcs-linux -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s -mcpu=arm1176jzf-s -mfpu=vfpv2 -mtls-dialect=gnu2");

    Ok(())
}

#[tokio::test]
async fn test_radula_behave_flags_environment_i686() -> Result<(), Box<dyn Error>> {
    architecture::radula_behave_architecture_environment("i686").await?;

    radula_behave_flags_environment()?;

    println!(
        "\nCFLAGS   :: {}",
        env::var(constants::RADULA_ENVIRONMENT_FLAGS_C_COMPILER)?
    );
    println!(
        "CXXFLAGS :: {}",
        env::var(constants::RADULA_ENVIRONMENT_FLAGS_CXX_COMPILER)?
    );
    println!(
        "LDFLAGS  :: {}\n",
        env::var(constants::RADULA_ENVIRONMENT_FLAGS_LINKER)?
    );

    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_FLAGS_C_COMPILER)?, "-pipe -g0 -Ofast -fomit-frame-pointer -fmerge-all-constants -fmodulo-sched -fmodulo-sched-allow-regmoves -fgcse-sm -fgcse-las -fdevirtualize-at-ltrans -fira-loop-pressure -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -floop-parallelize-all -fvariable-expansion-in-unroller -falign-functions=32 -flimit-function-alignment -flto=auto -flto-compression-level=19 -fuse-linker-plugin -ftracer -funroll-loops -ffunction-sections -fdata-sections -fno-stack-protector -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -march=i686 -mtune=generic -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2");
    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_FLAGS_CXX_COMPILER)?, "-pipe -g0 -Ofast -fomit-frame-pointer -fmerge-all-constants -fmodulo-sched -fmodulo-sched-allow-regmoves -fgcse-sm -fgcse-las -fdevirtualize-at-ltrans -fira-loop-pressure -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -floop-parallelize-all -fvariable-expansion-in-unroller -falign-functions=32 -flimit-function-alignment -flto=auto -flto-compression-level=19 -fuse-linker-plugin -ftracer -funroll-loops -ffunction-sections -fdata-sections -fno-stack-protector -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -march=i686 -mtune=generic -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2 -fno-rtti -fvisibility-inlines-hidden -fvisibility=hidden");
    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_FLAGS_LINKER)?, "-Wl,--strip-all -Wl,-z,noexecstack,-z,now,-z,relro -Wl,--as-needed -Wl,--gc-sections -Wl,--sort-common -Wl,--hash-style=gnu -pipe -g0 -Ofast -fomit-frame-pointer -fmerge-all-constants -fmodulo-sched -fmodulo-sched-allow-regmoves -fgcse-sm -fgcse-las -fdevirtualize-at-ltrans -fira-loop-pressure -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -floop-parallelize-all -fvariable-expansion-in-unroller -falign-functions=32 -flimit-function-alignment -flto=auto -flto-compression-level=19 -fuse-linker-plugin -ftracer -funroll-loops -ffunction-sections -fdata-sections -fno-stack-protector -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -march=i686 -mtune=generic -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2");

    Ok(())
}

#[tokio::test]
async fn test_radula_behave_flags_environment_riscv64() -> Result<(), Box<dyn Error>> {
    architecture::radula_behave_architecture_environment("riscv64").await?;

    radula_behave_flags_environment()?;

    println!(
        "\nCFLAGS   :: {}",
        env::var(constants::RADULA_ENVIRONMENT_FLAGS_C_COMPILER)?
    );
    println!(
        "CXXFLAGS :: {}",
        env::var(constants::RADULA_ENVIRONMENT_FLAGS_CXX_COMPILER)?
    );
    println!(
        "LDFLAGS  :: {}\n",
        env::var(constants::RADULA_ENVIRONMENT_FLAGS_LINKER)?
    );

    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_FLAGS_C_COMPILER)?, "-pipe -g0 -Ofast -fomit-frame-pointer -fmerge-all-constants -fmodulo-sched -fmodulo-sched-allow-regmoves -fgcse-sm -fgcse-las -fdevirtualize-at-ltrans -fira-loop-pressure -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -floop-parallelize-all -fvariable-expansion-in-unroller -falign-functions=32 -flimit-function-alignment -flto=auto -flto-compression-level=19 -fuse-linker-plugin -ftracer -funroll-loops -ffunction-sections -fdata-sections -fno-stack-protector -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -mabi=lp64d -march=rv64gc -mcpu=sifive-u74 -mtune=sifive-7-series -mcmodel=medany");
    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_FLAGS_CXX_COMPILER)?, "-pipe -g0 -Ofast -fomit-frame-pointer -fmerge-all-constants -fmodulo-sched -fmodulo-sched-allow-regmoves -fgcse-sm -fgcse-las -fdevirtualize-at-ltrans -fira-loop-pressure -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -floop-parallelize-all -fvariable-expansion-in-unroller -falign-functions=32 -flimit-function-alignment -flto=auto -flto-compression-level=19 -fuse-linker-plugin -ftracer -funroll-loops -ffunction-sections -fdata-sections -fno-stack-protector -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -mabi=lp64d -march=rv64gc -mcpu=sifive-u74 -mtune=sifive-7-series -mcmodel=medany -fno-rtti -fvisibility-inlines-hidden -fvisibility=hidden");
    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_FLAGS_LINKER)?, "-Wl,--strip-all -Wl,-z,noexecstack,-z,now,-z,relro -Wl,--as-needed -Wl,--gc-sections -Wl,--sort-common -Wl,--hash-style=gnu -pipe -g0 -Ofast -fomit-frame-pointer -fmerge-all-constants -fmodulo-sched -fmodulo-sched-allow-regmoves -fgcse-sm -fgcse-las -fdevirtualize-at-ltrans -fira-loop-pressure -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -floop-parallelize-all -fvariable-expansion-in-unroller -falign-functions=32 -flimit-function-alignment -flto=auto -flto-compression-level=19 -fuse-linker-plugin -ftracer -funroll-loops -ffunction-sections -fdata-sections -fno-stack-protector -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -mabi=lp64d -march=rv64gc -mcpu=sifive-u74 -mtune=sifive-7-series -mcmodel=medany");

    Ok(())
}

#[tokio::test]
async fn test_radula_behave_flags_environment_x86_64_v3() -> Result<(), Box<dyn Error>> {
    architecture::radula_behave_architecture_environment("x86-64").await?;

    radula_behave_flags_environment()?;

    println!(
        "\nCFLAGS   :: {}",
        env::var(constants::RADULA_ENVIRONMENT_FLAGS_C_COMPILER)?
    );
    println!(
        "CXXFLAGS :: {}",
        env::var(constants::RADULA_ENVIRONMENT_FLAGS_CXX_COMPILER)?
    );
    println!(
        "LDFLAGS  :: {}\n",
        env::var(constants::RADULA_ENVIRONMENT_FLAGS_LINKER)?
    );

    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_FLAGS_C_COMPILER)?, "-pipe -g0 -Ofast -fomit-frame-pointer -fmerge-all-constants -fmodulo-sched -fmodulo-sched-allow-regmoves -fgcse-sm -fgcse-las -fdevirtualize-at-ltrans -fira-loop-pressure -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -floop-parallelize-all -fvariable-expansion-in-unroller -falign-functions=32 -flimit-function-alignment -flto=auto -flto-compression-level=19 -fuse-linker-plugin -ftracer -funroll-loops -ffunction-sections -fdata-sections -fno-stack-protector -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -march=x86-64-v3 -mtune=generic -mfpmath=sse -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2");
    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_FLAGS_CXX_COMPILER)?, "-pipe -g0 -Ofast -fomit-frame-pointer -fmerge-all-constants -fmodulo-sched -fmodulo-sched-allow-regmoves -fgcse-sm -fgcse-las -fdevirtualize-at-ltrans -fira-loop-pressure -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -floop-parallelize-all -fvariable-expansion-in-unroller -falign-functions=32 -flimit-function-alignment -flto=auto -flto-compression-level=19 -fuse-linker-plugin -ftracer -funroll-loops -ffunction-sections -fdata-sections -fno-stack-protector -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -march=x86-64-v3 -mtune=generic -mfpmath=sse -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2 -fno-rtti -fvisibility-inlines-hidden -fvisibility=hidden");
    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_FLAGS_LINKER)?, "-Wl,--strip-all -Wl,-z,noexecstack,-z,now,-z,relro -Wl,--as-needed -Wl,--gc-sections -Wl,--sort-common -Wl,--hash-style=gnu -pipe -g0 -Ofast -fomit-frame-pointer -fmerge-all-constants -fmodulo-sched -fmodulo-sched-allow-regmoves -fgcse-sm -fgcse-las -fdevirtualize-at-ltrans -fira-loop-pressure -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -floop-parallelize-all -fvariable-expansion-in-unroller -falign-functions=32 -flimit-function-alignment -flto=auto -flto-compression-level=19 -fuse-linker-plugin -ftracer -funroll-loops -ffunction-sections -fdata-sections -fno-stack-protector -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -march=x86-64-v3 -mtune=generic -mfpmath=sse -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2");

    Ok(())
}
