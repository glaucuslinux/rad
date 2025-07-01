# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

const
  # dir
  dirCleanBootstrap* = ["cross", "log", "tmp", "toolchain"]
  dirPrepareBootstrap* = ["cross", "log", "pkg", "src", "tmp", "toolchain"]

  # env
  envBootstrap* =
    [("CORD", "core"), ("CRSD", "cross"), ("TMPD", "tmp"), ("TLCD", "toolchain")]
  envCross* = [
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
  envNative* = [
    ("AR", "gcc-ar"),
    ("AWK", "mawk"),
    ("CC", "gcc"),
    ("CPP", "gcc -E"),
    ("CXX", "g++"),
    ("CXXCPP", "g++ -E"),
    ("LEX", "reflex"),
    ("LIBTOOL", "slibtool"),
    ("NM", "gcc-nm"),
    ("PKG_CONFIG", "u-config"),
    ("RANLIB", "gcc-ranlib"),
    ("YACC", "byacc"),
  ]
  envPkgConfig* = [
    ("PKG_CONFIG_LIBDIR", "usr/lib/pkgconfig"),
    ("PKG_CONFIG_PATH", "usr/lib/pkgconfig"),
    ("PKG_CONFIG_SYSROOT_DIR", "/"),
    ("PKG_CONFIG_SYSTEM_INCLUDE_PATH", "usr/include"),
    ("PKG_CONFIG_SYSTEM_LIBRARY_PATH", "usr/lib"),
  ]

  # flags
  cflags* =
    "-pipe -O2 -fgraphite-identity -floop-nest-optimize -flto=auto -flto-compression-level=3 -fuse-linker-plugin -fstack-protector-strong -fstack-clash-protection -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -march=x86-64-v3 -mfpmath=sse -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2"
  ldflags* =
    "-Wl,-O1,-s,-z,noexecstack,-z,now,-z,pack-relative-relocs,-z,relro,-z,x86-64-v3,--as-needed,--gc-sections,--sort-common,--hash-style=gnu"
  ltoflags* = "-flto=auto -flto-compression-level=3 -fuse-linker-plugin "
  makeflags* = "-j 5 -O"

  # help
  help* =
    """
USAGE:
  rad [ COMMAND ]

COMMANDS:
  build      Build packages
  clean      Clean cache
  contents   List package contents
  help       Display this help message
  info       Show package information
  install    Install packages
  list       List installed packages
  orphan     List orphaned packages
  remove     Remove packages
  search     Search for packages
  update     Update repositories
  upgrade    Upgrade packages
  version    Display rad version"""
  helpBootstrap* =
    """
USAGE:
  rad bootstrap [ COMMAND ]

COMMANDS:
  clean      Clean cache
  cross      Bootstrap cross glaucus (stage 2)
  native     Bootstrap native glaucus (stage 3)
  toolchain  Bootstrap toolchain (stage 1)"""
  version* =
    """
rad version 0.1.0

Licensed under the Mozilla Public License Version 2.0 (MPL-2.0)
Copyright © 2018-2025 Firas Khana"""

  # path
  pathPkgCache* = "/var/cache/rad/pkg"
  pathSrcCache* = "/var/cache/rad/src"
  pathLocalLib* = "/var/lib/rad/local"
  pathCoreRepo* = "/var/lib/rad/repo/core"
  pathExtraRepo* = "/var/lib/rad/repo/extra"
  pathLog* = "/var/log/rad"
  pathTmp* = "/var/tmp/rad"
  pathTmpLock* = "/var/tmp/rad.lock"

  # shell
  shellRedirect* = ">/dev/null 2>&1"

  # tool
  tool* = [
    "autoconf", "automake", "autopoint", "awk", "bash", "booster", "bzip2", "curl",
    "diff", "find", "gcc", "git", "grep", "gzip", "ld.bfd", "lex", "libtool", "limine",
    "m4", "make", "meson", "mkfs.erofs", "mkfs.fat", "ninja", "patch", "perl",
    "pkg-config", "sed", "tar", "xz", "yacc", "zstd",
  ]

type Stages* = enum
  cross
  native
  toolchain
