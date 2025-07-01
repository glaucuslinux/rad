# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import
  std/[algorithm, os, osproc, sequtils, strformat, strutils, tables, terminal, times],
  constants,
  flags,
  tools,
  toml_serialization

type Package = object
  nom, ver, url, sum, bld, run*, opt = "nil"

proc `$`(self: Package): string =
  self.nom

proc cleanPackages*() =
  for i in [pathLog, pathTmp]:
    removeDir(i)
    createDir(i)

proc parsePackage*(nom: string): Package =
  let path = pathCoreRepo / nom

  if not dirExists(path):
    abort(&"""{"nom":8}{&"\{nom\} not found":48}""")

  Toml.loadFile(path / "info", Package)

proc printContent(idx: int, nom, ver, cmd: string) =
  echo &"""{idx + 1:<8}{nom:24}{ver:24}{cmd:8}{now().format("hh:mm tt")}"""

proc printHeader() =
  echo &"""
{'~'.repeat(72)}
{"idx":8}{"nom":24}{"ver":24}{"cmd":8}fin
{'~'.repeat(72)}"""

proc fetchPackages(packages: openArray[string]) =
  printHeader()

  for idx, nom in packages:
    let package = parsePackage(nom)

    printContent(idx, $package, package.ver, "fetch")

    # Skip virtual packages
    if package.url == "nil":
      continue

    let
      src = getEnv("SRCD") / $package
      tmp = getEnv("TMPD") / $package

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
        abort(&"""{"sum":8}{package:24}{package.ver:24}""")

proc resolveDeps(
    nom: string,
    cluster: var seq[string],
    deps: var Table[string, seq[string]],
    run = true,
) =
  if nom in cluster:
    return

  let
    package = parsePackage(nom)
    dep = if run: package.run else: package.bld

  deps[$package] =
    if dep == "nil":
      @[]
    else:
      dep.split()

  for nom in deps[$package]:
    resolveDeps(nom, cluster, deps, run)

  cluster &= nom

proc sortPackages(packages: openArray[string], run = true): seq[string] =
  var
    cluster: seq[string]
    deps: Table[string, seq[string]]

  for nom in packages.deduplicate():
    resolveDeps(nom, cluster, deps, run)

  cluster

proc installPackage(
    nom: string,
    fs = "/",
    pkgCache = pathPkgCache,
    pkgLib = pathLocalLib,
    implicit = false,
) =
  let
    package = parsePackage(nom)
    skel = parsePackage("skel").run

  discard extractTar(
    pkgCache / $package / $package &
      (if package.url == "nil": ""
      else: '-' & package.ver & ".tar.zst"),
    fs,
  )

  createDir(pkgLib / $package)
  copyFileWithPermissions(
    pkgCache / $package / "contents", pkgLib / $package / "contents"
  )
  copyFileWithPermissions(pkgCache / $package / "info", pkgLib / $package / "info")

  if implicit and $package notin skel:
    writeFile(pkgLib / $package / "implicit", "")

  if package.run.len() > 0:
    for dep in package.run.split():
      if dirExists(pkgLib / dep):
        createDir(pkgLib / dep / "run")
        writeFile(pkgLib / dep / "run" / nom, "")

proc buildPackages*(
    packages: openArray[string],
    fs = "/",
    pkgCache = pathPkgCache,
    pkgLib = pathLocalLib,
    resolve = true,
    stage = $native,
    implicit = false,
) =
  let cluster = sortPackages(packages, false)

  fetchPackages(cluster)

  echo ""

  printHeader()

  for idx, nom in (if resolve: cluster else: packages.toSeq()):
    let
      package = parsePackage(nom)
      archive =
        $pkgCache / $package / $package &
        (if package.url == "nil": ""
        else: '-' & package.ver & ".tar.zst")
      tmp = getEnv("TMPD") / $package

    printContent(idx, $package, package.ver, "build")

    if stage == $native:
      # Skip installed packages
      if dirExists(pkgLib / $package):
        continue

      # Skip package if archive exists
      if fileExists(archive):
        # Install build-time dependency (if not installed)
        if implicit:
          installPackage($package, implicit = true)
        continue

      putEnv("DSTD", pkgCache / $package / "dst")
      createDir(getEnv("DSTD"))

    if stage != $toolchain:
      setEnvFlags()

      if "no-lto" in $package.opt:
        setEnvFlagsNoLTO()

    if "no-parallel" in $package.opt:
      setEnvFlagsNoParallel()
    else:
      putEnv("MAKEFLAGS", "parallel")

    if dirExists(tmp):
      setCurrentDir(tmp)
    if dirExists(tmp / &"{package}-{package.ver}"):
      setCurrentDir(tmp / &"{package}-{package.ver}")

    let shell = execCmdEx(
      &"""sh -efu -c '
        nom={package} ver={package.ver} . {pathCoreRepo / $package / (if stage == $native: "build" else: "build" & '-' & stage)}

        for i in prepare configure build; do
          if command -v $i {shellRedirect}; then
            $i
          fi
        done

        package
      '"""
    )

    writeFile(
      getEnv("LOGD") / $package & (if stage == $native: "" else: stage),
      shell.output.strip(),
    )

    if shell.exitCode != QuitSuccess:
      abort(&"{shell.exitCode:<8}{package:24}{package.ver:24}")

    if stage == $native:
      let
        dst = getEnv("DSTD")
        status = createTarZst(archive, dst)

      # Purge
      # if "empty" notin $package.opt:

      if status == QuitSuccess:
        genContents(dst, $pkgCache / $package / "contents")
        removeDir(dst)

        copyFileWithPermissions(
          pathCoreRepo / $package / "info", pkgCache / $package / "info"
        )

      if "bootstrap" in $package.opt or implicit:
        installPackage($package, implicit = true)

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

  for idx, nom in cluster:
    let
      package = parsePackage(nom)
      archive =
        pkgCache / $package / $package &
        (if package.url == "nil": ""
        else: '-' & package.ver & ".tar.zst")

    if not fileExists(archive):
      notBuilt &= $package

  if notBuilt.len() > 0:
    buildPackages(notBuilt, implicit = true)

    echo ""

  printHeader()

  for idx, nom in cluster:
    let package = parsePackage(nom)

    printContent(idx, $package, package.ver, "install")

    # Skip installed packages
    if dirExists(pkgLib / $package):
      if $package notin packages and $package notin skel:
        writeFile(pkgLib / $package / "implicit", "")

      for nom in packages:
        removeFile(pkgLib / nom / "implicit")

      continue

    discard extractTar(
      pkgCache / $package / $package &
        (if package.url == "nil": ""
        else: '-' & package.ver & ".tar.zst"),
      fs,
    )

    createDir(pkgLib / $package)
    copyFileWithPermissions(
      pkgCache / $package / "contents", pkgLib / $package / "contents"
    )
    copyFileWithPermissions(pkgCache / $package / "info", pkgLib / $package / "info")

    if package.run.len() > 0:
      for dep in package.run.split():
        if dirExists(pkgLib / dep):
          createDir(pkgLib / dep / "run")
          writeFile(pkgLib / dep / "run" / nom, "")

proc showInfo*(packages: openArray[string]) =
  for nom in packages.deduplicate():
    let package = parsePackage(nom)

    echo &"""
nom  :: {package}
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
  for nom in packages.deduplicate():
    let package = parsePackage(nom)

    for line in lines(pathLocalLib / $package / "contents"):
      echo &"/{line}"

proc listOrphans*(pkgLib = pathLocalLib) =
  let
    installed = walkDir(pathLocalLib, true, skipSpecial = true).toSeq().unzip()[1]
    skel = parsePackage("skel").run

  for nom in installed:
    if nom notin skel and fileExists(pkgLib / $nom / "implicit"):
      if not dirExists(pkgLib / $nom / "run"):
        styledEcho fgYellow,
          styleBright, &"""{$QuitFailure:8}{&"\{nom\} is an orphan":48}"""

proc removePackages*(packages: openArray[string], pkgLib = pathLocalLib) =
  let
    installed = walkDir(pathLocalLib, true, skipSpecial = true).toSeq().unzip()[1]
    skel = parsePackage("skel").run
  var shouldAbort: bool

  for nom in packages:
    if nom notin installed:
      abort(&"""{$QuitFailure:8}{&"\{nom\} not installed":48}""")
    if nom in skel:
      abort(&"""{$QuitFailure:8}{&"\{nom\} is a skel package":48}""")
    if dirExists(pkgLib / $nom / "run"):
      let runDeps = walkDir(pkgLib / $nom / "run", true, skipSpecial = true)
      .toSeq()
      .unzip()[1].sorted()
      if runDeps.len() > 0:
        for dep in runDeps:
          if dep notin packages:
            styledEcho fgYellow,
              styleBright, &"""{$QuitFailure:8}{&"\{dep\} depends on \{nom\}":48}"""
            shouldAbort = true

    if shouldAbort:
      abort(&"""{$QuitFailure:8}{&"\{nom\} is a dependency":48}""")

  printHeader()

  for idx, nom in packages:
    let package = parsePackage(nom)

    printContent(idx, $package, package.ver, "remove")

    for line in lines(pathLocalLib / $package / "contents"):
      if not line.endsWith("/"):
        removeFile(&"/{line}")

    for line in lines(pathLocalLib / $package / "contents"):
      if line.endsWith("/"):
        let path = &"/{line}"

        if path.isEmpty():
          removeDir(path)

    removeDir(pathLocalLib / $package)

    for installedPackage in walkDir(pkgLib, true, skipSpecial = true).toSeq().unzip()[1].sorted():
      removeFile(pkgLib / $installedPackage / "run" / $package)
      if isEmpty(pkgLib / $installedPackage / "run"):
        removeDir(pkgLib / $installedPackage / "run")

proc searchPackages*(pattern: openArray[string]) =
  var packages: seq[string]

  for package in walkDir(pathCoreRepo, true, skipSpecial = true):
    for nom in pattern:
      if nom.toLowerAscii() in package[1]:
        packages &= package[1]

  if packages.len() == 0:
    exit(status = QuitFailure)

  showInfo(packages.sorted())
