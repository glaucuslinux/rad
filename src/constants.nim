# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

const
  # path
  pathPkgCache* = "/var/cache/rad/pkg"
  pathSrcCache* = "/var/cache/rad/src"
  pathLocalLib* = "/var/lib/rad/local"
  pathCoreRepo* = "/var/lib/rad/repo/core"
  pathExtraRepo* = "/var/lib/rad/repo/extra"
  pathLog* = "/var/log/rad"
  pathTmp* = "/var/tmp/rad"
  pathTmpLock* = "/var/tmp/rad.lock"

  # shell
  shellRedirect* = ">/dev/null 2>&1"

type Stages* = enum
  cross
  native
  toolchain
