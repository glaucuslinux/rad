# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/os, ../../src/[constants, flags]

setEnvFlagsNoParallel()

echo "MAKEFLAGS  :: ", getEnv($MAKEFLAGS)

doAssert getEnv($MAKEFLAGS) == "-j 1"
