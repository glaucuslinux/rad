# SPDX-License-Identifier: MPL-2.0
# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/[algorithm, os, osproc, strformat, strutils, terminal, times], constants

proc createTarZst*(archive, dir: string): int =
  execCmd(&"{tar} --use-compress-program '{zstd} -3 -T0' -cP -f {archive} -C {dir} .")

proc downloadFile*(url, file: string): int =
  execCmd(&"{curl} -fL -o {file} -s {url}")

proc exit*(msg = "", status = QuitSuccess) =
  removeFile($radLock)

  if not msg.isEmptyOrWhitespace():
    quit(msg, status)

  quit(status)

proc abort*(err: string, status = QuitFailure) =
  styledEcho fgRed, styleBright, &"""{err}{"abort":8}{now().format("hh:mm tt")}"""

  exit(status = status)

proc extractTar*(archive, dir: string): int =
  execCmd(&"{tar} --no-same-owner -xmP -f {archive} -C {dir}")

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
    $make,
    $perl,
    $pigz,
    $pkgConfig,
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
