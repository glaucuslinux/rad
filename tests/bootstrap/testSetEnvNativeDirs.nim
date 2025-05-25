# SPDX-License-Identifier: MPL-2.0
# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/os, ../../src/[bootstrap, constants]

setEnvNativeDirs()

echo "CERD  :: ", getEnv($CERD)
echo "LOGD  :: ", getEnv($LOGD)
echo "PKGD  :: ", getEnv($PKGD)
echo "SRCD  :: ", getEnv($SRCD)
echo "TMPD  :: ", getEnv($TMPD)

doAssert getEnv($CERD) == "/var/lib/rad/clusters/cerata"
doAssert getEnv($LOGD) == "/var/log/rad"
doAssert getEnv($PKGD) == "/var/cache/rad/pkg"
doAssert getEnv($SRCD) == "/var/cache/rad/src"
doassert getEnv($TMPD) == "/var/tmp/rad"
