# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/[algorithm, os, osproc, strformat, strutils, terminal, times], constants

proc createTarZst*(ar, dir: string): int =
  execCmd(&"tar --use-compress-program 'zstd -3 -T0' -cP -f {ar} -C {dir} .")

proc downloadFile*(dir, url: string): int =
  execCmd(&"curl -fL --output-dir {dir} -Os {url}")

proc exit*(msg = "", status = QuitSuccess) =
  removeFile(pathTmpLock)

  if not msg.isEmptyOrWhitespace():
    quit(msg, status)

  quit(status)

proc abort*(err: string, status = QuitFailure) =
  styledEcho fgRed, styleBright, err & &"""{"abort":8}""" & now().format("hh:mm tt")

  exit(status = status)

proc extractTar*(ar, dir: string): int =
  execCmd(&"tar --no-same-owner -xmP -f {ar} -C {dir}")

proc genContents*(dir, contents: string) =
  var entries: seq[string]

  for entry in walkDirRec(
    dir, yieldFilter = {pcFile .. pcLinkToDir}, relative = true, skipSpecial = true
  ):
    entries &= (
      if getFileInfo(dir / entry, followSymlink = false).kind == pcDir: entry & '/'
      else: entry
    )

  let contents = open(contents, fmWrite)

  for entry in entries.sorted():
    contents &= entry & '\n'

  contents.close()

proc gitCheckoutRepo*(dir, cmt: string): int =
  execCmd(&"git -C {dir} checkout {cmt} -q")

proc gitCloneRepo*(url, dir: string): int =
  execCmd(&"git clone {url} {dir} -q")

proc interrupt*() {.noconv.} =
  abort(&"""{$QuitFailure:8}{"interrupt received":48}""")

proc isEmpty*(dir: string): bool =
  for entry in walkDir(dir):
    return
  return true

proc lock*() =
  if fileExists(pathTmpLock):
    styledEcho fgRed,
      styleBright,
      &"""{$QuitFailure:8}{"lock exists":48}{"abort":8}""" & now().format("hh:mm tt")

    quit(QuitFailure)

  writeFile(pathTmpLock, "")

  setControlCHook(interrupt)

proc require*() =
  const tools = [
    "autoconf", "automake", "autopoint", "awk", "bash", "booster", "bzip2", "curl",
    "diff", "find", "gcc", "git", "grep", "gzip", "ld.bfd", "lex", "libtool", "limine",
    "m4", "make", "meson", "mkfs.erofs", "mkfs.fat", "ninja", "patch", "perl",
    "pkg-config", "sed", "tar", "xz", "yacc", "zstd",
  ]

  for i in tools:
    if findExe(i).isEmptyOrWhitespace():
      abort(&"""{127:8}{&"\{i\} not found":48}""")

proc verifyContents*(contents: string): bool =
  for file in lines(contents):
    if not fileExists(file):
      return

proc xxhsum(file: string): string =
  execCmdEx(&"xxhsum -H2 {file}").output.strip()

proc verifyFile*(file, sum: string): bool =
  fileExists(file) and xxhsum(file).split()[0] == sum
