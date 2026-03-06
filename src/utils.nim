# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2026 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/[os, osproc, strformat, strutils]

proc requireCmd(cmd: string) =
  if findExe(cmd).isEmptyOrWhitespace():
    raise newException(OSError, &"{cmd}: no such command")

proc requireDir(dir: string) =
  if not dirExists(dir):
    raise newException(IOError, &"{dir}: no such directory")

proc requireFile(file: string) =
  if not fileExists(file):
    raise newException(IOError, &"{file}: no such file")

proc requireParentDir(path: string) =
  let parent = path.parentDir()
  if parent.len() > 0 and not dirExists(parent):
    raise newException(IOError, &"{parent}: no such directory")

template checkResult(exitCode: int, output: string, dsc: string) =
  if exitCode != QuitSuccess:
    let details =
      if output.strip().len() > 0:
        output.strip()
      else:
        "no additional output"
    raise newException(OSError, dsc & ":\n" & details)
    # This does not work... weird...
    # raise newException(OSError, &"{dsc}:\n{details}")

proc createTar*(archive: string, dir: string) =
  requireCmd("tar")
  requireCmd("zstd")
  requireDir(dir)
  requireParentDir(archive)

  let (output, exitCode) = execCmdEx(
    &"tar --use-compress-program 'zstd -3 -T0' -cP -f {quoteShellPosix(archive)} -C {quoteShellPosix(dir)} .",
    options = {poStdErrToStdOut, poUsePath},
  )

  checkResult(
    exitCode,
    output,
    &"tar failed with exit code {exitCode} while creating {archive} from {dir}",
  )

proc downloadFile*(url: string, dir: string) =
  requireCmd("curl")
  requireDir(dir)

  let (output, exitCode) = execCmdEx(
    &"curl -fL --output-dir {quoteShellPosix(dir)} -OSs {quoteShellPosix(url)}",
    options = {poStdErrToStdOut, poUsePath},
  )

  checkResult(
    exitCode,
    output,
    &"curl failed with exit code {exitCode} while downloading {url} to {dir}",
  )

proc extractTar*(archive: string, dir: string) =
  requireCmd("tar")
  requireFile(archive)
  requireDir(dir)

  let (output, exitCode) = execCmdEx(
    &"tar --no-same-owner -xmP -f {quoteShellPosix(archive)} -C {quoteShellPosix(dir)}",
    options = {poStdErrToStdOut, poUsePath},
  )

  checkResult(
    exitCode,
    output,
    &"tar failed with exit code {exitCode} while extracting {archive} to {dir}",
  )

proc gitCheckoutRepo*(dir: string, cmt: string) =
  requireCmd("git")
  requireDir(dir)

  var (output, exitCode) = execCmdEx(
    &"git -C {quoteShellPosix(dir)} rev-parse --git-dir",
    options = {poStdErrToStdOut, poUsePath},
  )

  checkResult(exitCode, output, &"fatal: {dir} is not a git repository")

  (output, exitCode) = execCmdEx(
    &"git -C {quoteShellPosix(dir)} checkout {quoteShellPosix(cmt)} -q",
    options = {poStdErrToStdOut, poUsePath},
  )

  checkResult(
    exitCode,
    output,
    &"git checkout failed with exit code {exitCode} for commit {cmt} in {dir}",
  )

proc gitCloneRepo*(url: string, dir: string) =
  requireCmd("git")

  if dirExists(dir):
    raise newException(IOError, &"fatal: destination path {dir} already exists")

  requireParentDir(dir)

  let (output, exitCode) = execCmdEx(
    &"git clone {quoteShellPosix(url)} {quoteShellPosix(dir)} -q",
    options = {poStdErrToStdOut, poUsePath},
  )

  checkResult(
    exitCode, output, &"git clone failed with exit code {exitCode} for {url} to {dir}"
  )

proc verifyFile*(file: string, sum: string): bool =
  requireCmd("xxhsum")
  requireFile(file)

  let (output, exitCode) = execCmdEx(
    &"xxhsum -H2 {quoteShellPosix(file)}", options = {poStdErrToStdOut, poUsePath}
  )

  checkResult(exitCode, output, &"xxhsum failed with exit code {exitCode} for {file}")

  output.strip().split()[0] == sum
