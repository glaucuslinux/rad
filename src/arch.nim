# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/[os, osproc, strformat, strutils], constants

proc setEnvArch*(vendor = "pc") =
  for i in [
    ("ARCH", "x86-64"),
    ("BLDT", execCmdEx(pathCoreRepo / "slibtool/files/config.guess").output.strip()),
    ("PRETTY_NAME", "glaucus s6 x86-64-v3"),
    ("TGTT", &"x86_64-{vendor}-linux-musl"),
  ]:
    putEnv(i[0], i[1])
