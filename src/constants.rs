// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

// * All constants are prefixed with `RADULA_`
// * All constants are ordered alphabetically
// * The `'static` is needed: https://doc.rust-lang.org/std/keyword.const.html

//
// Architectures
//

pub const RADULA_ARCHITECTURE_CERATA: &'static str = "--with-gcc-arch=";
pub const RADULA_ARCHITECTURE_TUPLE_TARGET: &'static str = "-glaucus-linux-musl";

// aarch64
pub const RADULA_ARCHITECTURE_AARCH64: &'static str = "aarch64";
pub const RADULA_ARCHITECTURE_AARCH64_CERATA: &'static str = "armv8-a";
pub const RADULA_ARCHITECTURE_AARCH64_FLAGS: &'static str =
    "-mabi=lp64 -mfix-cortex-a53-835769 -mfix-cortex-a53-843419 -march=armv8-a -mtune=generic";
pub const RADULA_ARCHITECTURE_AARCH64_GCC_CONFIGURATION: &'static str = "--with-arch=armv8-a --with-abi=lp64 --enable-fix-cortex-a53-835769 --enable-fix-cortex-a53-843419";
pub const RADULA_ARCHITECTURE_AARCH64_GCOMPAT: &'static str = ".so.1";
pub const RADULA_ARCHITECTURE_AARCH64_LINUX: &'static str = "arm64";
pub const RADULA_ARCHITECTURE_AARCH64_LINUX_IMAGE: &'static str = "arch/arm64/boot/Image";

// armv6zk
pub const RADULA_ARCHITECTURE_ARMV6ZK: &'static str = "armv6zk";
pub const RADULA_ARCHITECTURE_ARMV6ZK_FLAGS: &'static str = "-mabi=aapcs-linux -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s -mcpu=arm1176jzf-s -mfpu=vfpv2 -mtls-dialect=gnu2";
pub const RADULA_ARCHITECTURE_ARMV6ZK_GCC_CONFIGURATION: &'static str = "--with-arch=armv6zk --with-tune=arm1176jzf-s --with-abi=aapcs-linux --with-fpu=vfpv2 --with-float=hard";
pub const RADULA_ARCHITECTURE_ARMV6ZK_GCOMPAT: &'static str = ".so.3";
pub const RADULA_ARCHITECTURE_ARMV6ZK_LINUX: &'static str = "arm";
pub const RADULA_ARCHITECTURE_ARMV6ZK_LINUX_CONFIGURATION: &'static str = "bcm2835_";
pub const RADULA_ARCHITECTURE_ARMV6ZK_LINUX_IMAGE: &'static str = "arch/arm/boot/zImage";
pub const RADULA_ARCHITECTURE_ARMV6ZK_MUSL_LINKER: &'static str = "hf";
pub const RADULA_ARCHITECTURE_ARMV6ZK_TUPLE_TARGET: &'static str = "eabihf";

// i686
pub const RADULA_ARCHITECTURE_I686: &'static str = "i686";
pub const RADULA_ARCHITECTURE_I686_FLAGS: &'static str =
    "-march=i686 -mtune=generic -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2";
pub const RADULA_ARCHITECTURE_I686_GCC_CONFIGURATION: &'static str =
    "--with-arch=i686 --with-tune=generic";
pub const RADULA_ARCHITECTURE_I686_GCOMPAT: &'static str = ".so.2";
pub const RADULA_ARCHITECTURE_I686_LINUX: &'static str = "i386";
pub const RADULA_ARCHITECTURE_I686_LINUX_CONFIGURATION: &'static str = "i386_";
pub const RADULA_ARCHITECTURE_I686_LINUX_IMAGE: &'static str = "arch/x86/boot/bzImage";
pub const RADULA_ARCHITECTURE_I686_UCONTEXT: &'static str = "x86";

// riscv64
pub const RADULA_ARCHITECTURE_RISCV64: &'static str = "riscv64";
pub const RADULA_ARCHITECTURE_RISCV64_CERATA: &'static str = "rv64gc";
pub const RADULA_ARCHITECTURE_RISCV64_FLAGS: &'static str =
    "-mabi=lp64d -march=rv64gc -mcpu=sifive-u74 -mtune=sifive-7-series -mcmodel=medany";
pub const RADULA_ARCHITECTURE_RISCV64_GCC_CONFIGURATION: &'static str =
    "--with-cpu=sifive-u74 --with-arch=rv64gc --with-tune=sifive-7-series --with-abi=lp64d";
pub const RADULA_ARCHITECTURE_RISCV64_GCOMPAT: &'static str = "-lp64d.so.1";
pub const RADULA_ARCHITECTURE_RISCV64_LINUX: &'static str = "riscv";
pub const RADULA_ARCHITECTURE_RISCV64_LINUX_IMAGE: &'static str = "arch/riscv/boot/Image";

// x86-64-v3
pub const RADULA_ARCHITECTURE_X86_64_V3: &'static str = "x86-64";
pub const RADULA_ARCHITECTURE_X86_64_V3_FLAGS: &'static str =
    "-march=x86-64-v3 -mtune=generic -mfpmath=sse -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2";
pub const RADULA_ARCHITECTURE_X86_64_V3_GCC_CONFIGURATION: &'static str =
    "--with-arch=x86-64-v3 --with-tune=generic";
pub const RADULA_ARCHITECTURE_X86_64_V3_LINUX: &'static str = "x86_64";
pub const RADULA_ARCHITECTURE_X86_64_V3_LINUX_CONFIGURATION: &'static str = "x86_64_";

//
// Cerata
//

pub const RADULA_CERAS: &'static str = "ceras";

pub const RADULA_CERAS_ACL: &'static str = "acl";
pub const RADULA_CERAS_ATTR: &'static str = "attr";

pub const RADULA_CERAS_BC: &'static str = "bc";
pub const RADULA_CERAS_BINUTILS: &'static str = "binutils";
pub const RADULA_CERAS_BZIP2: &'static str = "bzip2";

pub const RADULA_CERAS_CERATA: &'static str = "cerata";

pub const RADULA_CERAS_DASH: &'static str = "dash";
pub const RADULA_CERAS_DIFFUTILS: &'static str = "diffutils";

pub const RADULA_CERAS_E2FSPROGS: &'static str = "e2fsprogs";
pub const RADULA_CERAS_ELFUTILS: &'static str = "elfutils";
pub const RADULA_CERAS_EUDEV: &'static str = "eudev";
pub const RADULA_CERAS_EXECLINE: &'static str = "execline";
pub const RADULA_CERAS_EXPAT: &'static str = "expat";

pub const RADULA_CERAS_FILE: &'static str = "file";
pub const RADULA_CERAS_FINDUTILS: &'static str = "findutils";

pub const RADULA_CERAS_GCC: &'static str = "gcc";
pub const RADULA_CERAS_GCOMPAT: &'static str = "gcompat";
pub const RADULA_CERAS_GETTEXT_TINY: &'static str = "gettext-tiny";
pub const RADULA_CERAS_GMP: &'static str = "gmp";
pub const RADULA_CERAS_GREP: &'static str = "grep";

pub const RADULA_CERAS_HOSTNAME: &'static str = "hostname";
pub const RADULA_CERAS_HWIDS: &'static str = "hwids";
pub const RADULA_CERAS_HYDROSKELETON: &'static str = "hydroskeleton";

pub const RADULA_CERAS_IANA_ETC: &'static str = "iana-etc";
pub const RADULA_CERAS_IPROUTE2: &'static str = "iproute2";
pub const RADULA_CERAS_IPUTILS: &'static str = "iputils";
pub const RADULA_CERAS_ISL: &'static str = "isl";

pub const RADULA_CERAS_KMOD: &'static str = "kmod";

pub const RADULA_CERAS_LBZIP2: &'static str = "lbzip2";
pub const RADULA_CERAS_LBZIP2_UTILS: &'static str = "lbzip2-utils";
pub const RADULA_CERAS_LESS: &'static str = "less";
pub const RADULA_CERAS_LIBARCHIVE: &'static str = "libarchive";
pub const RADULA_CERAS_LIBCAP: &'static str = "libcap";
pub const RADULA_CERAS_LIBEDIT: &'static str = "libedit";
pub const RADULA_CERAS_LIBELF: &'static str = "libelf";
pub const RADULA_CERAS_LIBGCC: &'static str = "libgcc";
pub const RADULA_CERAS_LIBGOMP: &'static str = "libgomp";
pub const RADULA_CERAS_LIBRESSL: &'static str = "libressl";
pub const RADULA_CERAS_LIBSTDCXX_V3: &'static str = "libstdc++-v3";
pub const RADULA_CERAS_LIBUARGP: &'static str = "libuargp";
pub const RADULA_CERAS_LIBUCONTEXT: &'static str = "libucontext";
pub const RADULA_CERAS_LINUX: &'static str = "linux";
pub const RADULA_CERAS_LINUX_HEADERS: &'static str = "linux-headers";
pub const RADULA_CERAS_LKSH: &'static str = "lksh";
pub const RADULA_CERAS_LZ4: &'static str = "lz4";
pub const RADULA_CERAS_LZLIB: &'static str = "lzlib";

pub const RADULA_CERAS_MANDOC: &'static str = "mandoc";
pub const RADULA_CERAS_MKSH: &'static str = "mksh";
pub const RADULA_CERAS_MOLD: &'static str = "mold";
pub const RADULA_CERAS_MPC: &'static str = "mpc";
pub const RADULA_CERAS_MPFR: &'static str = "mpfr";
pub const RADULA_CERAS_MUSL: &'static str = "musl";
pub const RADULA_CERAS_MUSL_FTS: &'static str = "musl-fts";
pub const RADULA_CERAS_MUSL_HEADERS: &'static str = "musl-headers";
pub const RADULA_CERAS_MUSL_LOCALES: &'static str = "musl-locales";
pub const RADULA_CERAS_MUSL_OBSTACK: &'static str = "musl-obstack";
pub const RADULA_CERAS_MUSL_RPMATCH: &'static str = "musl-rpmatch";
pub const RADULA_CERAS_MUSL_UTILS: &'static str = "musl-utils";

pub const RADULA_CERAS_NETBSD_CURSES: &'static str = "netbsd-curses";

pub const RADULA_CERAS_OKSH: &'static str = "oksh";

pub const RADULA_CERAS_PCIUTILS: &'static str = "pciutils";
pub const RADULA_CERAS_PCRE2: &'static str = "pcre2";
pub const RADULA_CERAS_PIGZ: &'static str = "pigz";
pub const RADULA_CERAS_PLOCATE: &'static str = "plocate";
pub const RADULA_CERAS_PLZIP: &'static str = "plzip";
pub const RADULA_CERAS_PROCPS_NG: &'static str = "procps-ng";
pub const RADULA_CERAS_PSMISC: &'static str = "psmisc";

pub const RADULA_CERAS_RADULA: &'static str = "radula";
pub const RADULA_CERAS_RSYNC: &'static str = "rsync";

pub const RADULA_CERAS_S6: &'static str = "s6";
pub const RADULA_CERAS_S6_BOOT_SCRIPTS: &'static str = "s6-boot-scripts";
pub const RADULA_CERAS_S6_LINUX_INIT: &'static str = "s6-linux-init";
pub const RADULA_CERAS_S6_RC: &'static str = "s6-rc";
pub const RADULA_CERAS_SDHCP: &'static str = "sdhcp";
pub const RADULA_CERAS_SED: &'static str = "sed";
pub const RADULA_CERAS_SHADOW: &'static str = "shadow";
pub const RADULA_CERAS_SKALIBS: &'static str = "skalibs";

pub const RADULA_CERAS_TOYBOX: &'static str = "toybox";
pub const RADULA_CERAS_TZCODE: &'static str = "tzcode";
pub const RADULA_CERAS_TZDATA: &'static str = "tzdata";

pub const RADULA_CERAS_UTIL_LINUX: &'static str = "util-linux";
pub const RADULA_CERAS_UTMPS: &'static str = "utmps";

pub const RADULA_CERAS_VIM: &'static str = "vim";

pub const RADULA_CERAS_WGET2: &'static str = "wget2";
pub const RADULA_CERAS_WHICH: &'static str = "which";

pub const RADULA_CERAS_XXHASH: &'static str = "xxhash";
pub const RADULA_CERAS_XZ: &'static str = "xz";

pub const RADULA_CERAS_ZLIB_NG: &'static str = "zlib-ng";
pub const RADULA_CERAS_ZSTD: &'static str = "zstd";

//
// Cross
//

pub const RADULA_CROSS_ARCHIVER: &'static str = "gcc-ar";
pub const RADULA_CROSS_ASSEMBLER: &'static str = "as";
pub const RADULA_CROSS_C_COMPILER: &'static str = "gcc";
pub const RADULA_CROSS_C_CXX_COMPILER_LINKER: &'static str = "bfd";
pub const RADULA_CROSS_C_PREPROCESSOR: &'static str = "-E";
pub const RADULA_CROSS_CXX_COMPILER: &'static str = "g++";
pub const RADULA_CROSS_LINKER: &'static str = "ld.bfd";
pub const RADULA_CROSS_NAMES: &'static str = "gcc-nm";
pub const RADULA_CROSS_OBJECT_COPY: &'static str = "objcopy";
pub const RADULA_CROSS_OBJECT_DUMP: &'static str = "objdump";
pub const RADULA_CROSS_RANDOM_ACCESS_LIBRARY: &'static str = "gcc-ranlib";
pub const RADULA_CROSS_READ_ELF: &'static str = "readelf";
pub const RADULA_CROSS_SIZE: &'static str = "size";
pub const RADULA_CROSS_STRINGS: &'static str = "strings";
pub const RADULA_CROSS_STRIP: &'static str = "strip";

//
// Directories
//

pub const RADULA_DIRECTORY_BACKUPS: &'static str = "bak";
pub const RADULA_DIRECTORY_BUILDS: &'static str = "bld";
pub const RADULA_DIRECTORY_CERATA: &'static str = "cerata";
pub const RADULA_DIRECTORY_CROSS: &'static str = "cross";
pub const RADULA_DIRECTORY_GLAUCUS: &'static str = "glaucus";
pub const RADULA_DIRECTORY_LOGS: &'static str = "log";
pub const RADULA_DIRECTORY_SOURCES: &'static str = "src";
pub const RADULA_DIRECTORY_TEMPORARY: &'static str = "tmp";
pub const RADULA_DIRECTORY_TOOLCHAIN: &'static str = "toolchain";

//
// Environment Variables
//

// Architectures
pub const RADULA_ENVIRONMENT_ARCHITECTURE: &'static str = "ARCH";
pub const RADULA_ENVIRONMENT_ARCHITECTURE_CERATA: &'static str = "CARCH";
pub const RADULA_ENVIRONMENT_ARCHITECTURE_FLAGS: &'static str = "FARCH";
pub const RADULA_ENVIRONMENT_ARCHITECTURE_GCC_CONFIGURATION: &'static str = "GCARCH";
pub const RADULA_ENVIRONMENT_ARCHITECTURE_GCOMPAT: &'static str = "GARCH";
pub const RADULA_ENVIRONMENT_ARCHITECTURE_LINUX: &'static str = "LARCH";
pub const RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_CONFIGURATION: &'static str = "LCARCH";
pub const RADULA_ENVIRONMENT_ARCHITECTURE_LINUX_IMAGE: &'static str = "LIARCH";
pub const RADULA_ENVIRONMENT_ARCHITECTURE_MUSL: &'static str = "MARCH";
pub const RADULA_ENVIRONMENT_ARCHITECTURE_MUSL_LINKER: &'static str = "MLARCH";
pub const RADULA_ENVIRONMENT_ARCHITECTURE_UCONTEXT: &'static str = "UARCH";

// Cross
pub const RADULA_ENVIRONMENT_CROSS_ARCHIVER: &'static str = "AR";
pub const RADULA_ENVIRONMENT_CROSS_ASSEMBLER: &'static str = "AS";
pub const RADULA_ENVIRONMENT_CROSS_BUILD_C_COMPILER: &'static str = "BUILD_CC";
pub const RADULA_ENVIRONMENT_CROSS_COMPILE: &'static str = "CROSS_COMPILE";
pub const RADULA_ENVIRONMENT_CROSS_C_COMPILER: &'static str = "CC";
pub const RADULA_ENVIRONMENT_CROSS_C_COMPILER_LINKER: &'static str = "CC_LD";
pub const RADULA_ENVIRONMENT_CROSS_C_PREPROCESSOR: &'static str = "CPP";
pub const RADULA_ENVIRONMENT_CROSS_CXX_COMPILER: &'static str = "CXX";
pub const RADULA_ENVIRONMENT_CROSS_CXX_COMPILER_LINKER: &'static str = "CXX_LD";
pub const RADULA_ENVIRONMENT_CROSS_HOST_C_COMPILER: &'static str = "HOSTCC";
pub const RADULA_ENVIRONMENT_CROSS_HOST_CXX_COMPILER: &'static str = "HOSTCXX";
pub const RADULA_ENVIRONMENT_CROSS_LINKER: &'static str = "LD";
pub const RADULA_ENVIRONMENT_CROSS_NAMES: &'static str = "NM";
pub const RADULA_ENVIRONMENT_CROSS_OBJECT_COPY: &'static str = "OBJCOPY";
pub const RADULA_ENVIRONMENT_CROSS_OBJECT_DUMP: &'static str = "OBJDUMP";
pub const RADULA_ENVIRONMENT_CROSS_RANDOM_ACCESS_LIBRARY: &'static str = "RANLIB";
pub const RADULA_ENVIRONMENT_CROSS_READ_ELF: &'static str = "READELF";
pub const RADULA_ENVIRONMENT_CROSS_SIZE: &'static str = "SIZE";
pub const RADULA_ENVIRONMENT_CROSS_STRINGS: &'static str = "STRINGS";
pub const RADULA_ENVIRONMENT_CROSS_STRIP: &'static str = "STRIP";

// Directories
pub const RADULA_ENVIRONMENT_DIRECTORY_BACKUPS: &'static str = "BAKD";
pub const RADULA_ENVIRONMENT_DIRECTORY_CERATA: &'static str = "CERD";

pub const RADULA_ENVIRONMENT_DIRECTORY_CROSS: &'static str = "CRSD";
pub const RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY: &'static str = "XTMP";
pub const RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY_BUILDS: &'static str = "XBLD";
pub const RADULA_ENVIRONMENT_DIRECTORY_CROSS_TEMPORARY_SOURCES: &'static str = "XSRC";

pub const RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS: &'static str = "GLAD";
pub const RADULA_ENVIRONMENT_DIRECTORY_LOGS: &'static str = "LOGD";
pub const RADULA_ENVIRONMENT_DIRECTORY_SOURCES: &'static str = "SRCD";
pub const RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY: &'static str = "TMPD";

pub const RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN: &'static str = "TLCD";
pub const RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY: &'static str = "TTMP";
pub const RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY_BUILDS: &'static str = "TBLD";
pub const RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN_TEMPORARY_SOURCES: &'static str = "TSRC";

// Files
pub const RADULA_ENVIRONMENT_FILE_CROSS_LOG: &'static str = "XLOG";
pub const RADULA_ENVIRONMENT_FILE_TOOLCHAIN_LOG: &'static str = "TLOG";

// Flags
pub const RADULA_ENVIRONMENT_FLAGS_C_COMPILER: &'static str = "CFLAGS";
pub const RADULA_ENVIRONMENT_FLAGS_CXX_COMPILER: &'static str = "CXXFLAGS";
pub const RADULA_ENVIRONMENT_FLAGS_LINKER: &'static str = "LDFLAGS";

// Paths
pub const RADULA_ENVIRONMENT_PATH: &'static str = "PATH";

// pkg-config
pub const RADULA_ENVIRONMENT_PKG_CONFIG_LIBDIR: &'static str = "PKG_CONFIG_LIBDIR";
pub const RADULA_ENVIRONMENT_PKG_CONFIG_PATH: &'static str = "PKG_CONFIG_PATH";
pub const RADULA_ENVIRONMENT_PKG_CONFIG_SYSROOT_DIR: &'static str = "PKG_CONFIG_SYSROOT_DIR";
pub const RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_INCLUDE_PATH: &'static str =
    "PKG_CONFIG_SYSTEM_INCLUDE_PATH";
pub const RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_LIBRARY_PATH: &'static str =
    "PKG_CONFIG_SYSTEM_LIBRARY_PATH";

// Teeth
pub const RADULA_ENVIRONMENT_TOOTH_AUTORECONF: &'static str = "AUTORECONF";
pub const RADULA_ENVIRONMENT_TOOTH_AWK: &'static str = "AWK";
pub const RADULA_ENVIRONMENT_TOOTH_BISON: &'static str = "BISON";
pub const RADULA_ENVIRONMENT_TOOTH_CHMOD: &'static str = "CHMOD";
pub const RADULA_ENVIRONMENT_TOOTH_CHOWN: &'static str = "CHOWN";
pub const RADULA_ENVIRONMENT_TOOTH_FLEX: &'static str = "FLEX";
pub const RADULA_ENVIRONMENT_TOOTH_GAWK: &'static str = "GAWK";
pub const RADULA_ENVIRONMENT_TOOTH_LEX: &'static str = "LEX";
pub const RADULA_ENVIRONMENT_TOOTH_LN: &'static str = "LN";
pub const RADULA_ENVIRONMENT_TOOTH_MAKE: &'static str = "MAKE";
pub const RADULA_ENVIRONMENT_TOOTH_MAKEFLAGS: &'static str = "MAKEFLAGS";
pub const RADULA_ENVIRONMENT_TOOTH_MKDIR: &'static str = "MKDIR";
pub const RADULA_ENVIRONMENT_TOOTH_MKDIR_P: &'static str = "MKDIR_P";
pub const RADULA_ENVIRONMENT_TOOTH_MV: &'static str = "MV";
pub const RADULA_ENVIRONMENT_TOOTH_PATCH: &'static str = "PATCH";
pub const RADULA_ENVIRONMENT_TOOTH_PKG_CONFIG: &'static str = "PKG_CONFIG";
pub const RADULA_ENVIRONMENT_TOOTH_RM: &'static str = "RM";
pub const RADULA_ENVIRONMENT_TOOTH_RSYNC: &'static str = "RSYNC";
pub const RADULA_ENVIRONMENT_TOOTH_YACC: &'static str = "YACC";

// Tuples
pub const RADULA_ENVIRONMENT_TUPLE_BUILD: &'static str = "BLD";
pub const RADULA_ENVIRONMENT_TUPLE_TARGET: &'static str = "TGT";

//
// Files
//

pub const RADULA_FILE_CONFIG_GUESS: &'static str = "binutils/config.guess";

pub const RADULA_FILE_GLAUCUS_IMAGE: &'static str = "glaucus.img";
pub const RADULA_FILE_GLAUCUS_IMAGE_SIZE: &'static str = "1024M";

pub const RADULA_FILE_SYSLINUX_EXTLINUX_CONF: &'static str = "syslinux/extlinux.conf";
pub const RADULA_FILE_SYSLINUX_MBR_BIN: &'static str = "syslinux/mbr.bin";

//
// Flags
//

pub const RADULA_FLAGS_C_COMPILER: &'static str = "-pipe -g0 -Ofast -fomit-frame-pointer -fmerge-all-constants -fmodulo-sched -fmodulo-sched-allow-regmoves -fgcse-sm -fgcse-las -fdevirtualize-at-ltrans -fira-loop-pressure -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -floop-parallelize-all -fvariable-expansion-in-unroller -falign-functions=32 -flimit-function-alignment -flto=auto -flto-compression-level=19 -fuse-linker-plugin -ftracer -funroll-loops -ffunction-sections -fdata-sections -fno-stack-protector -fno-exceptions -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt";
pub const RADULA_FLAGS_CXX_COMPILER: &'static str =
    "-fno-rtti -fvisibility-inlines-hidden -fvisibility=hidden";
pub const RADULA_FLAGS_LINKER: &'static str = "-Wl,--strip-all -Wl,-z,noexecstack,-z,now,-z,relro -Wl,--as-needed -Wl,--gc-sections -Wl,--sort-common -Wl,--hash-style=gnu";

//
// Help Messages
//

pub const RADULA_HELP: &'static str = "USAGE:
\tradula [ OPTIONS ]

OPTIONS:
\t-b, --behave \tPerform a behavior
\t-c, --ceras  \tDisplay ceras information
\t-h, --help   \tDisplay this help message
\t-v, --version\tDisplay current version number";

pub const RADULA_HELP_BEHAVE: &'static str = "USAGE:
\tradula [ -b | --behave ] [ OPTIONS ]

OPTIONS:
\tb, bootstrap \tPerform bootstrap behavior
\te, envenomate\tPerform envenomate behavior
\th, help      \tDisplay this help message
\ti, binary    \tPerform binary behavior";

pub const RADULA_HELP_BEHAVE_BINARY: &'static str = "USAGE:
\tradula [ -b | --behave ] [ i | binary ] [ OPTIONS ] [ cerata ]

OPTIONS:
\td, decyst    \tRemove binary cerata without preserving their cysts
\th, help      \tDisplay this help message
\ti, install   \tInstall binary cerata (default)
\tr, remove    \tRemove binary cerata while preserving their cyst(s)
\ts, search    \tSearch for binary cerata within the remote repositories
\tu, upgrade   \tUpgrade binary cerata";

pub const RADULA_HELP_BEHAVE_BOOTSTRAP: &'static str = "USAGE:
\tradula [ -b | --behave ] [ b | bootstrap ] [ OPTIONS ]

OPTIONS:
\tc, clean     \tClean up while preserving sources and backups
\td, distclean \tClean up everything
\th, help      \tDisplay this help message
\ti, image     \tCreate an image file of the glaucus system
\tl, list      \tList supported genomes
\tr, require   \tCheck if host has all required packages
\ts, release   \tRelease a compressed tarball of the toolchain
\tt, toolchain \tBootstrap a cross compiler toolchain
\tx, cross     \tBootstrap a cross compiled glaucus system";

pub const RADULA_HELP_BEHAVE_BOOTSTRAP_LIST: &'static str = "GENOMES (ARCHITECTURES):
\taarch64,     \tarm64,       \tarmv8-a
\tarm,         \tarmv6zk,     \tbcm2835
\ti386,        \ti686,        \tx86
\triscv,       \triscv64,     \trv64gc
\tx86-64,      \tx86-64-v3,   \tx86_64     \tx86_64-v3    \tx86_64_v3";

pub const RADULA_HELP_BEHAVE_ENVENOMATE: &'static str = "USAGE:
\tradula [ -b | --behave ] [ e | envenomate ] [ OPTIONS ] [ cerata ]

OPTIONS:
\td, decyst    \tRemove cerata without preserving their cyst(s)
\th, help      \tDisplay this help message
\ti, install   \tInstall cerata from source (default)
\tr, remove    \tRemove cerata while preserving their cyst(s)
\ts, search    \tSearch for cerata within the cerata directory
\tu, upgrade   \tUpgrade cerata";

pub const RADULA_HELP_VERSION: &'static str = "radula version 1.1.3

Copyright (c) 2018-2022, Firas Khalil Khana
Distributed under the terms of the ISC License";

//
// Paths
//

pub const RADULA_PATH_BIN: &'static str = "bin";

pub const RADULA_PATH_BOOT: &'static str = "boot";

// Default `ccache` directories on both Arch and Fedora
pub const RADULA_PATH_CCACHE: &'static str = "/usr/lib/ccache/bin:/usr/lib64/ccache";

pub const RADULA_PATH_ETC: &'static str = "etc";

pub const RADULA_PATH_INFO: &'static str = "info";

pub const RADULA_PATH_LIB: &'static str = "lib";
pub const RADULA_PATH_LIB64: &'static str = "lib64";

pub const RADULA_PATH_LOST_FOUND: &'static str = "lost+found";

pub const RADULA_PATH_MAN: &'static str = "man";

pub const RADULA_PATH_MNT: &'static str = "mnt";

// `pkgconf` and `pkg-config` don't respect the provided sysroot (it doesn't get
// automatically prefixed to PATH and LIBDIR)
pub const RADULA_PATH_PKG_CONFIG_LIBDIR_PATH: &'static str = "/usr/lib/pkgconfig";
pub const RADULA_PATH_PKG_CONFIG_SYSROOT_DIR: &'static str = "/";
pub const RADULA_PATH_PKG_CONFIG_SYSTEM_INCLUDE_PATH: &'static str = "/usr/include";
pub const RADULA_PATH_PKG_CONFIG_SYSTEM_LIBRARY_PATH: &'static str = "/usr/lib";

pub const RADULA_PATH_SHARE: &'static str = "share";

pub const RADULA_PATH_RADULA_CLUSTERS: &'static str = "/var/db/radula/clusters";
pub const RADULA_PATH_RADULA_SOURCES: &'static str = "/var/cache/radula/src";

pub const RADULA_PATH_USR: &'static str = "usr";

pub const RADULA_PATH_UTMPS: &'static str = "s6/current/run-image/utmps";

//
// Teeth
//

// A tooth is a tool available on the host system
pub const RADULA_TOOTH_AUTORECONF: &'static str = "autoreconf";
pub const RADULA_TOOTH_AUTORECONF_FLAGS: &'static str = "-vfis";

// `bash` is only required for building some packages, and isn't actually used
// for running shell scripts (we're using `dash` for that)
pub const RADULA_TOOTH_BASH: &'static str = "bash";

pub const RADULA_TOOTH_BYACC: &'static str = "byacc";

// Both `chmod` and `chown` use the same flags
pub const RADULA_TOOTH_CHMOD: &'static str = "chmod";
pub const RADULA_TOOTH_CHMOD_CHOWN_FLAGS: &'static str = "-Rv";
pub const RADULA_TOOTH_CHOWN: &'static str = "chown";

pub const RADULA_TOOTH_DATE: &'static str = "date";

pub const RADULA_TOOTH_DD: &'static str = "dd";

pub const RADULA_TOOTH_EXTLINUX: &'static str = "extlinux";
pub const RADULA_TOOTH_EXTLINUX_FLAGS: &'static str = "--install";

pub const RADULA_TOOTH_FIND: &'static str = "find";

pub const RADULA_TOOTH_FLEX: &'static str = "flex";

pub const RADULA_TOOTH_GIT: &'static str = "git";

pub const RADULA_TOOTH_LN: &'static str = "ln";
pub const RADULA_TOOTH_LN_FLAGS: &'static str = "-fnsv";

pub const RADULA_TOOTH_LOSETUP: &'static str = "losetup";

pub const RADULA_TOOTH_MAKE: &'static str = "make";
pub const RADULA_TOOTH_MAKEFLAGS: &'static str = "-O V=1";

pub const RADULA_TOOTH_MAWK: &'static str = "mawk";

// Full path is being used here to not overlap with the install() function in stage files
pub const RADULA_TOOTH_MKDIR: &'static str = "/usr/bin/install";
pub const RADULA_TOOTH_MKDIR_FLAGS: &'static str = "-dv";

pub const RADULA_TOOTH_MKE2FS: &'static str = "mke2fs";
pub const RADULA_TOOTH_MKE2FS_FLAGS: &'static str = "-t";

pub const RADULA_TOOTH_MODPROBE: &'static str = "modprobe";

pub const RADULA_TOOTH_MOUNT: &'static str = "mount";

pub const RADULA_TOOTH_MV: &'static str = "mv";
pub const RADULA_TOOTH_MV_FLAGS: &'static str = "-v";

pub const RADULA_TOOTH_PARTED: &'static str = "parted";
pub const RADULA_TOOTH_PARTED_FLAGS: &'static str = "-s";

pub const RADULA_TOOTH_PARTX: &'static str = "partx";

pub const RADULA_TOOTH_PATCH: &'static str = "patch";
pub const RADULA_TOOTH_PATCH_FLAGS: &'static str = "--verbose";

pub const RADULA_TOOTH_PKGCONF: &'static str = "pkgconf";

pub const RADULA_TOOTH_QEMU_IMAGE: &'static str = "qemu-img";

pub const RADULA_TOOTH_RM: &'static str = "rm";
pub const RADULA_TOOTH_RM_FLAGS: &'static str = "-frv";

pub const RADULA_TOOTH_RSYNC: &'static str = "rsync";
pub const RADULA_TOOTH_RSYNC_FLAGS: &'static str = "-vaHAXSx";

// The main shell used for running shell scripts
pub const RADULA_TOOTH_SHELL: &'static str = "dash";
pub const RADULA_TOOTH_SHELL_FLAGS: &'static str = "-c";

pub const RADULA_TOOTH_UMOUNT: &'static str = "umount";
pub const RADULA_TOOTH_UMOUNT_FLAGS: &'static str = "-fqRv";
