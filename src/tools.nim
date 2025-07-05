# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/[os, osproc, strformat, strutils, terminal, times]

proc createTarZst*(archive, dir: string): int =
  execCmd(&"tar --use-compress-program 'zstd -3 -T0' -cP -f {archive} -C {dir} .")

proc downloadFile*(dir, url: string): int =
  execCmd(&"curl -fL --output-dir {dir} -Os {url}")

proc exit*(radLock = "/var/tmp/rad.lock", msg = "", status = QuitSuccess) =
  removeFile(radLock)

  if not msg.isEmptyOrWhitespace():
    quit(msg, status)

  quit(status)

proc abort*(err: string, status = QuitFailure) =
  styledEcho fgRed, styleBright, err & &"""{"abort":8}""" & now().format("hh:mm tt")

  exit(status = status)

proc extractTar*(archive, dir: string): int =
  execCmd(&"tar --no-same-owner -xmP -f {archive} -C {dir}")

proc gitCheckoutRepo*(dir, cmt: string): int =
  execCmd(&"git -C {dir} checkout {cmt} -q")

proc gitCloneRepo*(url, dir: string): int =
  execCmd(&"git clone {url} {dir} -q")

proc interrupt() {.noconv.} =
  abort(&"""{$QuitFailure:8}{"interrupt received":48}""")

proc lock*(radLock = "/var/tmp/rad.lock") =
  if fileExists(radLock):
    styledEcho fgRed,
      styleBright,
      &"""{$QuitFailure:8}{"lock exists":48}{"abort":8}""" & now().format("hh:mm tt")

    quit(QuitFailure)

  writeFile(radLock, "")

  setControlCHook(interrupt)

proc xxhsum(file: string): string =
  execCmdEx(&"xxhsum -H2 {file}").output.strip()

proc verifyFile*(file, sum: string): bool =
  fileExists(file) and xxhsum(file).split()[0] == sum
