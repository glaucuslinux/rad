# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/os, constants

proc cleanBootstrap*() =
  const dirs = ["../cross", "../log", "../tmp", "../toolchain"]

  for i in dirs:
    removeDir(i)

proc prepareBootstrap*() =
  const dirs = ["../cross", "../log", "../pkg", "../src", "../tmp", "../toolchain"]

  for i in dirs:
    createDir(i)

proc prepareCross*() =
  const dir = "../tmp"

  removeDir(dir)
  createDir(dir)

proc setEnvBootstrap*() =
  const env = [
    ("CORD", "../core"),
    ("CRSD", "../cross"),
    ("TMPD", "../tmp"),
    ("TLCD", "../toolchain"),
  ]

  for (i, j) in env:
    putEnv(i, absolutePath(j))

  putEnv("PATH", getEnv("TLCD") / "usr/bin" & PathSep & getEnv("PATH"))

proc setEnvCross*() =
  const
    env = [
      ("AR", "x86_64-glaucus-linux-musl-gcc-ar"),
      ("AS", "x86_64-glaucus-linux-musl-as"),
      ("CC", "x86_64-glaucus-linux-musl-gcc"),
      ("CPP", "x86_64-glaucus-linux-musl-gcc -E"),
      ("CROSS_COMPILE", "x86_64-glaucus-linux-musl-"),
      ("CXX", "x86_64-glaucus-linux-musl-g++"),
      ("CXXCPP", "x86_64-glaucus-linux-musl-g++ -E"),
      ("HOSTCC", "gcc"),
      ("NM", "x86_64-glaucus-linux-musl-gcc-nm"),
      ("OBJCOPY", "x86_64-glaucus-linux-musl-objcopy"),
      ("OBJDUMP", "x86_64-glaucus-linux-musl-objdump"),
      ("RANLIB", "x86_64-glaucus-linux-musl-gcc-ranlib"),
      ("READELF", "x86_64-glaucus-linux-musl-readelf"),
      ("SIZE", "x86_64-glaucus-linux-musl-size"),
      ("STRIP", "x86_64-glaucus-linux-musl-strip"),
    ]
    envPkgConfig = [
      ("PKG_CONFIG_LIBDIR", "../cross/usr/lib/pkgconfig"),
      ("PKG_CONFIG_PATH", "../cross/usr/lib/pkgconfig"),
      ("PKG_CONFIG_SYSROOT_DIR", "../cross/"),
      ("PKG_CONFIG_SYSTEM_INCLUDE_PATH", "../cross/usr/include"),
      ("PKG_CONFIG_SYSTEM_LIBRARY_PATH", "../cross/usr/lib"),
    ]

  for (i, j) in env:
    putEnv(i, j)
  for (i, j) in envPkgConfig:
    putEnv(i, absolutePath(j))

proc setEnvNative*() =
  const env = [
    ("AR", "gcc-ar"),
    ("AWK", "mawk"),
    ("CC", "gcc"),
    ("CORD", pathCoreRepo),
    ("CPP", "gcc -E"),
    ("CXX", "g++"),
    ("CXXCPP", "g++ -E"),
    ("LEX", "reflex"),
    ("LIBTOOL", "slibtool"),
    ("NM", "gcc-nm"),
    ("PKG_CONFIG", "u-config"),
    ("RANLIB", "gcc-ranlib"),
    ("TMPD", pathTmp),
    ("YACC", "byacc"),
  ]

  for (i, j) in env:
    putEnv(i, j)
