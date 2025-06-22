# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

type
  radArch* = enum
    glaucus
    tupleCross = "glaucus-linux-musl"
    tupleNative = "pc-linux-musl"
    x86_64 = "x86-64"
    x86_64Linux = "x86_64"
    x86_64_v3 = "x86-64-v3"

  radPackages* = enum
    acl
    attr
    autoconf
    automake
    awsLc = "aws-lc"
    bash
    binutils
    byacc
    bzip2
    cmake
    core
    curl
    dash
    eiwd
    execline
    expat
    file
    fping
    fs
    gcc
    gettextTiny = "gettext-tiny"
    gmp
    gperf
    grep
    hwdata
    iproute2
    isl
    kmod
    less
    libarchive
    libcap
    libedit
    libgcc
    libstdcxxV3 = "libstdc++-v3"
    libudevZero = "libudev-zero"
    limine
    linux
    linuxCachyOS = "linux-cachyos"
    linuxHeaders = "linux-headers"
    lz4
    m4
    make
    mawk
    mdevd
    mimalloc
    mpc
    mpfr
    muon
    musl
    muslHeaders = "musl-headers"
    muslUtils = "musl-utils"
    neatvi
    netbsdCurses = "netbsd-curses"
    openresolv
    pciutils
    pcre2
    perl
    pigz
    rad
    reflex
    s6
    s6BootScripts = "s6-boot-scripts"
    s6LinuxInit = "s6-linux-init"
    s6Rc = "s6-rc"
    sdhcp
    shadow
    skalibs
    skel
    slibtool
    toybox
    uConfig = "u-config"
    utilLinux = "util-linux"
    utmps
    xxhash
    xz
    yash
    zlibNg = "zlib-ng"
    zstd

  radDirs* = enum
    dst
    log
    pkg
    src
    tmp

  radEnv* = enum
    # ARCH
    ARCH
    BARCH
    CARCH
    CROSS_COMPILE
    PRETTY_NAME
    TARCH

    # DIRS
    CRSD
    DSTD
    LOGD
    PATH
    PKGD
    SRCD
    TLCD
    TMPD

    # FLAGS
    CFLAGS
    CXXFLAGS
    LDFLAGS
    MAKEFLAGS

    # PKG_CONFIG
    PKG_CONFIG_LIBDIR
    PKG_CONFIG_PATH
    PKG_CONFIG_SYSROOT_DIR
    PKG_CONFIG_SYSTEM_INCLUDE_PATH
    PKG_CONFIG_SYSTEM_LIBRARY_PATH

    # TOOLS
    AR
    AS
    AWK
    CC
    CPP
    CXX
    CXXCPP
    HOSTCC
    LEX
    LIBTOOL
    NM
    OBJCOPY
    OBJDUMP
    PKG_CONFIG
    RANLIB
    READELF
    SIZE
    STRIP
    YACC

  radFiles* = enum
    configGuess = "config.guess"
    contents
    files
    info
    tarZst = ".tar.zst"

  radFlags* = enum
    cflags =
      "-pipe -O2 -fgraphite-identity -floop-nest-optimize -flto=auto -flto-compression-level=3 -fuse-linker-plugin -fstack-protector-strong -fstack-clash-protection -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -march=x86-64-v3 -mfpmath=sse -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2"
    cpp = "-E"
    ldflags =
      "-Wl,-O1,-s,-z,noexecstack,-z,now,-z,pack-relative-relocs,-z,relro,-z,x86-64-v3,--as-needed,--gc-sections,--sort-common,--hash-style=gnu"
    lto = "-flto=auto -flto-compression-level=3 -fuse-linker-plugin "
    noParallel = "-j 1"
    parallel = "-j 5 -O"
    shellRedirect = ">/dev/null 2>&1"

  radHelp* = enum
    Bootstrap =
      """
USAGE:
  rad bootstrap [ COMMAND ]

COMMANDS:
  clean      Clean cache
  cross      Bootstrap cross glaucus
  distclean  Clean everything
  native     Bootstrap native glaucus
  toolchain  Bootstrap toolchain"""
    Rad =
      """
USAGE:
  rad [ COMMAND ]

COMMANDS:
  build      Build packages
  clean      Clean cache
  contents   List package contents
  distclean  Clean everything
  help       Display this help message
  info       Show package information
  install    Build and install packages
  list       List installed packages
  orphan     List orphaned packages
  remove     Remove packages
  search     Search for packages
  update     Update repositories
  upgrade    Upgrade packages
  version    Display rad version"""
    Version =
      """
rad version 0.1.0

Licensed under the Mozilla Public License Version 2.0 (MPL-2.0)
Copyright © 2018-2025 Firas Khana"""

  radOpt* = enum
    bootstrap
    empty
    noCheck = "no-check"
    noLTO = "no-lto"
    noParallel = "no-parallel"

  radPaths* = enum
    bin
    # `pkgconf` and `pkg-config` do not respect sysroot; does not get prefixed
    # to PATH and LIBDIR
    pkgConfigLibdir = "/usr/lib/pkgconfig"
    radBuildCache = "/var/cache/rad/build"
    radConf = "/etc/rad.conf"
    radCoreRepo = "/var/lib/rad/repo/core"
    radExtraRepo = "/var/lib/rad/repo/extra"
    radLocalLib = "/var/lib/rad/local"
    radLock = "/tmp/rad.lock"
    radLog = "/var/log/rad"
    radSrcCache = "/var/cache/rad/src"
    radTmp = "/var/tmp/rad"
    root = "/"
    usr
    usrInclude = "/usr/include"
    usrLibrary = "/usr/lib"

  radPrint* = enum
    build
    fetch
    install
    Nil = "nil"
    remove

  radStages* = enum
    cross
    native
    toolchain

  radTools* = enum
    ar = "gcc-ar"
    As = "as"
    awk
    cxx = "g++"
    git
    lex
    nm = "gcc-nm"
    objcopy
    objdump
    pkgConfig = "pkg-config"
    ranlib = "gcc-ranlib"
    readelf
    sh
    size
    strip
    tar
    yacc

const
  Cross* = [
    # Development
    $fs,
    $packages,
    $expat,
    $linuxHeaders,
    $muslUtils,
    $netbsdCurses,
    $uConfig,

    # Init
    $skalibs,
    $execline,
    $mdevd,
    $s6,
    $utmps,

    # Security
    $attr,
    $acl,
    $awsLc,
    $libcap,
    $shadow,

    # Compression
    $zlibNg,
    $bzip2,
    $pigz,
    $lz4,
    $xz,
    $zstd,
    $libarchive,

    # Development
    $autoconf,
    $binutils,
    $file,
    $gcc,
    $gettextTiny,
    $libedit,
    $m4,
    $make,
    $mawk,
    $pcre2,
    $reflex,
    $slibtool,

    # Utilities
    $dash,
    $grep,
    $kmod,
    $libudevZero,
    $toybox,
    $utilLinux,
    $xxhash,

    # Services
    $s6LinuxInit,
    $s6Rc,
    $s6BootScripts,

    # Kernel
    $linuxCachyOS,
  ]
  Native* = [
    # Development
    $fs,
    $packages,
    $linuxHeaders,
    $musl,
    $perl,
    $automake,
    $bash,
    $byacc,
    $curl,
    $cmake,
    $gmp,
    $gperf,
    $mpfr,
    $mpc,
    $isl,

    # Security
    $acl,
    $attr,
    $awsLc,
    $libcap,
    $shadow,

    # Compression
    $bzip2,
    $pigz,
    $libarchive,
    $lz4,
    $xz,
    $zlibNg,
    $zstd,

    # Development
    $autoconf,
    $binutils,
    $expat,
    $file,
    $gcc,
    $gettextTiny,
    $libedit,
    $m4,
    $make,
    $mawk,
    $muon,
    $netbsdCurses,
    $pcre2,
    $reflex,
    $slibtool,
    $uConfig,

    # Networking
    $eiwd,
    $fping,
    $iproute2,
    $openresolv,
    $sdhcp,

    # Utilities
    $dash,
    $grep,
    $hwdata,
    $kmod,
    $less,
    $libudevZero,
    $neatvi,
    $pciutils,
    $toybox,
    $utilLinux,
    $xxhash,
    $yash,

    # Init & Services
    $skalibs,
    $execline,
    $mdevd,
    $s6,
    $utmps,
    $s6LinuxInit,
    $s6Rc,
    $s6BootScripts,

    # Kernel
    $linuxCachyOS,
  ]
  Toolchain* = [$muslHeaders, $binutils, $gcc, $musl, $libgcc, $libstdcxxV3]
