# Copyright (c) 2018-2025, Firas Khalil Khana
# Distributed under the terms of the ISC License

import
  std/[algorithm, os, osproc, sequtils, strformat, strutils, tables, times],
  constants,
  flags,
  tools,
  toml_serialization

type Ceras = object
  nom, ver, url, sum, bld, run, nop = $Nil

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
    abort(&"""{"nom":8}{&"\{nom\} not found":48}""", 127)

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
        discard gitCheckoutRepo(src, ceras.ver)

      copyDirWithPermissions(src, tmp)
    else:
      let archive = src / lastPathPart(ceras.url)

      if not verifyFile(archive, ceras.sum):
        removeDir(src)
        createDir(src)
        discard downloadFile(ceras.url, archive)

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
    ceras: string, fs = $root, pkgCache = $radPkgCache, pkgLib = $radPkgLib
) =
  let ceras = parseCeras(ceras)

  discard extractTar(
    pkgCache / $ceras /
      &"""{ceras}{(if ceras.url == $Nil: "" else: &"-\{ceras.ver\}")}{tarZst}""",
    fs,
  )

  createDir(pkgLib / $ceras)
  copyFileWithPermissions(pkgCache / $ceras / $contents, pkgLib / $ceras / $contents)
  copyFileWithPermissions(pkgCache / $ceras / $info, pkgLib / $ceras / $info)

proc buildCerata*(
    cerata: openArray[string],
    fs = $root,
    pkgCache = $radPkgCache,
    pkgLib = $radPkgLib,
    resolve = true,
    stage = $native,
) =
  let cluster = sortCerata(cerata, false)

  fetchCerata(cluster)

  echo ""

  printHeader()

  for idx, nom in (if resolve: cluster else: cerata.toSeq()):
    let
      ceras = parseCeras(nom)
      archive =
        $radPkgCache / $ceras /
        &"""{ceras}{(if ceras.url == $Nil: "" else: &"-\{ceras.ver\}")}{tarZst}"""
      tmp = getEnv($TMPD) / $ceras

    printContent(idx, $ceras, ceras.ver, $build)

    if stage == $native:
      # Skip build-time dependency if installed
      if $ceras notin cerata and dirExists(pkgLib / $ceras):
        continue

      # Skip package if archive exists
      if fileExists(archive):
        # Install build-time dependency (if not installed)
        if $ceras notin cerata:
          installCeras($ceras)
        continue

      putEnv($SACD, pkgCache / $ceras / $sac)
      createDir(getEnv($SACD))

    if stage != $toolchain:
      setEnvFlags()

      if $radNop.lto in $ceras.nop:
        setEnvFlagsNopLto()

    if $radNop.parallel in $ceras.nop:
      setEnvFlagsNopParallel()
    else:
      putEnv($MAKEFLAGS, $radFlags.make)

    if dirExists(tmp):
      setCurrentDir(tmp)
    if dirExists(tmp / &"{ceras}-{ceras.ver}"):
      setCurrentDir(tmp / &"{ceras}-{ceras.ver}")

    # Only use `nom` and `ver` from `ceras`
    #
    # Call phases sequentially to preserve the current working directory
    let shell = execCmdEx(
      &"""{sh} -c 'nom={ceras} ver={ceras.ver} . {$radClustersCerataLib / $ceras / (if stage == $native: $build else: $build & '-' & stage)} && for i in prepare configure build; do if command -v $i {shellRedirect}; then $i; fi done && package'"""
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
      # if $radNop.empty notin $ceras.nop:

      if status == QuitSuccess:
        genContents(sac, $radPkgCache / $ceras / $contents)
        removeDir(sac)

        copyFileWithPermissions(
          $radClustersCerataLib / $ceras / $info, pkgCache / $ceras / $info
        )

      if $bootstrap in $ceras.nop or $ceras notin cerata:
        installCeras($ceras)

proc installCerata*(
    cerata: openArray[string], fs = $root, pkgCache = $radPkgCache, pkgLib = $radPkgLib
) =
  let cluster = sortCerata(cerata)

  printHeader()

  for idx, nom in cluster:
    let ceras = parseCeras(nom)

    printContent(idx, $ceras, ceras.ver, $install)

    discard extractTar(
      pkgCache / $ceras /
        &"""{ceras}{(if ceras.url == $Nil: "" else: &"-\{ceras.ver\}")}{tarZst}""",
      fs,
    )

    createDir(pkgLib / $ceras)
    copyFileWithPermissions(pkgCache / $ceras / $contents, pkgLib / $ceras / $contents)
    copyFileWithPermissions(pkgCache / $ceras / $info, pkgLib / $ceras / $info)

    if nom notin cerata:
      createDir(pkgLib / $ceras / "run")
      # writeFile(lib / $ceras / "run" / )

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
nop  :: {ceras.nop}
"""

proc listCerata*() =
  showInfo(walkDir($radPkgLib, true, skipSpecial = true).toSeq().unzip()[1].sorted())

proc removeCerata*(cerata: openArray[string]) =
  let
    installed = walkDir($radPkgLib, true, skipSpecial = true).toSeq().unzip()[1]
    skel = parseCeras($skel).run

  for nom in cerata:
    if nom notin installed:
      abort(&"""{$QuitFailure:8}{&"\{nom\} not installed":48}""")
    if nom in skel:
      abort(&"""{$QuitFailure:8}{&"\{nom\} is a skel ceras":48}""")

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

proc searchCerata*(pattern: openArray[string]) =
  var cerata: seq[string]

  for ceras in walkDir($radClustersCerataLib, true, skipSpecial = true):
    for nom in pattern:
      if nom.toLowerAscii() in ceras[1]:
        cerata &= ceras[1]

  if cerata.len() == 0:
    exit(status = QuitFailure)

  showInfo(cerata.sorted())
