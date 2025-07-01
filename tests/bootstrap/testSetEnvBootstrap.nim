# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/[os, strutils], ../../src/[bootstrap, constants]

setEnvBootstrap()

echo "CORD  :: ", getEnv("CORD")
echo "CRSD  :: ", getEnv("CRSD")
echo "TMPD  :: ", getEnv("TMPD")
echo "TLCD  :: ", getEnv("TLCD")

echo ""

echo "PATH  :: ", getEnv("PATH")

doAssert getEnv("CORD").endsWith("core")
doAssert getEnv("CRSD").endsWith("cross")
doAssert getEnv("TMPD").endsWith("tmp")
doAssert getEnv("TLCD").endsWith("toolchain")

doAssert getEnv($PATH).split(':')[0].endsWith("toolchain/usr/bin")
