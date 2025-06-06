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

type Ceras = object
  nom, ver, url, sum, bld, run, opt = $Nil

proc `$`(self: Ceras): string =
  self.nom

proc cleanCerata*() =
  removeDir($radLog)
  removeDir($radTmp)

  createDir($radLog)
  createDir($radTmp)

proc distcleanCerata*() =
  cleanCerata()

  removeDir($radPkgCache)
  removeDir($radSrcCache)

  createDir($radPkgCache)
  createDir($radSrcCache)

proc parseCeras(nom: string): Ceras =
  let path = $radClustersCerataLib / nom

  if not dirExists(path):
    abort(&"""{"nom":8}{&"\{nom\} not found":48}""")

  Toml.loadFile(path / $info, Ceras)

proc printContent(idx: int, nom, ver, cmd: string) =
  echo &"""{idx + 1:<8}{nom:24}{ver:24}{cmd:8}{now().format("hh:mm tt")}"""

proc printHeader() =
  echo &"""
{'~'.repeat(72)}
{"idx":8}{"nom":24}{"ver":24}{"cmd":8}fin
{'~'.repeat(72)}"""

proc fetchCerata(cerata: openArray[string]) =
  printHeader()

  for idx, nom in cerata:
    let ceras = parseCeras(nom)

    printContent(idx, $ceras, ceras.ver, $fetch)

    # Skip virtual cerata
    if ceras.url == $Nil:
      continue

    let
      src = getEnv($SRCD) / $ceras
      tmp = getEnv($TMPD) / $ceras

    if ceras.sum == $Nil:
      if not dirExists(src):
        discard gitCloneRepo(ceras.url, src)

        if ceras.ver != "latest":
          discard gitCheckoutRepo(src, ceras.ver)

      copyDirWithPermissions(src, tmp)
    else:
      let archive = src / lastPathPart(ceras.url)

      if not verifyFile(archive, ceras.sum):
        removeDir(src)
        createDir(src)
        discard downloadFile(src, ceras.url)

      if verifyFile(archive, ceras.sum):
        createDir(tmp)
        discard extractTar(archive, tmp)
      else:
        abort(&"""{"sum":8}{ceras:24}{ceras.ver:24}""")

proc resolveDeps(
    nom: string,
    cluster: var seq[string],
    deps: var Table[string, seq[string]],
    run = true,
) =
  if nom in cluster:
    return

  let
    ceras = parseCeras(nom)
    dep = if run: ceras.run else: ceras.bld

  deps[$ceras] =
    if dep == $Nil:
      @[]
    else:
      dep.split()

  for nom in deps[$ceras]:
    resolveDeps(nom, cluster, deps, run)

  cluster &= nom

proc sortCerata(cerata: openArray[string], run = true): seq[string] =
  var
    cluster: seq[string]
    deps: Table[string, seq[string]]

  for nom in cerata.deduplicate():
    resolveDeps(nom, cluster, deps, run)

  cluster

proc installCeras(
    nom: string,
    fs = $root,
    pkgCache = $radPkgCache,
    pkgLib = $radPkgLib,
    implicit = false,
) =
  let
    ceras = parseCeras(nom)
    skel = parseCeras($skel).run

  discard extractTar(
    pkgCache / $ceras /
      &"""{ceras}{(if ceras.url == $Nil: "" else: &"-\{ceras.ver\}")}{tarZst}""",
    fs,
  )

  createDir(pkgLib / $ceras)
  copyFileWithPermissions(pkgCache / $ceras / $contents, pkgLib / $ceras / $contents)
  copyFileWithPermissions(pkgCache / $ceras / $info, pkgLib / $ceras / $info)

  if implicit and $ceras notin skel:
    writeFile(pkgLib / $ceras / "implicit", "")

  if ceras.run.len() > 0:
    for dep in ceras.run.split():
      if dirExists(pkgLib / dep):
        createDir(pkgLib / dep / "run")
        writeFile(pkgLib / dep / "run" / nom, "")

proc buildCerata*(
    cerata: openArray[string],
    fs = $root,
    pkgCache = $radPkgCache,
    pkgLib = $radPkgLib,
    resolve = true,
    stage = $native,
    implicit = false,
) =
  let cluster = sortCerata(cerata, false)

  fetchCerata(cluster)

  echo ""

  printHeader()

  for idx, nom in (if resolve: cluster else: cerata.toSeq()):
    let
      ceras = parseCeras(nom)
      archive =
        $pkgCache / $ceras /
        &"""{ceras}{(if ceras.url == $Nil: "" else: &"-\{ceras.ver\}")}{tarZst}"""
      tmp = getEnv($TMPD) / $ceras

    printContent(idx, $ceras, ceras.ver, $build)

    if stage == $native:
      # Skip installed packages
      if dirExists(pkgLib / $ceras):
        continue

      # Skip package if archive exists
      if fileExists(archive):
        # Install build-time dependency (if not installed)
        if implicit:
          installCeras($ceras, implicit = true)
        continue

      putEnv($SACD, pkgCache / $ceras / $sac)
      createDir(getEnv($SACD))

    if stage != $toolchain:
      setEnvFlags()

      if $noLTO in $ceras.opt:
        setEnvFlagsNoLTO()

    if $radOpt.noParallel in $ceras.opt:
      setEnvFlagsNoParallel()
    else:
      putEnv($MAKEFLAGS, $parallel)

    if dirExists(tmp):
      setCurrentDir(tmp)
    if dirExists(tmp / &"{ceras}-{ceras.ver}"):
      setCurrentDir(tmp / &"{ceras}-{ceras.ver}")

    let shell = execCmdEx(
      &"""{sh} -efu -c '
        nom={ceras} ver={ceras.ver} . {$radClustersCerataLib / $ceras / (if stage == $native: $build else: $build & '-' & stage)}

        for i in prepare configure build; do
          if command -v $i {shellRedirect}; then
            $i
          fi
        done

        package
      '"""
    )

    writeFile(
      getEnv($LOGD) / &"""{ceras}{(if stage == $native: "" else: &".\{stage\}")}""",
      shell.output.strip(),
    )

    if shell.exitCode != QuitSuccess:
      abort(&"{shell.exitCode:<8}{ceras:24}{ceras.ver:24}")

    if stage == $native:
      let
        sac = getEnv($SACD)
        status = createTarZst(archive, sac)

      # Purge
      # if empty notin $ceras.opt:

      if status == QuitSuccess:
        genContents(sac, $pkgCache / $ceras / $contents)
        removeDir(sac)

        copyFileWithPermissions(
          $radClustersCerataLib / $ceras / $info, pkgCache / $ceras / $info
        )

      if $bootstrap in $ceras.opt or implicit:
        installCeras($ceras, implicit = true)

proc installCerata*(
    cerata: openArray[string], fs = $root, pkgCache = $radPkgCache, pkgLib = $radPkgLib
) =
  let
    cluster = sortCerata(cerata)
    skel = parseCeras($skel).run
  var notBuilt: seq[string]

  for idx, nom in cluster:
    let
      ceras = parseCeras(nom)
      archive =
        pkgCache / $ceras /
        &"""{ceras}{(if ceras.url == $Nil: "" else: &"-\{ceras.ver\}")}{tarZst}"""

    if not fileExists(archive):
      notBuilt &= $ceras

  if notBuilt.len() > 0:
    buildCerata(notBuilt, implicit = true)

    echo ""

  printHeader()

  for idx, nom in cluster:
    let ceras = parseCeras(nom)

    printContent(idx, $ceras, ceras.ver, $install)

    # Skip installed packages
    if dirExists(pkgLib / $ceras):
      if $ceras notin cerata and $ceras notin skel:
        writeFile(pkgLib / $ceras / "implicit", "")

      for nom in cerata:
        removeFile(pkgLib / nom / "implicit")

      continue

    discard extractTar(
      pkgCache / $ceras /
        &"""{ceras}{(if ceras.url == $Nil: "" else: &"-\{ceras.ver\}")}{tarZst}""",
      fs,
    )

    createDir(pkgLib / $ceras)
    copyFileWithPermissions(pkgCache / $ceras / $contents, pkgLib / $ceras / $contents)
    copyFileWithPermissions(pkgCache / $ceras / $info, pkgLib / $ceras / $info)

    if ceras.run.len() > 0:
      for dep in ceras.run.split():
        if dirExists(pkgLib / dep):
          createDir(pkgLib / dep / "run")
          writeFile(pkgLib / dep / "run" / nom, "")

proc showInfo*(cerata: openArray[string]) =
  for nom in cerata.deduplicate():
    let ceras = parseCeras(nom)

    echo &"""
nom  :: {ceras}
ver  :: {ceras.ver}
url  :: {ceras.url}
sum  :: {ceras.sum}
bld  :: {ceras.bld}
run  :: {ceras.run}
opt  :: {ceras.opt}
"""

proc listCerata*() =
  showInfo(walkDir($radPkgLib, true, skipSpecial = true).toSeq().unzip()[1].sorted())

proc listContents*(cerata: openArray[string]) =
  for nom in cerata.deduplicate():
    let ceras = parseCeras(nom)

    for line in lines($radPkgLib / $ceras / $contents):
      echo &"/{line}"

proc listOrphans*(pkgLib = $radPkgLib) =
  let
    installed = walkDir($radPkgLib, true, skipSpecial = true).toSeq().unzip()[1]
    skel = parseCeras($skel).run

  for nom in installed:
    if nom notin skel and fileExists(pkgLib / $nom / "implicit"):
      if not dirExists(pkgLib / $nom / "run"):
        styledEcho fgYellow,
          styleBright, &"""{$QuitFailure:8}{&"\{nom\} is an orphan":48}"""

proc removeCerata*(cerata: openArray[string], pkgLib = $radPkgLib) =
  let
    installed = walkDir($radPkgLib, true, skipSpecial = true).toSeq().unzip()[1]
    skel = parseCeras($skel).run
  var shouldAbort: bool

  for nom in cerata:
    if nom notin installed:
      abort(&"""{$QuitFailure:8}{&"\{nom\} not installed":48}""")
    if nom in skel:
      abort(&"""{$QuitFailure:8}{&"\{nom\} is a skel ceras":48}""")
    if dirExists(pkgLib / $nom / "run"):
      let runDeps = walkDir(pkgLib / $nom / "run", true, skipSpecial = true)
      .toSeq()
      .unzip()[1].sorted()
      if runDeps.len() > 0:
        for dep in runDeps:
          if dep notin cerata:
            styledEcho fgYellow,
              styleBright, &"""{$QuitFailure:8}{&"\{dep\} depends on \{nom\}":48}"""
            shouldAbort = true

    if shouldAbort:
      abort(&"""{$QuitFailure:8}{&"\{nom\} is a dependency":48}""")

  printHeader()

  for idx, nom in cerata:
    let ceras = parseCeras(nom)

    printContent(idx, $ceras, ceras.ver, $remove)

    for line in lines($radPkgLib / $ceras / $contents):
      if not line.endsWith($root):
        removeFile(&"/{line}")

    for line in lines($radPkgLib / $ceras / $contents):
      if line.endsWith($root):
        let path = &"/{line}"

        if path.isEmpty():
          removeDir(path)

    removeDir($radPkgLib / $ceras)

    for installedPackage in walkDir(pkgLib, true, skipSpecial = true).toSeq().unzip()[1].sorted():
      removeFile(pkgLib / $installedPackage / "run" / $ceras)
      if isEmpty(pkgLib / $installedPackage / "run"):
        removeDir(pkgLib / $installedPackage / "run")

proc searchCerata*(pattern: openArray[string]) =
  var cerata: seq[string]

  for ceras in walkDir($radClustersCerataLib, true, skipSpecial = true):
    for nom in pattern:
      if nom.toLowerAscii() in ceras[1]:
        cerata &= ceras[1]

  if cerata.len() == 0:
    exit(status = QuitFailure)

  showInfo(cerata.sorted())
