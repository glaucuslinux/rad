# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import
  std/[algorithm, os, osproc, sequtils, strformat, strutils, tables, times],
  tools,
  toml_serialization

const
  radPkgCache* = "/var/cache/rad/pkg"
  radSrcCache* = "/var/cache/rad/src"
  radCoreRepo* = "/var/lib/rad/repo/core"
  radExtraRepo* = "/var/lib/rad/repo/extra"
  radLog* = "/var/log/rad"
  radTmp* = "/var/tmp/rad"

type
  Package = object
    ver, url, sum, bld, run*, opt = "nil"

  Stages* = enum
    cross
    native
    toolchain

proc cleanPackages*() =
  removeDir(radTmp)
  createDir(radTmp)

proc parsePackage*(name: string): Package =
  let path = radCoreRepo / name

  if not dirExists(path):
    abort(&"""{"name":8}{&"\{name\} not found":48}""")

  Toml.loadFile(path / "info", Package)

proc printContent(idx: int, name, ver, cmd: string) =
  echo &"""{idx + 1:<8}{name:24}{ver:24}{cmd:8}""" & now().format("hh:mm tt")

proc printHeader() =
  echo """
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
idx     name                    version                 cmd     time    
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"""

proc genContents(dir, contents: string) =
  var entries: seq[string]

  for entry in walkDirRec(
    dir, yieldFilter = {pcFile .. pcLinkToDir}, relative = true, skipSpecial = true
  ):
    entries &= (
      if getFileInfo(dir / entry, followSymlink = false).kind == pcDir: entry & '/'
      else: entry
    )

  let contents = open(contents, fmWrite)

  for entry in entries.sorted():
    contents &= entry & '\n'

  contents.close()

proc isEmpty(dir: string): bool =
  for entry in walkDir(dir):
    return
  return true

proc fetchPackages(packages: openArray[string]) =
  printHeader()

  for idx, name in packages:
    let package = parsePackage(name)

    printContent(idx, name, package.ver, "fetch")

    # Skip virtual packages
    if package.url == "nil":
      continue

    let
      src = radSrcCache / name
      tmp = radTmp / name

    if package.sum == "nil":
      if not dirExists(src):
        discard gitCloneRepo(package.url, src)

        if package.ver != "latest":
          discard gitCheckoutRepo(src, package.ver)

      copyDirWithPermissions(src, tmp)
    else:
      let archive = src / lastPathPart(package.url)

      if not verifyFile(archive, package.sum):
        removeDir(src)
        createDir(src)
        discard downloadFile(src, package.url)

      if verifyFile(archive, package.sum):
        createDir(tmp)
        discard extractTar(archive, tmp)
      else:
        abort(&"""{"sum":8}{name:24}{package.ver:24}""")

proc resolveDeps(
    name: string,
    packages: var seq[string],
    deps: var Table[string, seq[string]],
    run = true,
) =
  if name in packages:
    return

  let
    package = parsePackage(name)
    dep = if run: package.run else: package.bld

  deps[name] =
    if dep == "nil":
      @[]
    else:
      dep.split()

  for name in deps[name]:
    resolveDeps(name, packages, deps, run)

  packages &= name

proc sortPackages(packages: openArray[string], run = true): seq[string] =
  var
    sorted: seq[string]
    deps: Table[string, seq[string]]

  for name in packages.deduplicate():
    resolveDeps(name, sorted, deps, run)

  sorted

proc installPackage(name: string, fs = "/", pkgCache = radPkgCache, implicit = false) =
  let package = parsePackage(name)

  discard extractTar(
    pkgCache / name / name &
      (if package.url == "nil": ""
      else: '-' & package.ver & ".tar.zst"),
    fs,
  )

proc configureArch*() =
  let env = [
    ("ARCH", "x86-64"),
    ("BUILD", execCmdEx(radCoreRepo / "slibtool/files/config.guess").output.strip()),
    ("CTARGET", "x86_64-glaucus-linux-musl"),
    ("PRETTY_NAME", "glaucus s6 x86-64-v3 " & now().format("YYYYMMdd")),
    ("TARGET", "x86_64-pc-linux-musl"),
  ]

  for (i, j) in env:
    putEnv(i, j)

proc configureFlags*(lto = true, parallel = true) =
  let
    cflags =
      if lto:
        "-pipe -O2 -fgraphite-identity -floop-nest-optimize -flto=auto -flto-compression-level=3 -fuse-linker-plugin -fstack-protector-strong -fstack-clash-protection -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -march=x86-64-v3 -mfpmath=sse -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2"
      else:
        "-pipe -O2 -fgraphite-identity -floop-nest-optimize -fstack-protector-strong -fstack-clash-protection -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -march=x86-64-v3 -mfpmath=sse -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2"
    env = [
      ("CFLAGS", cflags),
      ("CXXFLAGS", cflags),
      (
        "LDFLAGS",
        if lto:
          "-Wl,-O1,-s,-z,noexecstack,-z,now,-z,pack-relative-relocs,-z,relro,-z,x86-64-v3,--as-needed,--gc-sections,--sort-common,--hash-style=gnu " &
            cflags
        else:
          "-Wl,-O1,-s,-z,noexecstack,-z,now,-z,pack-relative-relocs,-z,relro,-z,x86-64-v3,--as-needed,--gc-sections,--sort-common,--hash-style=gnu",
      ),
      ("MAKEFLAGS", if parallel: "-j 5 -O" else: "-j 1"),
    ]

  for (i, j) in env:
    putEnv(i, j)

proc buildPackages*(
    packages: openArray[string],
    fs = "/",
    pkgCache = radPkgCache,
    resolve = true,
    stage = native,
    implicit = false,
) =
  let sorted = sortPackages(packages, false)

  fetchPackages(sorted)

  echo ""

  printHeader()

  for idx, name in (if resolve: sorted else: packages.toSeq()):
    let
      package = parsePackage(name)
      archive =
        pkgCache / name / name &
        (if package.url == "nil": ""
        else: '-' & package.ver & ".tar.zst")
      tmp = getEnv("TMPD") / name

    printContent(idx, name, package.ver, "build")

    if stage == native:
      # Skip package if archive exists
      if fileExists(archive):
        # Install build-time dependency (if not installed)
        if implicit:
          installPackage(name, implicit = true)
        continue

      putEnv("DSTD", pkgCache / name / "dst")
      createDir(getEnv("DSTD"))

    if stage != toolchain:
      configureFlags()

      if "no-lto" in package.opt:
        configureFlags(lto = false)

      if "no-parallel" in package.opt:
        configureFlags(parallel = false)

    if dirExists(tmp):
      setCurrentDir(tmp)
    if dirExists(tmp / name & package.ver):
      setCurrentDir(tmp / name & package.ver)

    if stage == native:
      const env = [
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
        ("REPO", radCoreRepo),
        ("TMPD", radTmp),
        ("YACC", "byacc"),
      ]

      for (i, j) in env:
        putEnv(i, j)

    let shell = execCmdEx(
      &"""sh -efu -c '
        name={name} ver={package.ver} . {radCoreRepo / name / (if stage == native: "build" else: "build" & '-' & $stage)}

        for i in prepare configure build; do
          if command -v $i >/dev/null 2>&1; then
            $i
          fi
        done

        package
      '"""
    )

    writeFile(
      radLog / name & (if stage == native: "" else: '.' & $stage), shell.output.strip()
    )

    if shell.exitCode != QuitSuccess:
      abort(&"{shell.exitCode:<8}{name:24}{package.ver:24}")

    if stage == native:
      let
        dst = getEnv("DSTD")
        status = createTarZst(archive, dst)

      # Purge
      # if "empty" notin package.opt:

      if status == QuitSuccess:
        genContents(dst, $pkgCache / name / "contents")
        removeDir(dst)

        copyFileWithPermissions(radCoreRepo / name / "info", pkgCache / name / "info")

      if "bootstrap" in package.opt or implicit:
        installPackage(name, implicit = true)

proc installPackages*(packages: openArray[string], fs = "/", pkgCache = radPkgCache) =
  let sorted = sortPackages(packages)
  var notBuilt: seq[string]

  for idx, name in sorted:
    let
      package = parsePackage(name)
      archive =
        pkgCache / name / name &
        (if package.url == "nil": ""
        else: '-' & package.ver & ".tar.zst")

    if not fileExists(archive):
      notBuilt &= name

  if notBuilt.len() > 0:
    buildPackages(notBuilt, implicit = true)

    echo ""

  printHeader()

  for idx, name in sorted:
    let package = parsePackage(name)

    printContent(idx, name, package.ver, "install")

    discard extractTar(
      pkgCache / name / name &
        (if package.url == "nil": ""
        else: '-' & package.ver & ".tar.zst"),
      fs,
    )

proc showInfo*(packages: openArray[string]) =
  for name in packages.deduplicate():
    let package = parsePackage(name)

    echo &"""
nom  :: {name}
ver  :: {package.ver}
url  :: {package.url}
sum  :: {package.sum}
bld  :: {package.bld}
run  :: {package.run}
opt  :: {package.opt}
"""

proc listPackages*() =
  showInfo(walkDir(radPkgCache, true, skipSpecial = true).toSeq().unzip()[1].sorted())

proc listContents*(packages: openArray[string]) =
  for name in packages.deduplicate():
    let package = parsePackage(name)

    for line in lines(radPkgCache / name / "contents"):
      echo &"/{line}"

proc searchPackages*(pattern: openArray[string]) =
  var packages: seq[string]

  for package in walkDir(radCoreRepo, true, skipSpecial = true):
    for name in pattern:
      if name.toLowerAscii() in package[1]:
        packages &= package[1]

  if packages.len() == 0:
    exit(status = QuitFailure)

  showInfo(packages.sorted())
