# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import
  std/[algorithm, os, osproc, sequtils, strformat, strutils, tables, times],
  constants,
  tools,
  toml_serialization

type Package = object
  ver, url, sum, bld, run*, opt = "nil"

proc cleanPackages*() =
  removeDir(pathTmp)
  createDir(pathTmp)

proc parsePackage*(name: string): Package =
  let path = pathCoreRepo / name

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

proc fetchPackages(packages: openArray[string]) =
  printHeader()

  for idx, name in packages:
    let package = parsePackage(name)

    printContent(idx, name, package.ver, "fetch")

    # Skip virtual packages
    if package.url == "nil":
      continue

    let
      src = pathSrcCache / name
      tmp = getEnv("TMPD") / name

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

proc installPackage(
    name: string,
    fs = "/",
    pkgCache = pathPkgCache,
    pkgLib = pathLocalLib,
    implicit = false,
) =
  let
    package = parsePackage(name)
    skel = parsePackage("skel").run

  discard extractTar(
    pkgCache / name / name &
      (if package.url == "nil": ""
      else: '-' & package.ver & ".tar.zst"),
    fs,
  )

  createDir(pkgLib / name)
  copyFileWithPermissions(pkgCache / name / "contents", pkgLib / name / "contents")
  copyFileWithPermissions(pkgCache / name / "info", pkgLib / name / "info")

  if implicit and name notin skel:
    writeFile(pkgLib / name / "implicit", "")

  if package.run.len() > 0:
    for dep in package.run.split():
      if dirExists(pkgLib / dep):
        createDir(pkgLib / dep / "run")
        writeFile(pkgLib / dep / "run" / name, "")

proc setEnvArch*() =
  let env = [
    ("ARCH", "x86-64"),
    ("BUILD", execCmdEx(pathCoreRepo / "slibtool/files/config.guess").output.strip()),
    ("CTARGET", "x86_64-glaucus-linux-musl"),
    ("PRETTY_NAME", "glaucus s6 x86-64-v3 " & now().format("YYYYMMdd")),
    ("TARGET", "x86_64-pc-linux-musl"),
  ]

  for (i, j) in env:
    putEnv(i, j)

proc setEnvFlags*(lto = true, parallel = true) =
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
    pkgCache = pathPkgCache,
    pkgLib = pathLocalLib,
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
      # Skip installed packages
      if dirExists(pkgLib / name):
        continue

      # Skip package if archive exists
      if fileExists(archive):
        # Install build-time dependency (if not installed)
        if implicit:
          installPackage(name, implicit = true)
        continue

      putEnv("DSTD", pkgCache / name / "dst")
      createDir(getEnv("DSTD"))

    if stage != toolchain:
      setEnvFlags()

      if "no-lto" in package.opt:
        setEnvFlags(lto = false)

      if "no-parallel" in package.opt:
        setEnvFlags(parallel = false)

    if dirExists(tmp):
      setCurrentDir(tmp)
    if dirExists(tmp / name & package.ver):
      setCurrentDir(tmp / name & package.ver)

    let shell = execCmdEx(
      &"""sh -efu -c '
        name={name} ver={package.ver} . {pathCoreRepo / name / (if stage == native: "build" else: "build" & '-' & $stage)}

        for i in prepare configure build; do
          if command -v $i {shellRedirect}; then
            $i
          fi
        done

        package
      '"""
    )

    writeFile(
      pathLog / name & (if stage == native: "" else: '.' & $stage), shell.output.strip()
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

        copyFileWithPermissions(pathCoreRepo / name / "info", pkgCache / name / "info")

      if "bootstrap" in package.opt or implicit:
        installPackage(name, implicit = true)

proc installPackages*(
    packages: openArray[string],
    fs = "/",
    pkgCache = pathPkgCache,
    pkgLib = pathLocalLib,
) =
  let
    cluster = sortPackages(packages)
    skel = parsePackage("skel").run
  var notBuilt: seq[string]

  for idx, name in cluster:
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

  for idx, name in cluster:
    let package = parsePackage(name)

    printContent(idx, name, package.ver, "install")

    # Skip installed packages
    if dirExists(pkgLib / name):
      if name notin packages and name notin skel:
        writeFile(pkgLib / name / "implicit", "")

      for name in packages:
        removeFile(pkgLib / name / "implicit")

      continue

    discard extractTar(
      pkgCache / name / name &
        (if package.url == "nil": ""
        else: '-' & package.ver & ".tar.zst"),
      fs,
    )

    createDir(pkgLib / name)
    copyFileWithPermissions(pkgCache / name / "contents", pkgLib / name / "contents")
    copyFileWithPermissions(pkgCache / name / "info", pkgLib / name / "info")

    if package.run.len() > 0:
      for dep in package.run.split():
        if dirExists(pkgLib / dep):
          createDir(pkgLib / dep / "run")
          writeFile(pkgLib / dep / "run" / name, "")

proc showInfo*(packages: openArray[string]) =
  for name in packages.deduplicate():
    let package = parsePackage(name)

    echo &"""
name  :: {name}
ver  :: {package.ver}
url  :: {package.url}
sum  :: {package.sum}
bld  :: {package.bld}
run  :: {package.run}
opt  :: {package.opt}
"""

proc listPackages*() =
  showInfo(walkDir(pathLocalLib, true, skipSpecial = true).toSeq().unzip()[1].sorted())

proc listContents*(packages: openArray[string]) =
  for name in packages.deduplicate():
    let package = parsePackage(name)

    for line in lines(pathLocalLib / name / "contents"):
      echo &"/{line}"

proc searchPackages*(pattern: openArray[string]) =
  var packages: seq[string]

  for package in walkDir(pathCoreRepo, true, skipSpecial = true):
    for name in pattern:
      if name.toLowerAscii() in package[1]:
        packages &= package[1]

  if packages.len() == 0:
    exit(status = QuitFailure)

  showInfo(packages.sorted())
