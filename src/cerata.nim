# Copyright (c) 2018-2025, Firas Khalil Khana
# Distributed under the terms of the ISC License

import
  std/[algorithm, os, osproc, sequtils, strformat, strutils, tables, terminal, times],
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

  Toml.loadFile(path / $ceras, Ceras)

proc printCerata*(cerata: openArray[string]) =
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

proc printContent(idx: int, nom, ver, cmd: string) =
  styledEcho fgMagenta,
    styleBright, &"""{idx + 1:<8}{nom:24}{ver:24}{cmd:8}{now().format("hh:mm tt")}"""

proc printFooter(idx: int, nom, ver, cmd: string) =
  cursorUp 1
  eraseLine()

  echo &"""{idx + 1:<8}{nom:24}{ver:24}{cmd:8}{now().format("hh:mm tt")}"""

proc printHeader() =
  echo &"""
{'~'.repeat(72)}
{"idx":8}{"nom":24}{"ver":24}{"cmd":8}fin
{'~'.repeat(72)}"""

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

  cluster.add(nom)

proc sortCerata(cerata: openArray[string], run = true): seq[string] =
  var
    cluster: seq[string]
    deps: Table[string, seq[string]]

  for nom in cerata.deduplicate():
    resolveDeps(nom, cluster, deps, run)

  cluster

proc prepareCerata(cerata: openArray[string]) =
  for idx, nom in cerata:
    let ceras = parseCeras(nom)

    printContent(idx, $ceras, ceras.ver, $prepare)

    # Skip virtual cerata
    if ceras.url == $Nil:
      printFooter(idx, $ceras, ceras.ver, $prepare)

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

    printFooter(idx, $ceras, ceras.ver, $prepare)

proc buildCerata*(cerata: openArray[string], stage = $native, resolve = true) =
  printHeader()

  let cluster = sortCerata(cerata, false)

  prepareCerata(cluster)

  echo ""

  printHeader()

  for idx, nom in (if resolve: cluster else: cerata.toSeq()):
    let
      ceras = parseCeras(nom)
      archive =
        $radPkgCache / $ceras /
        &"""{ceras}{(if ceras.url == $Nil: "" else: &"-\{ceras.ver\}")}{tarZst}"""
      log = open(
        getEnv($LOGD) / &"""{ceras}{(if stage == $native: "" else: &".\{stage\}")}""",
        fmWrite,
      )
      tmp = getEnv($TMPD) / $ceras

    printContent(idx, $ceras, ceras.ver, $build)

    if stage == $native:
      if fileExists(archive):
        printFooter(idx, $ceras, ceras.ver, $build)

        continue

      putEnv($SACD, $radPkgCache / $ceras / $sac)
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
      &"""{sh} -c 'nom={ceras} ver={ceras.ver} . {$radClustersCerataLib / $ceras / (if stage == $native: $build else: $build & '.' & stage)} && prepare && configure && build && package'"""
    )

    log.writeLine(shell.output.strip())
    log.close()

    if shell.exitCode != QuitSuccess:
      abort(&"{shell.exitCode:<8}{ceras:24}{ceras.ver:24}")

    if stage == $native:
      let
        sac = getEnv($SACD)
        status = createTarZst(archive, sac)

      # Purge
      # if $radNop.empty notin $ceras.nop:

      if status == QuitSuccess:
        genFiles(sac, $radPkgCache / $ceras / $files)
        removeDir(sac)

      if $bootstrap in $ceras.nop:
        discard extractTar(archive, $root)

    printFooter(idx, $ceras, ceras.ver, $build)

proc installCerata*(
    cerata: openArray[string], cache = $radPkgCache, fs = $root, lib = $radPkgLib
) =
  printHeader()

  let cluster = sortCerata(cerata)

  for idx, nom in cluster:
    let ceras = parseCeras(nom)

    printContent(idx, $ceras, ceras.ver, $install)

    discard extractTar(
      cache / $ceras /
        &"""{ceras}{(if ceras.url == $Nil: "" else: &"-\{ceras.ver\}")}{tarZst}""",
      fs,
    )

    createDir(lib / $ceras)
    writeFile(lib / $ceras / "ver", ceras.ver)
    copyFileWithPermissions(cache / $ceras / $files, lib / $ceras / $files)

    printFooter(idx, $ceras, ceras.ver, $install)

proc listCerata*() =
  printCerata(walkDir($radPkgLib, true, skipSpecial = true).toSeq().unzip()[1])

proc removeCerata*(cerata: openArray[string]) =
  printHeader()

  let cluster = sortCerata(cerata)

  for idx, nom in cluster:
    let ceras = parseCeras(nom)

    printContent(idx, $ceras, ceras.ver, $remove)

    for line in lines($radPkgLib / $ceras / $files):
      let path = &"/{line}"

      if fileExists(path):
        removeFile(path)

    for line in lines($radPkgLib / $ceras / $files):
      let path = &"/{line}"

      if dirExists(path):
        if path.isEmpty():
          removeDir(path)

    removeDir($radPkgLib / $ceras)

    printFooter(idx, $ceras, ceras.ver, $remove)

proc searchCerata*(pattern: openArray[string]) =
  var cerata: seq[string]

  for ceras in walkDir($radClustersCerataLib, true, skipSpecial = true):
    for nom in pattern:
      if nom.toLowerAscii() in ceras[1]:
        cerata.add(ceras[1])

  if cerata.len() == 0:
    exit(status = QuitFailure)

  sort(cerata)

  printCerata(cerata)
