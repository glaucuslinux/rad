# Copyright (c) 2018-2025, Firas Khalil Khana
# Distributed under the terms of the ISC License

import std/[algorithm, os, osproc, strformat, strutils, terminal, times], constants

proc createTarZst*(archive, dir: string): int =
  execCmd(
    &"{tar} --use-compress-program '{zstd} {zstdCompress}' -cPf {archive} -C {dir} ."
  )

proc downloadFile*(url, file: string): int =
  execCmd(&"{wget2} -q -O {file} -c -N {url}")

proc exit*(msg = "", status = QuitSuccess) =
  removeFile($radLock)

  if not msg.isEmptyOrWhitespace():
    quit(msg, status)

  quit(status)

proc abort*(err: string, status = QuitFailure) =
  styledEcho fgRed, styleBright, &"""{err}{"abort":8}{now().format("hh:mm tt")}"""

  exit(status = status)

proc extractTar*(archive, dir: string): int =
  execCmd(&"{tar} --no-same-owner -xmPf {archive} -C {dir}")

proc genContents*(dir, contents: string) =
  var entries: seq[string]

  for entry in walkDirRec(
    dir, yieldFilter = {pcFile .. pcLinkToDir}, relative = true, skipSpecial = true
  ):
    entries &= (
      if getFileInfo(dir / entry, followSymlink = false).kind == pcDir: &"{entry}/"
      else: $entry
    )

  let contents = open(contents, fmWrite)

  for entry in entries.sorted():
    contents &= &"{entry}\n"

  contents.close()

proc gitCheckoutRepo*(dir, cmt: string): int =
  execCmd(&"{git} -C {dir} checkout {cmt} -q")

proc gitCloneRepo*(url, dir: string): int =
  execCmd(&"{git} clone {url} {dir} -q")

proc interrupt*() {.noconv.} =
  abort(&"""{$QuitFailure:8}{"interrupt received":48}""")

proc isEmpty*(dir: string): bool =
  for entry in walkDir(dir):
    return
  return true

proc lock*() =
  if fileExists($radLock):
    styledEcho fgRed,
      styleBright,
      &"""{$QuitFailure:8}{"lock exists":48}{"abort":8}{now().format("hh:mm tt")}"""

    quit(QuitFailure)

  writeFile($radLock, "")

  setControlCHook(interrupt)

proc require*() =
  for exe in [
    $autoconf,
    $automake,
    $awk,
    $bash,
    $bzip2,
    $gcc,
    $git,
    $grep,
    $lex,
    $limine,
    $m4,
    $radCerata.make,
    $perl,
    $pigz,
    $pkgconf,
    $tar,
    $xz,
    $yacc,
    $zstd,
  ]:
    if findExe(exe).isEmptyOrWhitespace():
      abort(&"""{127:8}{&"\{exe\} not found":48}""")

proc xxhsum(file: string): string =
  execCmdEx(&"xxhsum -H2 {file}").output.strip()

proc verifyFile*(file, sum: string): bool =
  fileExists(file) and xxhsum(file).split()[0] == sum

proc verifyContents*(contents: string): bool =
  for file in lines(contents):
    if not fileExists(file):
      return
