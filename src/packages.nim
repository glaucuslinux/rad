# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/[algorithm, os, osproc, sequtils, strformat, strutils, tables, times], utils

const
  pkgCache* = "/var/cache/rad/pkg"
  srcCache* = "/var/cache/rad/src"
  coreRepo* = "/var/lib/rad/repo/core"
  radLog* = "/var/log/rad"
  radTmp* = "/var/tmp/rad"

type
  Package = object
    ver, url, sum, bld, run*, opt = "nil"

  Stages* = enum
    cross
    native
    toolchain

proc cleanCache*() =
  removeDir(radTmp)
  createDir(radTmp)

proc parseInfo*(name: string): Package =
  let path = coreRepo / name

  if not dirExists(path):
    abort(&"""{"name":8}{&"\{name\} not found":48}""")

  result = Package()

  for line in (path / "info").lines:
    if line.isEmptyOrWhitespace() or line.startsWith('#'):
      continue

    if line.contains("= \"") or line.contains(" =\"") or '=' notin line:
      abort(&"""{"name":8}{"whitespace found":48}""")

    let
      pair = line.split('=', 1)
      key = pair[0]
    var val = pair[1]

    if not (val.startsWith('"') and val.endsWith('"')):
      abort(&"""{"name":8}{"quotes not found":48}""")

    val = val.strip(chars = {'"'})

    case key
    of "ver":
      result.ver = val
    of "url":
      result.url = val
    of "sum":
      result.sum = val
    of "bld":
      result.bld = val
    of "run":
      result.run = val
    of "opt":
      result.opt = val
    else:
      abort(&"""{"name":8}{&"\{key\} not found":48}""")

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
    let package = parseInfo(name)

    printContent(idx, name, package.ver, "fetch")

    # Skip virtual packages
    if package.url == "nil":
      continue

    let
      src = srcCache / name
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
        discard downloadFile(package.url, src)

      if not verifyFile(archive, package.sum):
        abort(&"""{"sum":8}{name:24}{package.ver:24}""")

      createDir(tmp)
      discard extractTar(archive, tmp)

proc resolveDeps(
    name: string, packages: var seq[string], deps: var Table[string, seq[string]]
) =
  if name in packages:
    return

  let
    package = parseInfo(name)
    dep = package.bld

  deps[name] =
    if dep == "nil":
      @[]
    else:
      dep.split()

  for name in deps[name]:
    resolveDeps(name, packages, deps)

  packages &= name

proc sortPackages(packages: openArray[string]): seq[string] =
  var
    deps: Table[string, seq[string]]
    sorted: seq[string]

  for name in packages.deduplicate():
    resolveDeps(name, sorted, deps)

  sorted

proc installPackage(name: string, fs = "/", pkgCache = pkgCache) =
  let package = parseInfo(name)

  discard extractTar(
    pkgCache / name / name &
      (if package.url == "nil": ""
      else: '-' & package.ver & ".tar.zst"),
    fs,
  )

proc buildPackages*(packages: openArray[string], bootstrap = false, stage = native) =
  let queue =
    if bootstrap:
      packages.toSeq()
    else:
      sortPackages(packages)

  fetchPackages(queue)

  echo ""

  printHeader()

  for idx, name in queue:
    let
      package = parseInfo(name)
      archive =
        if package.url == "nil":
          pkgCache / name / name & ".tar.zst"
        else:
          pkgCache / name / name & '-' & package.ver & ".tar.zst"

    printContent(idx, name, package.ver, "build")

    if stage == native:
      # Skip package if archive exists
      if fileExists(archive):
        continue

      putEnv("dir", pkgCache / name / "dir")
      createDir(getEnv("dir"))

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
        ("REPO", coreRepo),
        ("TMPD", radTmp),
        ("YACC", "byacc"),
      ]

      for (i, j) in env:
        putEnv(i, j)

    if dirExists(radTmp):
      setCurrentDir(radTmp)
    if dirExists(radTmp / name & package.ver):
      setCurrentDir(radTmp / name & package.ver)

    let env = [
      ("ARCH", "x86-64"),
      ("BUILD", execCmdEx(coreRepo / "slibtool/files/config.guess").output.strip()),
      ("CTARGET", "x86_64-glaucus-linux-musl"),
      ("PRETTY_NAME", "glaucus s6 x86-64-v3 " & now().format("YYYYMMdd")),
      ("TARGET", "x86_64-pc-linux-musl"),
    ]

    for (i, j) in env:
      putEnv(i, j)

    let
      cflags =
        if "no-lto" notin package.opt:
          "-pipe -O2 -fgraphite-identity -floop-nest-optimize -flto=auto -flto-compression-level=3 -fuse-linker-plugin -fstack-protector-strong -fstack-clash-protection -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -march=x86-64-v3 -mfpmath=sse -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2"
        else:
          "-pipe -O2 -fgraphite-identity -floop-nest-optimize -fstack-protector-strong -fstack-clash-protection -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -march=x86-64-v3 -mfpmath=sse -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2"
      envFlags = [
        ("CFLAGS", cflags),
        ("CXXFLAGS", cflags),
        (
          "LDFLAGS",
          if "no-lto" notin package.opt:
            "-Wl,-O1,-s,-z,noexecstack,-z,now,-z,pack-relative-relocs,-z,relro,-z,x86-64-v3,--as-needed,--gc-sections,--sort-common,--hash-style=gnu " &
              cflags
          else:
            "-Wl,-O1,-s,-z,noexecstack,-z,now,-z,pack-relative-relocs,-z,relro,-z,x86-64-v3,--as-needed,--gc-sections,--sort-common,--hash-style=gnu",
        ),
        ("MAKEFLAGS", if "no-parallel" notin package.opt: "-j 5 -O" else: "-j 1"),
      ]

    for (i, j) in envFlags:
      putEnv(i, j)

    let shell = execCmdEx(
      &"""sh -efu -c '
        name={name} ver={package.ver} . {coreRepo / name / (if stage == native: "build" else: "build" & '-' & $stage)}

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
        genContents(dst, pkgCache / name / "contents")
        removeDir(dst)

        copyFileWithPermissions(coreRepo / name / "info", pkgCache / name / "info")

      if "bootstrap" in package.opt:
        installPackage(name)

proc showInfo*(packages: openArray[string]) =
  for name in packages.deduplicate():
    let package = parseInfo(name)

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
  showInfo(walkDir(pkgCache, true, skipSpecial = true).toSeq().unzip()[1].sorted())

proc listContents*(packages: openArray[string]) =
  for package in packages.deduplicate():
    for line in lines(pkgCache / package / "contents"):
      echo &"/{line}"

proc searchPackages*(pattern: openArray[string]) =
  var packages: seq[string]

  for package in walkDir(coreRepo, true, skipSpecial = true):
    for name in pattern:
      if name.toLowerAscii() in package[1]:
        packages &= package[1]

  if packages.len() == 0:
    exit(status = QuitFailure)

  showInfo(packages.sorted())
