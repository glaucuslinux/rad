# Copyright (c) 2018-2023, Firas Khalil Khana
# Distributed under the terms of the ISC License

import std/[
  os,
  osproc,
  strutils
]

import constants

# This will be used regardless of the bootstrap behavior, so don't add `_bootstrap_`
# to its name. Also, ensure `radula_behave_genome_environment()` is run beforehand.
proc radula_behave_flags_environment*() =
  putEnv(RADULA_ENVIRONMENT_FLAGS_C_COMPILER, RADULA_FLAGS_C_CXX_COMPILER)
  putEnv(RADULA_ENVIRONMENT_FLAGS_CXX_COMPILER, getEnv(RADULA_ENVIRONMENT_FLAGS_C_COMPILER) & ' ' & RADULA_FLAGS_C_CXX_COMPILER)
  putEnv(RADULA_ENVIRONMENT_FLAGS_LINKER, RADULA_FLAGS_LINKER & ' ' & getEnv(RADULA_ENVIRONMENT_FLAGS_C_COMPILER))

# Return the canonical system tuple from `config.guess`
proc radula_behave_genome_tuple*(): (string, int) =
  execCmdEx(RADULA_PATH_RADULA_CLUSTERS_GLAUCUS / RADULA_CERAS_BINUTILS / RADULA_FILE_CONFIG_GUESS)

proc radula_behave_genome_environment*(genome: string, stage = RADULA_DIRECTORY_CROSS) =
  putEnv(RADULA_ENVIRONMENT_GENOME, genome)
  putEnv(RADULA_ENVIRONMENT_GENOME_CERATA, RADULA_GENOME_CERATA & genome)
  putEnv(RADULA_ENVIRONMENT_GENOME_GCC_CONFIGURATION, RADULA_GENOME_X86_64_V3_GCC_CONFIGURATION)
  putEnv(RADULA_ENVIRONMENT_GENOME_LINUX, RADULA_GENOME_X86_64_V3_LINUX)
  putEnv(RADULA_ENVIRONMENT_GENOME_LINUX_CONFIGURATION, RADULA_GENOME_X86_64_V3_LINUX_CONFIGURATION)
  putEnv(RADULA_ENVIRONMENT_GENOME_LINUX_IMAGE, RADULA_GENOME_X86_64_V3_LINUX_IMAGE)
  putEnv(RADULA_ENVIRONMENT_GENOME_MUSL, RADULA_GENOME_X86_64_V3_LINUX)
  putEnv(RADULA_ENVIRONMENT_GENOME_MUSL_LINKER, RADULA_GENOME_X86_64_V3_LINUX)

  putEnv(RADULA_ENVIRONMENT_TUPLE_BUILD, radula_behave_genome_tuple()[0].strip())
  putEnv(RADULA_ENVIRONMENT_TUPLE_TARGET, RADULA_GENOME_X86_64_V3_LINUX & (if stage == RADULA_DIRECTORY_CROSS: RADULA_GENOME_TUPLE_TARGET_CROSS else: RADULA_GENOME_TUPLE_TARGET_SYSTEM))
