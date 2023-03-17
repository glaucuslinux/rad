# Copyright (c) 2018-2023, Firas Khalil Khana
# Distributed under the terms of the ISC License

import os

import constants

# This will be used whether we're bootstrapping or not so don't add _bootstrap_ to its name
# Also, this must be used after `radula_behave_architecture_environment` is run
proc radula_behave_flags_environment*() =
    putEnv(RADULA_ENVIRONMENT_FLAGS_C_COMPILER, RADULA_FLAGS_C_COMPILER & " " &
        getEnv(RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS))
    putEnv(RADULA_ENVIRONMENT_FLAGS_CXX_COMPILER, getEnv(
        RADULA_ENVIRONMENT_FLAGS_C_COMPILER) & " " & RADULA_FLAGS_CXX_COMPILER)
    putEnv(RADULA_ENVIRONMENT_FLAGS_LINKER, RADULA_FLAGS_LINKER & " " &
        getEnv(RADULA_ENVIRONMENT_FLAGS_C_COMPILER))