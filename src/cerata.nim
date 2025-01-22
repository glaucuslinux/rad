# Copyright (c) 2018-2025, Firas Khalil Khana
# Distributed under the terms of the ISC License

import
  std/[algorithm, os, osproc, sequtils, strformat, strutils, tables, terminal, times],
  constants,
  flags,
  tools,
  hashlib/misc/blake3,
  toml_serialization,
  toposort

type Ceras = object
  nom, ver, url, sum, bld, run, nop = $Nil

proc `$`(self: Ceras): string =
  self.nom

proc cleanCerata*() =
  # removeDir($radLog)
  removeDir($radTmp)

proc distcleanCerata*() =
  cleanCerata()

  # removeDir($radLog)
  removeDir($radPkgCache)
  removeDir($radSrcCache)

proc parseCeras(nom: string): Ceras =
  let path = $radClustersCerataLib / nom / $ceras

  if not fileExists(path):
    abort(&"""{"nom":8}{nom:48}""")

  Toml.loadFile(path, Ceras)

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

proc resolveDeps(nom: string, deps: var Table[string, seq[string]], run = true) =
  let
    ceras = parseCeras(nom)
    dep = if run: ceras.run else: ceras.bld

  deps[$ceras] =
    case dep
    of $Nil:
      @[]
    else:
      dep.split()

  if deps[$ceras].len() > 0:
    for dep in deps[$ceras]:
      resolveDeps(dep, deps, run)

proc sortCerata*(cerata: openArray[string], run = true): seq[string] =
  var deps: Table[string, seq[string]]

  for nom in cerata.deduplicate():
    resolveDeps(nom, deps, run)

  topoSort(deps)

proc prepareCerata(cerata: openArray[string]) =
  for idx, nom in cerata:
    let ceras = parseCeras(nom)

    printContent(idx, $ceras, ceras.ver, $prepare)

    # Skip virtual cerata
    case ceras.url
    of $Nil:
      discard
    else:
      let
        src = getEnv($SRCD) / $ceras
        tmp = getEnv($TMPD) / $ceras

      case ceras.sum
      of $Nil:
        if not dirExists(src):
          discard gitCloneRepo(ceras.url, src)
          discard gitCheckoutRepo(src, ceras.ver)

        copyDirWithPermissions(src, tmp)
      else:
        let archive = src / lastPathPart(ceras.url)

        if not verifyFile(archive, ceras.sum):
          removeDir(src)
          createDir(src)

          discard downloadFile(archive, ceras.url)

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

  printHeader()

  for idx, nom in (if resolve: cluster else: cerata.toSeq()):
    let
      ceras = parseCeras(nom)
      archive =
        $radPkgCache / $ceras /
        &"""{ceras}{(case ceras.url
        of $Nil: ""
        else: '-' & ceras.ver)}{tarZst}"""
      tmp = getEnv($TMPD) / $ceras

    printContent(idx, $ceras, ceras.ver, $build)

    case stage
    of $native:
      if fileExists(archive):
        printFooter(idx, $ceras, ceras.ver, $build)

        continue

      putEnv($SACD, $radPkgCache / $ceras / $sac)
      createDir(getEnv($SACD))

      setEnvFlags()

      if $radFlags.lto in $ceras.nop:
        setEnvFlagsNopLto()

      if $radFlags.parallel in $ceras.nop:
        setEnvFlagsNopParallel()

    putEnv($MAKEFLAGS, $radFlags.make)

    if dirExists(tmp):
      setCurrentDir(tmp)
    if dirExists(tmp / &"{ceras}-{ceras.ver}"):
      setCurrentDir(tmp / &"{ceras}-{ceras.ver}")

    # Only use `nom` and `ver` from `ceras`
    #
    # Call all phases sequentially to preserve the current working dir
    var status = execCmd(
      &"""{sh} -c 'nom={ceras} ver={ceras.ver} {CurDir} {$radClustersCerataLib / $ceras / (
        case stage
        of $native: $build
        else: $build & CurDir & stage)} &&
      prepare $1 &&
      configure >$1 &&
      build >$1 &&
      package >$1'""" %
        &"""> {getEnv($LOGD) / (
        case stage
        of $native: $ceras
        else:
          createDir(getEnv($LOGD) / $stage)
          $stage / $ceras
      )}{CurDir}{log} 2>&1"""
    )

    if status != 0:
      abort(&"""{status:<8}{ceras:24}{ceras.ver:24}""")

    case stage
    of $native:
      status = createTarZst(archive, getEnv($SACD))

      if status == 0:
        genSum(getEnv($SACD), $radPkgCache / $ceras / $sum)
        removeDir(getEnv($SACD))

      if $bootstrap in $ceras.nop:
        discard extractTar(archive, $DirSep)

    printFooter(idx, $ceras, ceras.ver, $build)

proc installCerata*(
    cerata: openArray[string], cache = $radPkgCache, fs = $DirSep, lib = $radPkgLib
) =
  printHeader()

  let cluster = sortCerata(cerata)

  for idx, nom in cluster:
    let ceras = parseCeras(nom)

    printContent(idx, $ceras, ceras.ver, $install)

    discard extractTar(
      cache / $ceras /
        &"""{ceras}{(
      case ceras.url
      of $Nil: ""
      else: '-' & ceras.ver
      )}{tarZst}""",
      fs,
    )

    createDir(lib / $ceras)
    writeFile(lib / $ceras / "ver", ceras.ver)
    copyFileWithPermissions(cache / $ceras / $sum, lib / $ceras / $sum)

    printFooter(idx, $ceras, ceras.ver, $install)

proc listCerata*() =
  printCerata(walkDir($radPkgLib, true, skipSpecial = true).toSeq().unzip()[1])

proc removeCerata*(cerata: openArray[string]) =
  let cluster = sortCerata(cerata)

  printHeader()

  for idx, nom in cluster:
    let ceras = parseCeras(nom)

    printContent(idx, $ceras, ceras.ver, $remove)

    for line in lines($radPkgLib / $ceras / $sum):
      removeFile(DirSep & line.split()[2])

    removeDir($radPkgLib / $ceras)

    printFooter(idx, $ceras, ceras.ver, $remove)

proc searchCerata*(pattern: openArray[string]) =
  var cerata: seq[string]

  for ceras in walkDir($radClustersCerataLib, true, skipSpecial = true):
    for nom in pattern:
      if nom.toLowerAscii() in ceras[1]:
        cerata.add(ceras[1])

  if cerata.len() == 0:
    exit(QuitFailure)

  sort(cerata)

  printCerata(cerata)
