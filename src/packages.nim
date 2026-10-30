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
        ("AWK", "wak"),
        ("LEX", "flex"),
        ("LIBTOOL", "slibtool"),
        ("PKG_CONFIG", "u-config"),
        ("YACC", "byacc"),
      ]

    let cflags =
      # "-O2" & (
      #   if "no-lto" notin package.opt:
      #     " -flto=thin "
      #   else:
      #     " "
      # ) &
      # "-march=x86-64-v3 -mtls-dialect=gnu2 -fdata-sections -ffunction-sections -fno-plt -fstack-clash-protection -fstack-protector-strong -fno-asynchronous-unwind-tables -fno-unwind-tables -fno-ident"
      "-O2 -target=x86_64-glaucus-linux-musl --sysroot=/home/firasuke/Git/glaucus/cross --rtlib=compiler-rt -unwindlib=libunwind --stdlib=libc++"

    let env = [
      ("ARCH", "x86-64"),
      ("CFLAGS", cflags),
      ("CXXFLAGS", cflags),
      (
        "LDFLAGS",
        # "-Wl,-O2,--as-needed,--gc-sections,--icf=safe,-z,defs,-z,now,-z,separate-code,--build-id=none,--hash-style=gnu,--pack-dyn-relocs=relr,--strip-all" &
        # (
        #   if "no-lto" notin package.opt:
        #     " " & cflags
        #   else:
        #     ""
        # ),
        "-target=x86_64-glaucus-linux-musl --sysroot=/home/firasuke/Git/glaucus/cross --rtlib=compiler-rt -unwindlib=libunwind --stdlib=libc++"
      ),
    ]
