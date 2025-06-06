# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/[os, strutils], ../../src/[arch, bootstrap, constants]

setEnvArch(cross)
setEnvBootstrap()
setEnvCrossTools()

echo "CROSS_COMPILE                   :: ", getEnv($CROSS_COMPILE)

echo ""

echo "AR                              :: ", getEnv($AR)
echo "AS                              :: ", getEnv($radEnv.AS)
echo "CC                              :: ", getEnv($CC)
echo "CPP                             :: ", getEnv($CPP)
echo "CXX                             :: ", getEnv($CXX)
echo "CXXCPP                          :: ", getEnv($CXXCPP)
echo "HOSTCC                          :: ", getEnv($HOSTCC)
echo "NM                              :: ", getEnv($NM)
echo "OBJCOPY                         :: ", getEnv($OBJCOPY)
echo "OBJDUMP                         :: ", getEnv($OBJDUMP)
echo "PKG_CONFIG_LIBDIR               :: ", getEnv($PKG_CONFIG_LIBDIR)
echo "PKG_CONFIG_PATH                 :: ", getEnv($PKG_CONFIG_PATH)
echo "PKG_CONFIG_SYSROOT_DIR          :: ", getEnv($PKG_CONFIG_SYSROOT_DIR)
echo "PKG_CONFIG_SYSTEM_INCLUDE_PATH  :: ", getEnv($PKG_CONFIG_SYSTEM_INCLUDE_PATH)
echo "PKG_CONFIG_SYSTEM_LIBRARY_PATH  :: ", getEnv($PKG_CONFIG_SYSTEM_LIBRARY_PATH)
echo "RANLIB                          :: ", getEnv($RANLIB)
echo "READELF                         :: ", getEnv($READELF)
echo "SIZE                            :: ", getEnv($SIZE)
echo "STRIP                           :: ", getEnv($STRIP)

doAssert getEnv($CROSS_COMPILE).endsWith("-")

doAssert getEnv($AR).endsWith("gcc-ar")
doAssert getEnv($radEnv.AS).endsWith("as")
doAssert getEnv($CC).endsWith("gcc")
doAssert getEnv($CPP).endsWith("gcc -E")
doAssert getEnv($CXX).endsWith("g++")
doAssert getEnv($CXXCPP).endsWith("g++ -E")
doAssert getEnv($HOSTCC) == "gcc"
doAssert getEnv($NM).endsWith("gcc-nm")
doAssert getEnv($OBJCOPY).endsWith("objcopy")
doAssert getEnv($OBJDUMP).endsWith("objdump")
doAssert getEnv($PKG_CONFIG_LIBDIR).endsWith("cross/usr/lib/pkgconfig")
doAssert getEnv($PKG_CONFIG_PATH).endsWith("cross/usr/lib/pkgconfig")
doAssert getEnv($PKG_CONFIG_SYSROOT_DIR).endsWith("cross/")
doAssert getEnv($PKG_CONFIG_SYSTEM_INCLUDE_PATH).endsWith("cross/usr/include")
doAssert getEnv($PKG_CONFIG_SYSTEM_LIBRARY_PATH).endsWith("cross/usr/lib")
doAssert getEnv($RANLIB).endsWith("gcc-ranlib")
doAssert getEnv($READELF).endsWith("readelf")
doAssert getEnv($SIZE).endsWith("size")
doAssert getEnv($STRIP).endsWith("strip")
