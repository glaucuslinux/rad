# Copyright (c) 2018-2023, Firas Khalil Khana
# Distributed under the terms of the ISC License

import std/[
    os,
    osproc,
    strutils
]

import constants

#
# Architecture Functions
#

# These will be used whether we're bootstrapping or not so don't add _bootstrap_ to its name
# Get canonical system tuple using the `config.guess` file
proc radula_behave_architecture_tuple*(): (string, int) =
    execCmdEx(RADULA_PATH_RADULA_CLUSTERS / RADULA_DIRECTORY_GLAUCUS / RADULA_CERAS_BINUTILS / RADULA_FILE_CONFIG_GUESS)

proc radula_behave_architecture_environment*(architecture: string) =
    putEnv(RADULA_ENVIRONMENT_TUPLE_BUILD, radula_behave_architecture_tuple()[0].strip())

    putEnv(RADULA_ENVIRONMENT_ARCHITECTURE, architecture)
    putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_CERATA, RADULA_ARCHITECTURE_CERATA & architecture)
    putEnv(RADULA_ENVIRONMENT_TUPLE_TARGET, architecture & RADULA_ARCHITECTURE_TUPLE_TARGET)

    case architecture
    of RADULA_ARCHITECTURE_AARCH64:
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_CERATA, RADULA_ARCHITECTURE_CERATA & RADULA_ARCHITECTURE_AARCH64_CERATA)
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS, RADULA_ARCHITECTURE_AARCH64_FLAGS)
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION, RADULA_ARCHITECTURE_AARCH64_GCC_CONFIGURATION)
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_LINUX, RADULA_ARCHITECTURE_AARCH64_LINUX)
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION, "")
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE, RADULA_ARCHITECTURE_AARCH64_LINUX_IMAGE)
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_MUSL, architecture)
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_MUSL_LINKER, architecture)
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_UCONTEXT, architecture)
    of RADULA_ARCHITECTURE_RISCV64:
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_CERATA, RADULA_ARCHITECTURE_CERATA & RADULA_ARCHITECTURE_RISCV64_CERATA)
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS, RADULA_ARCHITECTURE_RISCV64_FLAGS)
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION, RADULA_ARCHITECTURE_RISCV64_GCC_CONFIGURATION)
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_LINUX, RADULA_ARCHITECTURE_RISCV64_LINUX)
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION, "")
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE, RADULA_ARCHITECTURE_RISCV64_LINUX_IMAGE)
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_MUSL, architecture)
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_MUSL_LINKER, architecture)
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_UCONTEXT, architecture)
    of RADULA_ARCHITECTURE_X86_64_V3:
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS, RADULA_ARCHITECTURE_X86_64_V3_FLAGS)
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION, RADULA_ARCHITECTURE_X86_64_V3_GCC_CONFIGURATION)
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_LINUX, RADULA_ARCHITECTURE_X86_64_V3_LINUX)
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION, RADULA_ARCHITECTURE_X86_64_V3_LINUX_CONFIGURATION)
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE, RADULA_ARCHITECTURE_X86_64_V3_LINUX_IMAGE)
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_MUSL, RADULA_ARCHITECTURE_X86_64_V3_LINUX)
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_MUSL_LINKER, RADULA_ARCHITECTURE_X86_64_V3_LINUX)
        putEnv(RADULA_ENVIRONMENT_TUPLE_TARGET, RADULA_ARCHITECTURE_X86_64_V3_LINUX & RADULA_ARCHITECTURE_TUPLE_TARGET)
        putEnv(RADULA_ENVIRONMENT_ARCHITECTURE_UCONTEXT, RADULA_ARCHITECTURE_X86_64_V3_LINUX)
