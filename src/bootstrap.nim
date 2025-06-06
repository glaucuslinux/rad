# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/[os, strformat], constants

proc cleanBootstrap*() =
  removeDir(getEnv($CRSD))
  removeDir(getEnv($LOGD))
  removeDir(getEnv($TMPD))
  removeDir(getEnv($TLCD))

proc distcleanBootstrap*() =
  cleanBootstrap()

  removeDir(getEnv($PKGD))
  removeDir(getEnv($SRCD))

proc prepareBootstrap*() =
  createDir(getEnv($CRSD))
  createDir(getEnv($LOGD))
  createDir(getEnv($PKGD))
  createDir(getEnv($SRCD))
  createDir(getEnv($TMPD))
  createDir(getEnv($TLCD))

proc prepareCross*() =
  removeDir(getEnv($TMPD))
  createDir(getEnv($TMPD))

proc setEnvBootstrap*() =
  let path = parentDir(getCurrentDir())

  putEnv($CERD, path / $cerata)
  putEnv($CRSD, path / $cross)
  putEnv($LOGD, path / $log)
  putEnv($PKGD, path / $pkg)
  putEnv($SRCD, path / $src)
  putEnv($TMPD, path / $tmp)
  putEnv($TLCD, path / $toolchain)

  putEnv($PATH, getEnv($TLCD) / $usr / &"{bin}:{getEnv($PATH)}")

proc setEnvCrossTools*() =
  let crossCompile = &"{getEnv($TGT)}-"

  putEnv($CROSS_COMPILE, crossCompile)

  putEnv($AR, &"{crossCompile}{ar}")
  putEnv($radEnv.AS, &"{crossCompile}{radTools.As}")
  putEnv($CC, &"{crossCompile}{gcc}")
  putEnv($CPP, &"{getEnv($CC)} {cpp}")
  putEnv($CXX, &"{crossCompile}{cxx}")
  putEnv($CXXCPP, &"{getEnv($CXX)} {cpp}")
  putEnv($HOSTCC, $gcc)
  putEnv($NM, &"{crossCompile}{nm}")
  putEnv($OBJCOPY, &"{crossCompile}{objcopy}")
  putEnv($OBJDUMP, &"{crossCompile}{objdump}")
  putEnv($PKG_CONFIG_LIBDIR, getEnv($CRSD) / $pkgConfigLibdir)
  putEnv($PKG_CONFIG_PATH, getEnv($PKG_CONFIG_LIBDIR))
  putEnv($PKG_CONFIG_SYSROOT_DIR, &"{getEnv($CRSD)}{root}")
  putEnv($PKG_CONFIG_SYSTEM_INCLUDE_PATH, getEnv($CRSD) / $pkgConfigSystemIncludePath)
  putEnv($PKG_CONFIG_SYSTEM_LIBRARY_PATH, getEnv($CRSD) / $pkgConfigSystemLibraryPath)
  putEnv($RANLIB, &"{crossCompile}{ranlib}")
  putEnv($READELF, &"{crossCompile}{readelf}")
  putEnv($SIZE, &"{crossCompile}{size}")
  putEnv($STRIP, &"{crossCompile}{strip}")

proc setEnvNativeDirs*() =
  putEnv($CERD, $radClustersCerataLib)
  putEnv($LOGD, $radLog)
  putEnv($PKGD, $radPkgCache)
  putEnv($SRCD, $radSrcCache)
  putEnv($TMPD, $radTmp)

proc setEnvNativeTools*() =
  putEnv($AR, $ar)
  putEnv($AWK, $mawk)
  putEnv($CC, $gcc)
  putEnv($CPP, &"{gcc} {cpp}")
  putEnv($CXX, $cxx)
  putEnv($CXXCPP, &"{cxx} {cpp}")
  putEnv($LEX, $reflex)
  putEnv($LIBTOOL, $slibtool)
  putEnv($NM, $nm)
  putEnv($PKG_CONFIG, $uConfig)
  putEnv($RANLIB, $ranlib)
  putEnv($YACC, $byacc)
