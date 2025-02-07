# Copyright (c) 2018-2025, Firas Khalil Khana
# Distributed under the terms of the ISC License

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
    libressl
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
    pkgconf
    rad
    reflex
    s6
    s6BootScripts = "s6-boot-scripts"
    s6LinuxInit = "s6-linux-init"
    s6Rc = "s6-rc"
    samurai
    sdhcp
    shadow
    skalibs
    slibtool
    toybox
    utilLinux = "util-linux"
    utmps
    wget2
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
    GLAD
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
    ceras
    configGuess = "config.guess"
    files
    tarZst = ".tar.zst"

  radFlags* = enum
    cflags =
      "-pipe -O2 -fgraphite-identity -floop-nest-optimize -flto=auto -flto-compression-level=19 -fuse-linker-plugin -fstack-protector-strong -fstack-clash-protection -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -march=x86-64-v3 -mfpmath=sse -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2"
    cpp = "-E"
    ldflags =
      "-Wl,-O1,-s,-z,noexecstack,-z,now,-z,pack-relative-relocs,-z,relro,-z,x86-64-v3,--as-needed,--gc-sections,--sort-common,--hash-style=gnu"
    lto = "-flto=auto -flto-compression-level=19 -fuse-linker-plugin "
    make = "-j4 -O"
    parallel = "-j1 -O"
    shellRedirect = ">/dev/null 2>&1"
    zstdCompress = "-22 --ultra -T0 --long"
    zstdDecompress = "-d -T0 --long"

  radHelp* = enum
    Rad =
      """
USAGE:
  rad [ OPTION ]

OPTIONS:
  -b, --bootstrap  Bootstrap glaucus
  -c, --cerata     Manage cerata
  -h, --help       Display this help message
  -v, --version    Display current version"""
    Bootstrap =
      """
USAGE:
  rad [ -b | --bootstrap ] [ COMMAND ]

COMMANDS:
  c, clean         Clean cache
  d, distclean     Clean everything
  h, help          Display this help message
  n, native        Bootstrap native glaucus
  t, toolchain     Bootstrap toolchain
  x, cross         Bootstrap cross glaucus"""
    Cerata =
      """
USAGE:
  rad [ -c | --cerata ] [ COMMAND ] [ cerata ]

COMMANDS:
  b, build         Build cerata
  c, clean         Clean cache
  d, distclean     Clean everything
  h, help          Display this help message
  i, install       Install cerata
  l, list          List installed cerata
  p, print         Print cerata information
  r, remove        Remove cerata
  s, search        Search for cerata
  u, upgrade       Upgrade cerata"""
    version =
      """
rad version 0.1.0

Copyright (c) 2018-2025, Firas Khalil Khana
Distributed under the terms of the ISC License"""

  radNop* = enum
    bootstrap
    check
    empty
    lto
    parallel

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
    install
    Nil = "nil"
    prepare
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
    ranlib = "gcc-ranlib"
    readelf
    sh
    size
    strip
    tar = "bsdtar"
    yacc

const
  Cross* = [
    # Filesystem
    $fs,

    # Development
    $cerata,
    $expat,
    $linuxHeaders,
    $muslUtils,
    $netbsdCurses,
    $pkgconf,

    # Init
    $skalibs,
    $execline,
    $mdevd,
    $s6,
    $utmps,

    # Security
    $attr,
    $acl,
    $libcap,
    $libressl,
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
    $radCerata.make,
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
    # Filesystem
    $fs,

    # Development
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
    $libcap,
    $libressl,
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
    $radCerata.make,
    $mawk,
    $netbsdCurses,
    $pcre2,
    $pkgconf,
    $reflex,
    $samurai,
    $slibtool,

    # Networking
    $eiwd,
    $fping,
    $iproute2,
    $openresolv,
    $sdhcp,
    $wget2,

    # Utilities
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
