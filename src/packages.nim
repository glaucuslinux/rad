proc installPackage(nom: string, fs = "/", pkgCache = pkgCache) =
  let package = parseInfo(nom)

  discard extractTar(
    pkgCache / nom / nom &
      (if package.url == "nil": ""
      else: '-' & package.ver & ".tar.zst"),
    fs,
  )

proc buildPackages*(packages: openArray[string], bootstrap = false, stage = native) =
  let sorted = sortPackages(packages)

  fetchPackages(sorted)

  for idx, nom in queue:
    let package = parseInfo(nom)
    let archive =
      pkgCache / nom / nom &
      (if package.url == "nil": ".tar.zst"
      else: '-' & package.ver & ".tar.zst")

    if stage == native:
      # Skip package if archive exists
      if fileExists(archive):
        continue

      const envExes = [
        ("AWK", "mawk"),
        ("LEX", "flex"),
        ("LIBTOOL", "slibtool"),
        ("PKG_CONFIG", "u-config"),
        ("YACC", "byacc"),
      ]

    let cflags =
      # "-pipe -Os -fgcse-las" & (
      #   if "no-lto" notin package.opt:
      #     " -flto=auto -fuse-linker-plugin "
      #   else:
      #     " "
      # ) &
      # "-ffunction-sections -fdata-sections -fstack-protector-strong -fstack-clash-protection -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-ident -fno-plt -march=x86-64-v3 -mtls-dialect=gnu2"
      "-pipe -Os -target=x86_64-glaucus-linux-musl --sysroot=/home/firasuke/Git/glaucus/cross --rtlib=compiler-rt -unwindlib=libunwind --stdlib=libc++"

    let env = [
      ("ARCH", "x86-64"),
      ("CFLAGS", cflags),
      ("CXXFLAGS", cflags),
      (
        "LDFLAGS",
        # "-Wl,-O1,-s,-z,defs,-z,noexecstack,-z,now,-z,pack-relative-relocs,-z,relro,-z,separate-code,-z,text,--as-needed,--gc-sections,--no-keep-memory,--relax,--sort-common,--enable-new-dtags,--hash-style=gnu,--build-id=none" &
        # (
        #   if "no-lto" notin package.opt:
        #     " " & cflags
        #   else:
        #     ""
        # ),
        "-target=x86_64-glaucus-linux-musl --sysroot=/home/firasuke/Git/glaucus/cross --rtlib=compiler-rt -unwindlib=libunwind --stdlib=libc++"
      ),
    ]
