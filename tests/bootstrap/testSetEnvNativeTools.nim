# Copyright (c) 2018-2025, Firas Khalil Khana
# Distributed under the terms of the ISC License

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
doAssert getEnv($PKG_CONFIG) == "pkgconf"
doAssert getEnv($RANLIB) == "gcc-ranlib"
doAssert getEnv($YACC) == "byacc"
