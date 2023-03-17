# Copyright (c) 2018-2023, Firas Khalil Khana
# Distributed under the terms of the ISC License

import os

import constants

#
# Ccache Functions
#

proc radula_behave_bootstrap_cross_ccache*() =
    putEnv(RADULA_ENVIRONMENT_CCACHE_CONFIGURATION,
        RADULA_PATH_RADULA_CLUSTERS / RADULA_DIRECTORY_GLAUCUS /
        RADULA_CERAS_CCACHE / RADULA_FILE_CCACHE_CONFIGURATION)
    putEnv(RADULA_ENVIRONMENT_CCACHE_DIRECTORY, getEnv(
        RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY) / RADULA_CERAS_CCACHE)
    putEnv(RADULA_ENVIRONMENT_PATH, getEnv(
        RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN) / RADULA_PATH_USR /
        RADULA_PATH_LIB / RADULA_CERAS_CCACHE / getEnv(RADULA_ENVIRONMENT_PATH))