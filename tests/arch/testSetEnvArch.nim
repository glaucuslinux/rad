# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/[os, strutils], ../../src/[arch, constants]

setEnvArch(cross)

echo "ARCH         :: ", getEnv($ARCH)
echo "BLD          :: ", getEnv($BLD)
echo "CARCH        :: ", getEnv($CARCH)
echo "PRETTY_NAME  :: ", getEnv($PRETTY_NAME)
echo "TGT          :: ", getEnv($TGT)

doAssert getEnv($ARCH) == "x86-64"
doAssert getEnv($BLD).startsWith("x86_64-pc-linux-")
doAssert getEnv($CARCH) == "x86-64-v3"
doAssert getEnv($PRETTY_NAME).startsWith("glaucus s6 x86-64-v3 ")
doAssert getEnv($TGT) == "x86_64-glaucus-linux-musl"
