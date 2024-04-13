# Copyright (c) 2018-2024, Firas Khalil Khana
# Distributed under the terms of the ISC License

import std/[
  os,
  osproc,
  strformat,
  strutils,
  terminal,
  times
]

import
  ceras,
  constants,
  teeth

proc radula_bootstrap_clean*() =
  removeDir(getEnv(RADULA_ENVIRONMENT_DIRECTORY_CROSS))
  removeDir(getEnv(RADULA_ENVIRONMENT_DIRECTORY_LOGS))
  removeDir(getEnv(RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY_BUILDS))
  removeDir(getEnv(RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN))

proc radula_bootstrap_cross_backup*() =
  discard radula_rsync(getEnv(RADULA_ENVIRONMENT_DIRECTORY_LOGS), getEnv(RADULA_ENVIRONMENT_DIRECTORY_BACKUPS))

proc radula_bootstrap_cross_envenomate*() =
  radula_ceras_envenomate([
    # Filesystem
    RADULA_CERAS_HYDROSKELETON,

    # Package Management
    RADULA_CERAS_CERATA,
    RADULA_CERAS_RADULA,

    # Compatibility
    RADULA_CERAS_MUSL_FTS,
    RADULA_CERAS_MUSL_UTILS,
    RADULA_CERAS_LINUX_HEADERS,

    # Init
    RADULA_CERAS_SKALIBS,
    RADULA_CERAS_NSSS,
    RADULA_CERAS_EXECLINE,
    RADULA_CERAS_MDEVD,
    RADULA_CERAS_S6,
    RADULA_CERAS_UTMPS,

    # Permissions & Capabilities
    RADULA_CERAS_ATTR,
    RADULA_CERAS_ACL,
    RADULA_CERAS_LIBCAP,
    RADULA_CERAS_LIBCAP_NG,
    RADULA_CERAS_SHADOW,

    # Hashing
    RADULA_CERAS_LIBRESSL,
    RADULA_CERAS_XXHASH,

    # Userland
    RADULA_CERAS_TOYBOX,
    RADULA_CERAS_DIFFUTILS,
    RADULA_CERAS_FILE,
    RADULA_CERAS_FINDUTILS,
    RADULA_CERAS_SED,

    # Development
    RADULA_CERAS_EXPAT,

    # Compression
    RADULA_CERAS_BZIP2,
    RADULA_CERAS_LZ4,
    RADULA_CERAS_XZ,
    RADULA_CERAS_ZLIB_NG,
    RADULA_CERAS_PIGZ,
    RADULA_CERAS_ZSTD,
    RADULA_CERAS_LIBARCHIVE,

    # Development
    RADULA_CERAS_AUTOCONF,
    RADULA_CERAS_AUTOMAKE,
    RADULA_CERAS_BINUTILS,
    RADULA_CERAS_BYACC,
    RADULA_CERAS_FLEX,
    RADULA_CERAS_GCC,
    RADULA_CERAS_GETTEXT_TINY,
    RADULA_CERAS_HELP2MAN,
    RADULA_CERAS_LIBTOOL,
    RADULA_CERAS_M4,
    RADULA_CERAS_MAKE,
    RADULA_CERAS_MAWK,
    RADULA_CERAS_PATCH,
    RADULA_CERAS_PKGCONF,
    RADULA_CERAS_MUON,
    RADULA_CERAS_RSYNC,
    RADULA_CERAS_SAMURAI,

    # Editors, Pagers and Shells
    RADULA_CERAS_NETBSD_CURSES,
    RADULA_CERAS_LIBEDIT,
    RADULA_CERAS_PCRE2,
    RADULA_CERAS_BASH,

    # Userland
    RADULA_CERAS_GREP,

    # Utilities
    RADULA_CERAS_KMOD,
    RADULA_CERAS_LIBUDEV_ZERO,
    RADULA_CERAS_PROCPS_NG,
    RADULA_CERAS_PSMISC,
    RADULA_CERAS_UTIL_LINUX,
    RADULA_CERAS_E2FSPROGS,

    # Services
    RADULA_CERAS_S6_LINUX_INIT,
    RADULA_CERAS_S6_RC,
    RADULA_CERAS_S6_BOOT_SCRIPTS,

    # Kernel
    RADULA_CERAS_LINUX
  ], RADULA_DIRECTORY_CROSS, false)

proc radula_bootstrap_cross_environment_pkg_config*() =
  putEnv(RADULA_ENVIRONMENT_PKG_CONFIG_LIBDIR, getEnv(RADULA_ENVIRONMENT_DIRECTORY_CROSS) / RADULA_PATH_PKG_CONFIG_LIBDIR_PATH)
  putEnv(RADULA_ENVIRONMENT_PKG_CONFIG_PATH, getEnv(RADULA_ENVIRONMENT_PKG_CONFIG_LIBDIR))
  putEnv(RADULA_ENVIRONMENT_PKG_CONFIG_SYSROOT_DIR, getEnv(RADULA_ENVIRONMENT_DIRECTORY_CROSS) / RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)

  # These environment variables are `pkgconf` specific, but setting them won't
  # do any harm...
  putEnv(RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_INCLUDE_PATH, getEnv(RADULA_ENVIRONMENT_DIRECTORY_CROSS) / RADULA_PATH_PKG_CONFIG_SYSTEM_INCLUDE_PATH)
  putEnv(RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_LIBRARY_PATH, getEnv(RADULA_ENVIRONMENT_DIRECTORY_CROSS) / RADULA_PATH_PKG_CONFIG_SYSTEM_LIBRARY_PATH)

proc radula_bootstrap_cross_environment_teeth*() =
  let cross_compile = getEnv(RADULA_ENVIRONMENT_TUPLE_TARGET) & '-'

  putEnv(RADULA_ENVIRONMENT_CROSS_COMPILE, cross_compile)

  putEnv(RADULA_ENVIRONMENT_TOOTH_ARCHIVER, cross_compile & RADULA_TOOTH_ARCHIVER)
  putEnv(RADULA_ENVIRONMENT_TOOTH_ASSEMBLER, cross_compile & RADULA_TOOTH_ASSEMBLER)
  putEnv(RADULA_ENVIRONMENT_TOOTH_C_COMPILER, cross_compile & RADULA_CERAS_GCC)
  putEnv(RADULA_ENVIRONMENT_TOOTH_C_PREPROCESSOR, cross_compile & RADULA_CERAS_GCC & ' ' & RADULA_TOOTH_C_CXX_PREPROCESSOR)
  putEnv(RADULA_ENVIRONMENT_TOOTH_CXX_COMPILER, cross_compile & RADULA_TOOTH_CXX_COMPILER)
  putEnv(RADULA_ENVIRONMENT_TOOTH_CXX_PREPROCESSOR, cross_compile & RADULA_TOOTH_CXX_COMPILER & ' ' & RADULA_TOOTH_C_CXX_PREPROCESSOR)
  putEnv(RADULA_ENVIRONMENT_TOOTH_HOST_C_COMPILER, RADULA_CERAS_GCC)
  putEnv(RADULA_ENVIRONMENT_TOOTH_NAMES, cross_compile & RADULA_TOOTH_NAMES)
  putEnv(RADULA_ENVIRONMENT_TOOTH_OBJECT_COPY, cross_compile & RADULA_TOOTH_OBJECT_COPY)
  putEnv(RADULA_ENVIRONMENT_TOOTH_OBJECT_DUMP, cross_compile & RADULA_TOOTH_OBJECT_DUMP)
  putEnv(RADULA_ENVIRONMENT_TOOTH_RANDOM_ACCESS_LIBRARY, cross_compile & RADULA_TOOTH_RANDOM_ACCESS_LIBRARY)
  putEnv(RADULA_ENVIRONMENT_TOOTH_READ_ELF, cross_compile & RADULA_TOOTH_READ_ELF)
  putEnv(RADULA_ENVIRONMENT_TOOTH_SIZE, cross_compile & RADULA_TOOTH_SIZE)
  putEnv(RADULA_ENVIRONMENT_TOOTH_STRIP, cross_compile & RADULA_TOOTH_STRIP)

proc radula_bootstrap_cross_prepare*() =
  discard radula_rsync(getEnv(RADULA_ENVIRONMENT_DIRECTORY_BACKUPS) / RADULA_DIRECTORY_CROSS, getEnv(RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS))

  removeDir(getEnv(RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY_BUILDS))
  createDir(getEnv(RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY_BUILDS))

  removeFile(getEnv(RADULA_ENVIRONMENT_FILE_CROSS_LOG))

proc radula_bootstrap_environment*() =
  let path = parentDir(getCurrentDir())

  putEnv(RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS, path)

  putEnv(RADULA_ENVIRONMENT_DIRECTORY_BACKUPS, path / RADULA_DIRECTORY_BACKUPS)
  putEnv(RADULA_ENVIRONMENT_DIRECTORY_CACHE_SOURCES, path / RADULA_DIRECTORY_SOURCES)
  putEnv(RADULA_ENVIRONMENT_DIRECTORY_CERATA, path / RADULA_DIRECTORY_CERATA)
  putEnv(RADULA_ENVIRONMENT_DIRECTORY_CROSS, path / RADULA_DIRECTORY_CROSS)
  putEnv(RADULA_ENVIRONMENT_DIRECTORY_ISO, path / RADULA_DIRECTORY_ISO)
  putEnv(RADULA_ENVIRONMENT_DIRECTORY_LOGS, path / RADULA_DIRECTORY_LOGS)
  putEnv(RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY_BUILDS, path / RADULA_DIRECTORY_TEMPORARY / RADULA_DIRECTORY_BUILDS)
  putEnv(RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY_SOURCES, path / RADULA_DIRECTORY_TEMPORARY / RADULA_DIRECTORY_SOURCES)
  putEnv(RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN, path / RADULA_DIRECTORY_TOOLCHAIN)

  putEnv(RADULA_ENVIRONMENT_FILE_CROSS_LOG, getEnv(RADULA_ENVIRONMENT_DIRECTORY_LOGS) / RADULA_DIRECTORY_CROSS & CurDir & RADULA_DIRECTORY_LOGS)
  putEnv(RADULA_ENVIRONMENT_FILE_TOOLCHAIN_LOG, getEnv(RADULA_ENVIRONMENT_DIRECTORY_LOGS) / RADULA_DIRECTORY_TOOLCHAIN & CurDir & RADULA_DIRECTORY_LOGS)

  putEnv(RADULA_ENVIRONMENT_PATH, getEnv(RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN) / RADULA_PATH_USR / RADULA_PATH_BIN & ':' & getEnv(RADULA_ENVIRONMENT_PATH))

proc radula_bootstrap_initialize*() =
  createDir(getEnv(RADULA_ENVIRONMENT_DIRECTORY_BACKUPS))
  createDir(getEnv(RADULA_ENVIRONMENT_DIRECTORY_CROSS))
  createDir(getEnv(RADULA_ENVIRONMENT_DIRECTORY_CACHE_SOURCES))
  createDir(getEnv(RADULA_ENVIRONMENT_DIRECTORY_LOGS))
  createDir(getEnv(RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY_BUILDS))
  createDir(getEnv(RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY_SOURCES))
  createDir(getEnv(RADULA_ENVIRONMENT_DIRECTORY_TOOLCHAIN))

proc radula_bootstrap_release_img*(compress = false) =
  if not isAdmin():
    styled_echo fg_red, style_bright, &"{\"Abort\":13} :! {\"permission denied\":48}{\"1\":13}{now().format(\"hh:mm:ss tt\")}", reset_style

    radula_exit(QuitFailure)

  let img = getEnv(RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS) / &"{RADULA_DIRECTORY_GLAUCUS}-{RADULA_CERAS_S6}-{RADULA_GENOME_X86_64_V3}-{now().format(\"YYYYMMdd\")}.img"

  # Create a new IMG file
  discard execCmd(&"{RADULA_TOOTH_DD} bs=1M count={RADULA_FILE_GLAUCUS_IMG_SIZE} if=/dev/zero of={img} {RADULA_TOOTH_SHELL_REDIRECTION}")

  # Partition the IMG file
  discard execCmd(&"{RADULA_TOOTH_PARTED} {RADULA_TOOTH_PARTED_FLAGS} {img} mklabel msdos {RADULA_TOOTH_SHELL_REDIRECTION}")
  discard execCmd(&"{RADULA_TOOTH_PARTED} {RADULA_TOOTH_PARTED_FLAGS} -a none {img} mkpart primary ext4 1 {RADULA_FILE_GLAUCUS_IMG_SIZE} {RADULA_TOOTH_SHELL_REDIRECTION}")
  discard execCmd(&"{RADULA_TOOTH_PARTED} {RADULA_TOOTH_PARTED_FLAGS} -a none {img} set 1 boot on {RADULA_TOOTH_SHELL_REDIRECTION}")

  # Load the `loop` module
  discard execCmd(&"{RADULA_TOOTH_MODPROBE} loop {RADULA_TOOTH_SHELL_REDIRECTION}")

  # Detach all used loop devices
  discard execCmd(&"{RADULA_TOOTH_LOSETUP} -D {RADULA_TOOTH_SHELL_REDIRECTION}")

  # Find the first unused loop device
  let
    device = execCmdEx(&"{RADULA_TOOTH_LOSETUP} -f")[0].strip()
    partition = device & "p1"

  # Associate the first unused loop device with the IMG file
  discard execCmd(&"{RADULA_TOOTH_LOSETUP} {device} {img} {RADULA_TOOTH_SHELL_REDIRECTION}")

  # Notify the kernel about the new partition on the IMG file
  discard execCmd(&"{RADULA_TOOTH_PARTX} -a {device} {RADULA_TOOTH_SHELL_REDIRECTION}")

  # Create an `ext4` file system in the partition
  discard execCmd(&"{RADULA_TOOTH_MKE2FS} {RADULA_TOOTH_MKE2FS_FLAGS} ext4 {partition}")

  let mount = RADULA_PATH_PKG_CONFIG_SYSROOT_DIR / RADULA_PATH_MNT / RADULA_DIRECTORY_GLAUCUS

  createDir(mount)

  discard execCmd(&"{RADULA_TOOTH_MOUNT} {partition} {mount} {RADULA_TOOTH_SHELL_REDIRECTION}")

  # Remove `/lost+found` directory
  removeDir(mount / RADULA_PATH_LOST_FOUND)

  discard radula_rsync(getEnv(RADULA_ENVIRONMENT_DIRECTORY_CROSS) / RADULA_PATH_PKG_CONFIG_SYSROOT_DIR, mount, RADULA_TOOTH_RSYNC_RELEASE_FLAGS)

  let path = mount / RADULA_PATH_BOOT

  # Generate initramfs
  radula_generate_initramfs(path, true)

  # Install `grub` as the default bootloader
  createDir(path / RADULA_CERAS_GRUB)

  discard radula_rsync(RADULA_PATH_RADULA_CLUSTERS_GLAUCUS / RADULA_CERAS_GRUB / RADULA_FILE_GRUB_CONF_IMG, path / RADULA_CERAS_GRUB / RADULA_FILE_GRUB_CONF, RADULA_TOOTH_RSYNC_RELEASE_FLAGS)

  discard execCmd(&"{RADULA_TOOTH_GRUB_INSTALL} {RADULA_TOOTH_GRUB_FLAGS} --boot-directory={mount / RADULA_PATH_BOOT} --target=i386-pc {device} {RADULA_TOOTH_SHELL_REDIRECTION}")

  # Change ownerships
  discard execCmd(&"{RADULA_TOOTH_CHOWN} {RADULA_TOOTH_CHOWN_FLAGS} 0:0 {mount} {RADULA_TOOTH_SHELL_REDIRECTION}")
  discard execCmd(&"{RADULA_TOOTH_CHOWN} {RADULA_TOOTH_CHOWN_FLAGS} 20:20 {mount / RADULA_PATH_VAR / RADULA_DIRECTORY_LOGS / RADULA_PATH_WTMPD} {RADULA_TOOTH_SHELL_REDIRECTION}")

  # Clean up
  discard execCmd(&"{RADULA_TOOTH_UMOUNT} {RADULA_TOOTH_UMOUNT_FLAGS} {mount} {RADULA_TOOTH_SHELL_REDIRECTION}")
  discard execCmd(&"{RADULA_TOOTH_PARTX} -d {partition} {RADULA_TOOTH_SHELL_REDIRECTION}")
  discard execCmd(&"{RADULA_TOOTH_LOSETUP} -d {device} {RADULA_TOOTH_SHELL_REDIRECTION}")

  # Compress the IMG file
  if compress:
    let status = radula_compress_zstd(img)

    if status == 0:
      removeFile(img)

proc radula_bootstrap_release_iso*() =
  let
    name = &"{RADULA_DIRECTORY_GLAUCUS}-{RADULA_CERAS_S6}-{RADULA_GENOME_X86_64_V3}-{now().format(\"YYYYMMdd\")}"
    iso = getEnv(RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS) / &"{name}.iso"

    path = getEnv(RADULA_ENVIRONMENT_DIRECTORY_ISO) / RADULA_PATH_BOOT

  # Install `grub` as the default bootloader
  removeDir(getEnv(RADULA_ENVIRONMENT_DIRECTORY_ISO))
  createDir(path / RADULA_CERAS_GRUB)

  discard radula_rsync(RADULA_PATH_RADULA_CLUSTERS_GLAUCUS / RADULA_CERAS_GRUB / RADULA_FILE_GRUB_CONF_ISO, path / RADULA_CERAS_GRUB / RADULA_FILE_GRUB_CONF)

  # Generate initramfs
  discard radula_rsync(getEnv(RADULA_ENVIRONMENT_DIRECTORY_GLAUCUS) / RADULA_FILE_INITRAMFS_GLAUCUS, path)

  # Compress rootfs
  discard execCmd(&"{RADULA_TOOTH_MKFS_EROFS} {path / RADULA_FILE_ROOT_FILE_SYSTEM} {getEnv(RADULA_ENVIRONMENT_DIRECTORY_CROSS)} {RADULA_TOOTH_SHELL_REDIRECTION}")

  # Copy kernel
  discard radula_rsync(getEnv(RADULA_ENVIRONMENT_DIRECTORY_CROSS) / RADULA_PATH_BOOT / RADULA_FILE_KERNEL, path)

  # Create a new ISO file
  discard execCmd(&"{RADULA_TOOTH_GRUB_MKRESCUE} {RADULA_TOOTH_GRUB_FLAGS} -v -o {iso} {getEnv(RADULA_ENVIRONMENT_DIRECTORY_ISO)} -volid GLAUCUS {RADULA_TOOTH_SHELL_REDIRECTION}")

proc radula_bootstrap_system_environment_directories*() =
  putEnv(RADULA_ENVIRONMENT_DIRECTORY_CACHE_SOURCES, RADULA_PATH_RADULA_CACHE_SOURCES)
  putEnv(RADULA_ENVIRONMENT_DIRECTORY_CERATA, RADULA_PATH_RADULA_CLUSTERS_GLAUCUS)
  putEnv(RADULA_ENVIRONMENT_DIRECTORY_LOGS, RADULA_PATH_RADULA_LOGS)

  putEnv(RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY_BUILDS, RADULA_PATH_RADULA_TEMPORARY / RADULA_DIRECTORY_BUILDS)
  putEnv(RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY_SOURCES, RADULA_PATH_RADULA_TEMPORARY / RADULA_DIRECTORY_SOURCES)

  # system log file
  putEnv(RADULA_ENVIRONMENT_FILE_SYSTEM_LOG, getEnv(RADULA_ENVIRONMENT_DIRECTORY_LOGS) / RADULA_DIRECTORY_SYSTEM & CurDir & RADULA_DIRECTORY_LOGS)

proc radula_bootstrap_system_environment_pkg_config*() =
  putEnv(RADULA_ENVIRONMENT_PKG_CONFIG_LIBDIR, RADULA_PATH_PKG_CONFIG_LIBDIR_PATH)
  putEnv(RADULA_ENVIRONMENT_PKG_CONFIG_PATH, getEnv(RADULA_ENVIRONMENT_PKG_CONFIG_LIBDIR))
  putEnv(RADULA_ENVIRONMENT_PKG_CONFIG_SYSROOT_DIR, RADULA_PATH_PKG_CONFIG_SYSROOT_DIR)

  # These environment variables are `pkgconf` specific, but setting them won't
  # do any harm...
  putEnv(RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_INCLUDE_PATH, RADULA_PATH_PKG_CONFIG_SYSTEM_INCLUDE_PATH)
  putEnv(RADULA_ENVIRONMENT_PKG_CONFIG_SYSTEM_LIBRARY_PATH, RADULA_PATH_PKG_CONFIG_SYSTEM_LIBRARY_PATH)

proc radula_bootstrap_system_environment_teeth*() =
  putEnv(RADULA_ENVIRONMENT_BOOTSTRAP, "yes")

  putEnv(RADULA_ENVIRONMENT_TOOTH_ARCHIVER, RADULA_TOOTH_ARCHIVER)
  putEnv(RADULA_ENVIRONMENT_TOOTH_ASSEMBLER, RADULA_TOOTH_ASSEMBLER)
  putEnv(RADULA_ENVIRONMENT_TOOTH_C_COMPILER, RADULA_CERAS_GCC)
  putEnv(RADULA_ENVIRONMENT_TOOTH_C_PREPROCESSOR, RADULA_CERAS_GCC & ' ' & RADULA_TOOTH_C_CXX_PREPROCESSOR)
  putEnv(RADULA_ENVIRONMENT_TOOTH_CXX_COMPILER, RADULA_TOOTH_CXX_COMPILER)
  putEnv(RADULA_ENVIRONMENT_TOOTH_CXX_PREPROCESSOR, RADULA_TOOTH_CXX_COMPILER & ' ' & RADULA_TOOTH_C_CXX_PREPROCESSOR)
  putEnv(RADULA_ENVIRONMENT_TOOTH_HOST_C_COMPILER, RADULA_CERAS_GCC)
  putEnv(RADULA_ENVIRONMENT_TOOTH_NAMES, RADULA_TOOTH_NAMES)
  putEnv(RADULA_ENVIRONMENT_TOOTH_OBJECT_COPY, RADULA_TOOTH_OBJECT_COPY)
  putEnv(RADULA_ENVIRONMENT_TOOTH_OBJECT_DUMP, RADULA_TOOTH_OBJECT_DUMP)
  putEnv(RADULA_ENVIRONMENT_TOOTH_RANDOM_ACCESS_LIBRARY, RADULA_TOOTH_RANDOM_ACCESS_LIBRARY)
  putEnv(RADULA_ENVIRONMENT_TOOTH_READ_ELF, RADULA_TOOTH_READ_ELF)
  putEnv(RADULA_ENVIRONMENT_TOOTH_SIZE, RADULA_TOOTH_SIZE)
  putEnv(RADULA_ENVIRONMENT_TOOTH_STRIP, RADULA_TOOTH_STRIP)

proc radula_bootstrap_system_prepare*() =
  removeDir(getEnv(RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY_BUILDS))
  createDir(getEnv(RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY_BUILDS))

  # Create the `src` directory if it doesn't exist, but don't remove it if it does exist!
  createDir(getEnv(RADULA_ENVIRONMENT_DIRECTORY_TEMPORARY_SOURCES))

  removeFile(getEnv(RADULA_ENVIRONMENT_FILE_SYSTEM_LOG))

proc radula_bootstrap_system_envenomate*() =
  radula_ceras_envenomate([
    # Filesystem
    RADULA_CERAS_HYDROSKELETON,

    # Development
    RADULA_CERAS_MUSL,
    RADULA_CERAS_CMAKE,
    RADULA_CERAS_GMP,
    RADULA_CERAS_MPFR,
    RADULA_CERAS_MPC,
    RADULA_CERAS_ISL,
    RADULA_CERAS_PERL,
    RADULA_CERAS_TEXINFO,

    # Package Management
    RADULA_CERAS_IANA_ETC,
    RADULA_CERAS_CERATA,
    RADULA_CERAS_RADULA,

    # Compatibility
    RADULA_CERAS_MUSL_FTS,
    RADULA_CERAS_MUSL_UTILS,
    RADULA_CERAS_LINUX_HEADERS,

    # Permissions & Capabilities
    RADULA_CERAS_ATTR,
    RADULA_CERAS_ACL,
    RADULA_CERAS_LIBCAP,
    RADULA_CERAS_LIBCAP_NG,
    RADULA_CERAS_OPENDOAS,
    RADULA_CERAS_SHADOW,

    # Hashing
    RADULA_CERAS_LIBRESSL,
    RADULA_CERAS_XXHASH,

    # Userland
    RADULA_CERAS_DIFFUTILS,
    RADULA_CERAS_FILE,
    RADULA_CERAS_FINDUTILS,
    RADULA_CERAS_SED,
    RADULA_CERAS_TOYBOX,
    RADULA_CERAS_UGREP,

    # Compression
    RADULA_CERAS_BZIP2,
    RADULA_CERAS_LZ4,
    RADULA_CERAS_XZ,
    RADULA_CERAS_ZLIB_NG,
    RADULA_CERAS_PIGZ,
    RADULA_CERAS_ZSTD,
    RADULA_CERAS_LIBARCHIVE,

    # Development
    RADULA_CERAS_AUTOCONF,
    RADULA_CERAS_AUTOMAKE,
    RADULA_CERAS_BINUTILS,
    RADULA_CERAS_BYACC,
    RADULA_CERAS_EXPAT,
    RADULA_CERAS_FLEX,
    RADULA_CERAS_GCC,
    RADULA_CERAS_GETTEXT_TINY,
    RADULA_CERAS_HELP2MAN,
    RADULA_CERAS_LIBTOOL,
    RADULA_CERAS_M4,
    RADULA_CERAS_MAKE,
    RADULA_CERAS_MAWK,
    RADULA_CERAS_PATCH,
    RADULA_CERAS_PKGCONF,
    RADULA_CERAS_RSYNC,
    RADULA_CERAS_SAMURAI,

    # Editors, Pagers and Shells
    RADULA_CERAS_NETBSD_CURSES,
    RADULA_CERAS_LIBEDIT,
    RADULA_CERAS_PCRE2,
    RADULA_CERAS_BASH,
    RADULA_CERAS_LESS,
    RADULA_CERAS_MANDOC,
    RADULA_CERAS_VIM,

    # Networking
    RADULA_CERAS_IPROUTE2,
    RADULA_CERAS_IPUTILS,
    RADULA_CERAS_SDHCP,
    RADULA_CERAS_WGET2,

    # Utilities
    RADULA_CERAS_KMOD,
    RADULA_CERAS_LIBUDEV_ZERO,
    RADULA_CERAS_PROCPS_NG,
    RADULA_CERAS_PSMISC,
    RADULA_CERAS_UTIL_LINUX,
    RADULA_CERAS_E2FSPROGS,

    # Init & Services
    RADULA_CERAS_SKALIBS,
    RADULA_CERAS_NSSS,
    RADULA_CERAS_EXECLINE,
    RADULA_CERAS_MDEVD,
    RADULA_CERAS_S6,
    RADULA_CERAS_UTMPS,
    RADULA_CERAS_S6_LINUX_INIT,
    RADULA_CERAS_S6_RC,
    RADULA_CERAS_S6_BOOT_SCRIPTS,

    # Kernel
    RADULA_CERAS_LINUX
  ], RADULA_DIRECTORY_SYSTEM, false)

proc radula_bootstrap_toolchain_backup*() =
  discard radula_rsync(getEnv(RADULA_ENVIRONMENT_DIRECTORY_CROSS), getEnv(RADULA_ENVIRONMENT_DIRECTORY_BACKUPS))
  discard radula_rsync(getEnv(RADULA_ENVIRONMENT_DIRECTORY_LOGS), getEnv(RADULA_ENVIRONMENT_DIRECTORY_BACKUPS))

proc radula_bootstrap_toolchain_envenomate*() =
  radula_ceras_envenomate([
    RADULA_CERAS_MUSL_HEADERS,
    RADULA_CERAS_BINUTILS,
    RADULA_CERAS_GCC,
    RADULA_CERAS_MUSL,
    RADULA_CERAS_LIBGCC,
    RADULA_CERAS_LIBSTDCXX_V3
  ], RADULA_DIRECTORY_TOOLCHAIN, false)
