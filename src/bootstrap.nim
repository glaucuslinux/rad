# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/os, constants

proc cleanBootstrap*() =
  for i in dirCleanBootstrap:
    removeDir(ParDir / i)

proc prepareBootstrap*() =
  for i in dirPrepareBootstrap:
    createDir(ParDir / i)

proc prepareCross*() =
  removeDir(ParDir / "tmp")
  createDir(ParDir / "tmp")

proc setEnvBootstrap*() =
  for i in envBootstrap:
    putEnv(i[0], absolutePath(ParDir) / i[1])

  putEnv("PATH", getEnv("TLCD") / "usr/bin" & ':' & getEnv("PATH"))

proc setEnvCross*() =
  for i in envCross:
    putEnv(i[0], i[1])

  for i in envPkgConfig:
    putEnv(i[0], getEnv("CRSD") / i[1])

proc setEnvNative*() =
  for i in envNative:
    putEnv(i[0], i[1])

  putEnv("CORD", pathCoreRepo)
  putEnv("TMPD", pathTmp)
