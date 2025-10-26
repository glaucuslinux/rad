# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/[os, strformat, strutils], packages, utils

proc cleanBootstrap*() =
  const dirs = ["../cross", "../log", "../tmp", "../toolchain"]

  for i in dirs:
    removeDir(i)

proc prepareToolchain() =
  const
    dirs = ["../cross", "../log", "../pkg", "../src", "../tmp", "../toolchain"]
    exes = [
      "autoconf", "automake", "autopoint", "awk", "bash", "booster", "bzip2", "curl",
      "diff", "find", "gcc", "git", "gperf", "grep", "gzip", "ld.bfd", "lex", "libtool",
      "limine", "m4", "make", "meson", "mkfs.erofs", "mkfs.fat", "ninja", "patch",
      "perl", "pkg-config", "sed", "tar", "xz", "yacc", "zstd",
    ]

  for i in exes:
    if findExe(i).isEmptyOrWhitespace():
      abort(&"""{127:<8}{&"\{i\} not found":48}""")

  cleanBootstrap()

  for i in dirs:
    createDir(i)

proc configureToolchain() =
  const env = [
    ("core", "../core"),
    ("cross", "../cross"),
    ("tmp", "../tmp"),
    ("toolchain", "../toolchain"),
  ]

  for (i, j) in env:
    putEnv(i, absolutePath(j))

  putEnv("PATH", getEnv("toolchain") / "usr/bin" & PathSep & getEnv("PATH"))

proc bootstrapToolchain*() =
  prepareToolchain()
  configureToolchain()
  buildPackages(parseInfo("toolchain").run.split(), true, toolchain)

proc prepareCross() =
  const dir = "../tmp"

  removeDir(dir)
  createDir(dir)

proc configureCross() =
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

proc bootstrapCross*() =
  prepareCross()
  configureCross()
  configureToolchain()
  buildPackages(parseInfo("cross").run.split(), true, cross)
