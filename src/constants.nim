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

  radCerata* = enum
    acl
    attr
    autoconf
    automake
    awsLc = "aws-lc"
    bash
    binutils
    byacc
    bzip2
    cerata
    cmake
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
    log
    pkg
    sac
    src
    tmp

  radEnv* = enum
    # ARCH
    ARCH
    CARCH
    CROSS_COMPILE
    PRETTY_NAME
    BLD
    TGT

    # DIRS
    CERD
    CRSD
    LOGD
    PATH
    PKGD
    SACD
    SRCD
    TMPD
    TLCD

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
    parallel = "-j 4 -O"
    noParallel = "-j 1"
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
  build      Build cerata
  clean      Clean cache
  contents   List cerata contents
  distclean  Clean everything
  help       Display this help message
  info       Show cerata information
  install    Build and install cerata
  list       List installed cerata
  orphan     List orphaned cerata
  remove     Remove cerata
  search     Search for cerata
  update     Update clusters
  upgrade    Upgrade cerata
  version    Display rad version"""
    version =
      """
rad version 0.1.0

Licensed under the Mozilla Public License Version 2.0 (MPL-2.0)
Copyright © 2018-2025 Firas Khana"""

  radOpt* = enum
    bootstrap
    empty
    noCheck
    noLTO
    noParallel

  radPaths* = enum
    bin
    # `pkgconf` and `pkg-config` do not respect sysroot; does not get prefixed
    # to PATH and LIBDIR
    pkgConfigLibdir = "/usr/lib/pkgconfig"
    pkgConfigSystemIncludePath = "/usr/include"
    pkgConfigSystemLibraryPath = "/usr/lib"
    radConf = "/etc/rad.conf"
    radLock = "/tmp/rad.lock"
    radPkgCache = "/var/cache/rad/pkg"
    radSrcCache = "/var/cache/rad/src"
    radClustersCerataLib = "/var/lib/rad/clusters/cerata"
    radPkgLib = "/var/lib/rad/pkg"
    radLog = "/var/log/rad"
    radTmp = "/var/tmp/rad"
    root = "/"
    usr

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
    tar = "bsdtar"
    yacc

const
  Cross* = [
    # Development
    $fs,
    $cerata,
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
    $cerata,
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
