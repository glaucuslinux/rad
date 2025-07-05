# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import
  std/[algorithm, os, osproc, sequtils, strformat, strutils, tables, times],
  constants,
  flags,
  tools,
  toml_serialization

type Package = object
  nom, ver, url, sum, bld, run*, opt = "nil"

proc cleanPackages*() =
  removeDir(pathTmp)
  createDir(pathTmp)

proc parsePackage*(nom: string): Package =
  let path = pathCoreRepo / nom

  if not dirExists(path):
    abort(&"""{"nom":8}{&"\{nom\} not found":48}""")

  Toml.loadFile(path / "info", Package)

proc printContent(idx: int, nom, ver, cmd: string) =
  echo &"""{idx + 1:<8}{nom:24}{ver:24}{cmd:8}""" & now().format("hh:mm tt")

proc printHeader() =
  echo """
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
idx     nom                     ver                     cmd     fin     
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"""

proc fetchPackages(packages: openArray[string]) =
  printHeader()

  for idx, nom in packages:
    let package = parsePackage(nom)

    printContent(idx, package.nom, package.ver, "fetch")

    # Skip virtual packages
    if package.url == "nil":
      continue

    let
      src = getEnv("SRCD") / package.nom
      tmp = getEnv("TMPD") / package.nom

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

  deps[package.nom] =
    if dep == "nil":
      @[]
    else:
      dep.split()

  for nom in deps[package.nom]:
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
    pkgCache / package.nom / package.nom &
      (if package.url == "nil": ""
      else: '-' & package.ver & ".tar.zst"),
    fs,
  )

  createDir(pkgLib / package.nom)
  copyFileWithPermissions(
    pkgCache / package.nom / "contents", pkgLib / package.nom / "contents"
  )
  copyFileWithPermissions(
    pkgCache / package.nom / "info", pkgLib / package.nom / "info"
  )

  if implicit and package.nom notin skel:
    writeFile(pkgLib / package.nom / "implicit", "")

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
        $pkgCache / package.nom / package.nom &
        (if package.url == "nil": ""
        else: '-' & package.ver & ".tar.zst")
      tmp = getEnv("TMPD") / package.nom

    printContent(idx, package.nom, package.ver, "build")

    if stage == $native:
      # Skip installed packages
      if dirExists(pkgLib / package.nom):
        continue

      # Skip package if archive exists
      if fileExists(archive):
        # Install build-time dependency (if not installed)
        if implicit:
          installPackage(package.nom, implicit = true)
        continue

      putEnv("DSTD", pkgCache / package.nom / "dst")
      createDir(getEnv("DSTD"))

    if stage != $toolchain:
      setEnvFlags()

      if "no-lto" in package.opt:
        setEnvFlags(lto = false)

    if "no-parallel" in package.opt:
      setEnvFlags(parallel = false)

    if dirExists(tmp):
      setCurrentDir(tmp)
    if dirExists(tmp / &"{package}-{package.ver}"):
      setCurrentDir(tmp / &"{package}-{package.ver}")

    let shell = execCmdEx(
      &"""sh -efu -c '
        nom={package.nom} ver={package.ver} . {pathCoreRepo / package.nom / (if stage == $native: "build" else: "build" & '-' & stage)}

        for i in prepare configure build; do
          if command -v $i {shellRedirect}; then
            $i
          fi
        done

        package
      '"""
    )

    writeFile(
      getEnv("LOGD") / package.nom & (if stage == $native: "" else: stage),
      shell.output.strip(),
    )

    if shell.exitCode != QuitSuccess:
      abort(&"{shell.exitCode:<8}{package:24}{package.ver:24}")

    if stage == $native:
      let
        dst = getEnv("DSTD")
        status = createTarZst(archive, dst)

      # Purge
      # if "empty" notin package.opt:

      if status == QuitSuccess:
        genContents(dst, $pkgCache / package.nom / "contents")
        removeDir(dst)

        copyFileWithPermissions(
          pathCoreRepo / package.nom / "info", pkgCache / package.nom / "info"
        )

      if "bootstrap" in package.opt or implicit:
        installPackage(package.nom, implicit = true)

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
        pkgCache / package.nom / package.nom &
        (if package.url == "nil": ""
        else: '-' & package.ver & ".tar.zst")

    if not fileExists(archive):
      notBuilt &= package.nom

  if notBuilt.len() > 0:
    buildPackages(notBuilt, implicit = true)

    echo ""

  printHeader()

  for idx, nom in cluster:
    let package = parsePackage(nom)

    printContent(idx, package.nom, package.ver, "install")

    # Skip installed packages
    if dirExists(pkgLib / package.nom):
      if package.nom notin packages and package.nom notin skel:
        writeFile(pkgLib / package.nom / "implicit", "")

      for nom in packages:
        removeFile(pkgLib / nom / "implicit")

      continue

    discard extractTar(
      pkgCache / package.nom / package.nom &
        (if package.url == "nil": ""
        else: '-' & package.ver & ".tar.zst"),
      fs,
    )

    createDir(pkgLib / package.nom)
    copyFileWithPermissions(
      pkgCache / package.nom / "contents", pkgLib / package.nom / "contents"
    )
    copyFileWithPermissions(
      pkgCache / package.nom / "info", pkgLib / package.nom / "info"
    )

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

    for line in lines(pathLocalLib / package.nom / "contents"):
      echo &"/{line}"

proc searchPackages*(pattern: openArray[string]) =
  var packages: seq[string]

  for package in walkDir(pathCoreRepo, true, skipSpecial = true):
    for nom in pattern:
      if nom.toLowerAscii() in package[1]:
        packages &= package[1]

  if packages.len() == 0:
    exit(status = QuitFailure)

  showInfo(packages.sorted())
