# SPDX-License-Identifier: MPL-2.0
# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/os, ../../src/[arch, bootstrap, constants]

setEnvArch()
setEnvNativeTools()

echo "AR          :: ", getEnv($AR)
echo "AWK         :: ", getEnv($AWK)
echo "CC          :: ", getEnv($CC)
echo "CPP         :: ", getEnv($CPP)
echo "CXX         :: ", getEnv($CXX)
echo "CXXCPP      :: ", getEnv($CXXCPP)
echo "LEX         :: ", getEnv($LEX)
echo "LIBTOOL     :: ", getEnv($LIBTOOL)
echo "NM          :: ", getEnv($NM)
echo "PKG_CONFIG  :: ", getEnv($PKG_CONFIG)
echo "RANLIB      :: ", getEnv($RANLIB)
echo "YACC        :: ", getEnv($YACC)

doAssert getEnv($AR) == "gcc-ar"
doAssert getEnv($AWK) == "mawk"
doAssert getEnv($CC) == "gcc"
doAssert getEnv($CPP) == "gcc -E"
doAssert getEnv($CXX) == "g++"
doAssert getEnv($CXXCPP) == "g++ -E"
doAssert getEnv($LEX) == "reflex"
doAssert getEnv($LIBTOOL) == "slibtool"
doAssert getEnv($NM) == "gcc-nm"
doAssert getEnv($PKG_CONFIG) == "u-config"
doAssert getEnv($RANLIB) == "gcc-ranlib"
doAssert getEnv($YACC) == "byacc"
