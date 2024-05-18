# Copyright (c) 2018-2024, Firas Khalil Khana
# Distributed under the terms of the ISC License

import
  std/[algorithm, os, osproc, sequtils, strformat, strutils, tables, terminal, times],
  constants, tools,
  hashlib/misc/blake3, toml_serialization, toposort

type
  Ceras = object
    nom, ver, cmt, url, sum, bld, run = $Nil

# Check if the `ceras` src is extracted
proc checkExtractSrc(file: string): bool =
  walkDir(parentDir(file)).toSeq().len() > 1

proc cleanCerata*() =
  removeDir($radLog)
  removeDir($radTmp)

proc distcleanCerata*() =
  removeDir($radCacheBin)
  removeDir($radCacheSrc)
  removeDir($radCacheLocal)

  cleanCerata()

# Return the full path to the `ceras` file
proc getCerasPath(nom: string): string =
  $radLibClustersCerata / nom / $ceras

# Check if the `ceras` file exists
proc checkCerasExist(nom: string) =
  if not fileExists(getCerasPath(nom)):
    abort(&"""{"nom":8}{nom:48}""")

# Parse the `ceras` file
proc parseCeras(nom: string): Ceras =
  Toml.loadFile(getCerasPath(nom), Ceras)

proc printCerata*(cerata: openArray[string]) =
  for nom in cerata.deduplicate():
    checkCerasExist(nom)

    let ceras = parseCeras(nom)

    echo &"nom  :: {ceras.nom}"
    echo &"ver  :: {ceras.ver}"
    echo &"cmt  :: {ceras.cmt}"
    echo &"url  :: {ceras.url}"
    echo &"sum  :: {ceras.sum}"
    echo &"bld  :: {ceras.bld}"
    echo &"run  :: {ceras.run}"

    echo ""

proc printContent(idx: int, nom, ver, cmd: string) =
  styledEcho fgMagenta, styleBright, &"""{idx + 1:<8}{nom:24}{ver:24}{cmd:8}{now().format("hh:mm tt")}"""

proc printFooter(idx: int, nom, ver, cmd: string) =
  echo &"""{idx + 1:<8}{nom:24}{ver:24}{cmd:8}{now().format("hh:mm tt")}"""

proc printHeader() =
  echo &"""{"idx":8}{"nom":24}{"ver":24}{"cmd":8}fin"""

# Resolve deps using topological sorting
proc resolveDeps(nom: string, deps: var Table[string, seq[string]], run = true) =
  let
    ceras = parseCeras(nom)
    dep = if run: ceras.run else: ceras.bld

  deps[ceras.nom] =
    case dep
    of $Nil:
      @[]
    else:
      dep.split()

  if deps[ceras.nom].len() > 0:
    for dep in deps[ceras.nom]:
      resolveDeps(dep, deps, if run: true else: false)

proc checkCerata*(cerata: openArray[string], run = true): seq[string] =
  var deps: Table[string, seq[string]]

  for nom in cerata.deduplicate():
    checkCerasExist(nom)

    resolveDeps(nom, deps, if run: true else: false)

  topoSort(deps)

proc fetchCerata(cerata: openArray[string]) =
  printHeader()

  for idx, nom in cerata:
    let ceras = parseCeras(nom)

    # Check for virtual cerata
    case ceras.url
    of $Nil:
      printFooter(idx, ceras.nom, ceras.ver, $fetch)
    else:
      let
        path = getEnv($SRCD) / ceras.nom
        archive = path / lastPathPart(ceras.url)

      if dirExists(path):
        case ceras.ver
        of $git:
          printFooter(idx, ceras.nom, ceras.cmt, $fetch)
        else:
          if verifyFile(archive, ceras.sum):
            if not checkExtractSrc(archive):
              printContent(idx, ceras.nom, ceras.ver, $fetch)

              discard extractTar(archive, path)

              cursorUp 1
              eraseLine()

            printFooter(idx, ceras.nom, ceras.ver, $fetch)
          else:
            printContent(idx, ceras.nom, ceras.ver, $fetch)

            removeDir(path)
            createDir(path)

            discard downloadFile(archive, ceras.url)

            if verifyFile(archive, ceras.sum):
              discard extractTar(archive, path)
            else:
              cursorUp 1
              eraseLine()

              abort(&"""{"sum":8}{ceras.nom:24}{ceras.ver:24}""")

            cursorUp 1
            eraseLine()

            printFooter(idx, ceras.nom, ceras.ver, $fetch)
      else:
        case ceras.ver
        of $git:
          printContent(idx, ceras.nom, ceras.cmt, $clone)

          discard gitCloneRepo(ceras.url, path)
          discard gitCheckoutRepo(path, ceras.cmt)

          cursorUp 1
          eraseLine()

          printFooter(idx, ceras.nom, ceras.cmt, $clone)
        else:
          printContent(idx, ceras.nom, ceras.ver, $fetch)

          createDir(path)

          discard downloadFile(archive, ceras.url)

          if verifyFile(archive, ceras.sum):
            discard extractTar(archive, path)
          else:
            cursorUp 1
            eraseLine()

            abort(&"""{"sum":8}{ceras.nom:24}{ceras.ver:24}""")

          cursorUp 1
          eraseLine()

          printFooter(idx, ceras.nom, ceras.ver, $fetch)

proc buildCerata*(cerata: openArray[string], stage = $native, resolve = true) =
  let cluster = checkCerata(cerata, false)

  fetchCerata(cluster)

  echo ""

  printHeader()

  for idx, nom in (if resolve: cluster else: cerata.toSeq()):
    let ceras = parseCeras(nom)

    printContent(idx, ceras.nom,
      case ceras.ver
      of $git:
        ceras.cmt
      else:
        ceras.ver
    , $build)

    case stage
    of $native:
      if fileExists($radCacheLocal / ceras.nom / &"""{ceras.nom}{(
        case ceras.url
        of $Nil:
          ""
        else:
          '-' & ceras.ver
      )}{(
        case ceras.ver
        of $git:
          '-' & ceras.cmt
        else:
          ""
      )}{tarZst}"""):
        cursorUp 1
        eraseLine()

        printFooter(idx, ceras.nom,
          case ceras.ver
          of $git:
            ceras.cmt
          else:
            ceras.ver
        , $build)

        continue

      putEnv($SACD, $radCacheLocal / ceras.nom / $sac)
      createDir(getEnv($SACD))

    # We only use `nom` and `ver` from `ceras`
    #
    # All phases need to be called sequentially to prevent the loss of the
    # current working dir...
    var status = execCmd(&"""
      {sh} {shellCommand} 'nom={ceras.nom} ver={ceras.ver} {CurDir} {$radLibClustersCerata / ceras.nom / $build}{CurDir}{stage} &&
      prepare $1 &&
      configure $1 &&
      build $1 &&
      check $1 &&
      package $1'
    """ % &">> {getEnv($LOGD) / ceras.nom}{CurDir}{log} 2>&1")

    if status != 0:
      abort(&"""{status:<8}{ceras.nom:24}{(
        case ceras.ver
        of $git:
          ceras.cmt
        else:
          ceras.ver
      ):24}""")

    case stage
    of $native:
      status = createTarZst($radCacheLocal / ceras.nom / &"""{ceras.nom}{(
        case ceras.url
        of $Nil:
          ""
        else:
          '-' & ceras.ver
      )}{(
        case ceras.ver
        of $git:
          '-' & ceras.cmt
        else:
          ""
      )}{tarZst}""", getEnv($SACD))

      if status == 0:
        genSum(getEnv($SACD), $radCacheLocal / ceras.nom / $sum)

        removeDir(getEnv($SACD))

    cursorUp 1
    eraseLine()

    printFooter(idx, ceras.nom,
      case ceras.ver
      of $git:
        ceras.cmt
      else:
        ceras.ver
    , $build)

proc installCerata*(cerata: openArray[string]) =
  let cluster = checkCerata(cerata)

  printHeader()

  for idx, nom in cluster:
    let ceras = parseCeras(nom)

    printContent(idx, ceras.nom,
      case ceras.ver
      of $git:
        ceras.cmt
      else:
        ceras.ver
    , $install)

    let status = extractTar($radCacheLocal / ceras.nom / &"""{ceras.nom}{(
      case ceras.url
      of $Nil:
        ""
      else:
        '-' & ceras.ver
    )}{(
      case ceras.ver
      of $git:
        '-' & ceras.cmt
      else:
        ""
    )}{tarZst}""", $DirSep)

    if status != 0:
      abort(&"""{status:<8}{ceras.nom:24}{(
        case ceras.ver
        of $git:
          ceras.cmt
        else:
          ceras.ver
      ):24}""")

    createDir($radLibLocal / ceras.nom)

    writeFile($radLibLocal / ceras.nom / "ver", ceras.ver)

    discard rsync($radCacheLocal / ceras.nom / $sum, $radLibLocal / ceras.nom)

    cursorUp 1
    eraseLine()

    printFooter(idx, ceras.nom,
      case ceras.ver
      of $git:
        ceras.cmt
      else:
        ceras.ver
    , $install)

proc listCerata*() =
  printCerata(walkDir($radLibLocal, relative = true, skipSpecial = true).toSeq().unzip()[1])

proc removeCerata*(cerata: openArray[string]) =
  let cluster = checkCerata(cerata)

  printHeader()

  for idx, nom in cluster:
    let ceras = parseCeras(nom)

    printContent(idx, ceras.nom,
      case ceras.ver
      of $git:
        ceras.cmt
      else:
        ceras.ver
    , $remove)

    for line in lines($radLibLocal / ceras.nom / $sum):
      removeFile(DirSep & line.split()[2])

    removeDir($radLibLocal / ceras.nom)

    cursorUp 1
    eraseLine()

    printFooter(idx, ceras.nom,
      case ceras.ver
      of $git:
        ceras.cmt
      else:
        ceras.ver
    , $remove)

proc searchCerata*(pattern: openArray[string]) =
  var cerata: seq[string]

  for ceras in walkDir($radLibClustersCerata, relative = true, skipSpecial = true):
    for nom in pattern:
      if nom.toLowerAscii() in ceras[1]:
        cerata.add(ceras[1])

  if cerata.len() == 0:
    exit(QuitFailure)

  sort(cerata)

  printCerata(cerata)
