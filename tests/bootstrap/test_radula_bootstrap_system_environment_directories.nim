# Copyright (c) 2018-2024, Firas Khalil Khana
# Distributed under the terms of the ISC License

import std/os

import
  ../../src/bootstrap,
  ../../src/constants

radula_bootstrap_system_environment_directories()

echo "SRCD  :: ", getEnv(RADULA_ENVIRONMENT_DIRECTORY_CACHE_SOURCES)
echo "VNMD  :: ", getEnv(RADULA_ENVIRONMENT_DIRECTORY_CACHE_VENOM)
echo "CERD  :: ", getEnv(RADULA_ENVIRONMENT_DIRECTORY_CERATA)
echo "LOGD  :: ", getEnv(RADULA_ENVIRONMENT_DIRECTORY_LOGS)

echo ""

echo "SBLD  :: ", getEnv(RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY_SYSTEM_BUILDS)
echo "SSRC  :: ", getEnv(RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY_SYSTEM_SOURCES)

echo ""

echo "SLOG  :: ", getEnv(RADULA_ENVIRONMENT_FILE_SYSTEM_LOG)

doAssert getEnv(RADULA_ENVIRONMENT_DIRECTORY_CACHE_SOURCES) == "/var/cache/radula/src"
doAssert getEnv(RADULA_ENVIRONMENT_DIRECTORY_CACHE_VENOM) == "/var/cache/radula/venom"
doAssert getEnv(RADULA_ENVIRONMENT_DIRECTORY_CERATA) == "/var/db/radula/clusters/glaucus"
doAssert getEnv(RADULA_ENVIRONMENT_DIRECTORY_LOGS) == "/var/log/radula"

doassert getEnv(RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY_SYSTEM_BUILDS) == "/var/tmp/radula/system/bld"
doassert getEnv(RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY_SYSTEM_SOURCES) == "/var/tmp/radula/system/src"

doassert getEnv(RADULA_ENVIRONMENT_FILE_SYSTEM_LOG) == "/var/log/radula/system.log"