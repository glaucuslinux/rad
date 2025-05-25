# SPDX-License-Identifier: MPL-2.0
# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/[os, strformat, strutils], constants

proc setEnvFlags*() =
  putEnv($CFLAGS, $cflags)
  putEnv($CXXFLAGS, $cflags)
  putEnv($LDFLAGS, &"{ldflags} {cflags}")
  putEnv($MAKEFLAGS, $radFlags.make)

proc setEnvFlagsNopLto*() =
  putEnv($CFLAGS, replace($cflags, $radFlags.lto))
  putEnv($CXXFLAGS, getEnv($CFLAGS))
  putEnv($LDFLAGS, $ldflags)
  putEnv($MAKEFLAGS, $radFlags.make)

proc setEnvFlagsNopParallel*() =
  putEnv($MAKEFLAGS, $radFlags.parallel)
