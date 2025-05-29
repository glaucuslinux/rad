# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/[os, osproc, strformat, strutils, times], constants

proc setEnvArch*(stage = native) =
  putEnv($ARCH, $x86_64)
  putEnv(
    $BLD,
    execCmdEx($radClustersCerataLib / $slibtool / $files / $configGuess).output.strip(),
  )
  putEnv($CARCH, $x86_64_v3)
  putEnv($PRETTY_NAME, &"""{glaucus} {s6} {x86_64_v3} {now().format("YYYYMMdd")}""")
  putEnv($TGT, &"{x86_64Linux}-{(if stage == native: tupleNative else: tupleCross)}")
