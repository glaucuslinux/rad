# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/[algorithm, os, osproc, sequtils, strformat, strutils, tables, times]
import utils

const pkgCache = "/var/cache/rad/pkg"
const srcCache = "/var/cache/rad/src"
const coreRepo = "/var/lib/rad/repo/core"
const radLog = "/var/log/rad"
const radTmp = "/var/tmp/rad"

type Package = object
  ver, url, sum, bld, run*, opt, lic = "nil"

type Stages* = enum
  cross
  native
  toolchain

proc cleanCache*() =
  removeDir(radTmp)
  createDir(radTmp)

proc parseInfo*(nom: string): Package =
  let path = coreRepo / nom

  if not dirExists(path):
    abort(&"""{"nom":8}{&"\{nom\} not found":48}""")

  result = Package()

  for line in lines(path / "info"):
    if line.isEmptyOrWhitespace() or line.startsWith('#'):
      continue

    if line.contains(" =\"") or line.contains("= \"") or '=' notin line:
      abort(&"""{"nom":8}{"whitespace found":48}""")

    let pair = line.split('=', 1)
    let key = pair[0]
    var val = pair[1]

    if not (val.startsWith('"') and val.endsWith('"')):
      abort(&"""{"nom":8}{"quotes not found":48}""")

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
    of "lic":
      result.lic = val
    else:
      abort(&"""{"nom":8}{&"\{key\} not found":48}""")

proc printContent(idx: int, nom, ver, cmd: string) =
  echo &"""{idx + 1:<8}{nom:24}{ver:24}{cmd:8}""" & now().format("hh:mm tt")

proc printHeader() =
  echo """
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
idx     nom                     ver                     cmd     now     
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

  for idx, nom in packages:
    let package = parseInfo(nom)

    printContent(idx, nom, package.ver, "fetch")

    # Skip virtual packages
    if package.url == "nil":
      continue

    let src = srcCache / nom
    let tmp = radTmp / nom

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
        abort(&"""{"sum":8}{nom:24}{package.ver:24}""")

      createDir(tmp)
      discard extractTar(archive, tmp)

proc resolveDeps(
    nom: string, packages: var seq[string], deps: var Table[string, seq[string]]
) =
  if nom in packages:
    return

  let package = parseInfo(nom)
  let dep = package.bld

  deps[nom] =
    if dep == "nil":
      @[]
    else:
      dep.split()

  for nom in deps[nom]:
    resolveDeps(nom, packages, deps)

  packages &= nom

proc sortPackages(packages: openArray[string]): seq[string] =
  var deps: Table[string, seq[string]]
  var sorted: seq[string]

  for nom in packages.deduplicate():
    resolveDeps(nom, sorted, deps)

  sorted

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
  let queue =
    if bootstrap:
      packages.toSeq()
    else:
      sorted

  fetchPackages(sorted)

  echo ""

  printHeader()

  let cross = absolutePath("../../glaucus/cross")
  let toolchain = absolutePath("../../glaucus/toolchain")

  for idx, nom in queue:
    let package = parseInfo(nom)
    let archive =
      pkgCache / nom / nom &
      (if package.url == "nil": ".tar.zst"
      else: '-' & package.ver & ".tar.zst")

    printContent(idx, nom, package.ver, "build")

    if stage == native:
      # Skip package if archive exists
      if fileExists(archive):
        continue

      createDir(pkgCache / nom / "dir")

      const envExes = [
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

      for (i, j) in envExes:
        putEnv(i, j)

    let cflags =
      "-pipe -O2" & (
        if "no-lto" notin package.opt:
          " -flto=auto -flto-compression-level=3 -fuse-linker-plugin "
        else:
          " "
      ) &
      "-fstack-protector-strong -fstack-clash-protection -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -march=x86-64-v3 -malign-data=cacheline -mtls-dialect=gnu2"

    let env = [
      ("ARCH", "x86-64"),
      ("CFLAGS", cflags),
      ("CXXFLAGS", cflags),
      (
        "LDFLAGS",
        "-Wl,-O1,-s,-z,noexecstack,-z,now,-z,pack-relative-relocs,-z,relro,-z,x86-64-v3,--as-needed,--gc-sections,--sort-common,--hash-style=gnu" &
        (
          if "no-lto" notin package.opt:
            " " & cflags
          else:
            ""
        ),
      ),
      ("MAKEFLAGS", if "no-parallel" notin package.opt: "-j 5 -O" else: "-j 1"),
      ("PRETTY_NAME", "glaucus s6 x86-64-v3 " & now().format("YYYYMMdd")),
    ]

    for (i, j) in env:
      putEnv(i, j)

    let shellBootstrap =
      &"""
      build={execCmdEx(coreRepo / "slibtool/files/config.guess").output.strip()}
      host=x86_64-glaucus-linux-musl
      target=x86_64-pc-linux-musl

      cross={cross}
      toolchain={toolchain}"""

    if dirExists(radTmp / nom):
      setCurrentDir(radTmp / nom)
    if dirExists(radTmp / nom / nom & '-' & package.ver):
      setCurrentDir(radTmp / nom / nom & '-' & package.ver)

    let shell = execCmdEx(
      &"""sh -efu -c '
        {(if bootstrap: shellBootstrap else: ":")}

        core={coreRepo}
        tmp={radTmp}

        dir={pkgCache / nom / "dir"}

        nom={nom}
        ver={package.ver}

        . {coreRepo / nom / (if stage == native: "build" else: "build" & '-' & $stage)}

        for i in prepare configure build; do
          if command -v $i >/dev/null 2>&1; then
            $i
          fi
        done

        package'"""
    )

    writeFile(
      radLog / nom & (if stage == native: "" else: '.' & $stage), shell.output.strip()
    )

    if shell.exitCode != QuitSuccess:
      abort(&"{shell.exitCode:<8}{nom:24}{package.ver:24}")

    if stage == native:
      let dst = getEnv("DSTD")
      let status = createTarZst(archive, dst)

      # Purge
      # if "empty" notin package.opt:

      if status == QuitSuccess:
        genContents(dst, pkgCache / nom / "contents")
        removeDir(dst)

        copyFileWithPermissions(coreRepo / nom / "info", pkgCache / nom / "info")

      if "bootstrap" in package.opt:
        installPackage(nom)

proc showInfo*(packages: openArray[string]) =
  for nom in packages.deduplicate():
    let package = parseInfo(nom)

    echo &"""
nom  :: {nom}
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
    for nom in pattern:
      if nom.toLowerAscii() in package[1]:
        packages &= package[1]

  if packages.len() == 0:
    exit(status = QuitFailure)

  showInfo(packages.sorted())
