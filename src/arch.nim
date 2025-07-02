# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/[os, osproc, strutils, times], constants

proc setEnvArch*() =
  let env = [
    ("ARCH", "x86-64"),
    ("BUILD", execCmdEx(pathCoreRepo / "slibtool/files/config.guess").output.strip()),
    ("CTARGET", "x86_64-glaucus-linux-musl"),
    ("PRETTY_NAME", "glaucus s6 x86-64-v3 " & now().format("YYYYMMdd")),
    ("TARGET", "x86_64-pc-linux-musl"),
  ]

  for (i, j) in env:
    putEnv(i, j)
