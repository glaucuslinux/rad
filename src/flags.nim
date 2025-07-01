# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/[os, strformat, strutils], constants

proc setEnvFlags*() =
  for i in [
    ("CFLAGS", cflags),
    ("CXXFLAGS", cflags),
    ("LDFLAGS", &"{ldflags} {cflags}"),
    ("MAKEFLAGS", makeflags),
  ]:
    putEnv(i[0], i[1])

proc setEnvFlagsNoLTO*() =
  for i in [
    ("CFLAGS", replace(cflags, $ltoflags)),
    ("CXXFLAGS", replace(cflags, $ltoflags)),
    ("LDFLAGS", ldflags),
    ("MAKEFLAGS", makeflags),
  ]:
    putEnv(i[0], i[1])

proc setEnvFlagsNoParallel*() =
  putEnv("MAKEFLAGS", "-j 1")
