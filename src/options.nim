# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/[os, parseopt, strutils], arch, bootstrap, packages, constants, tools

proc options*() =
  const
    help =
      """
USAGE:
  rad [ COMMAND ]

COMMANDS:
  bootstrap  Bootstrap glaucus
  build      Build packages
  clean      Clean build cache
  contents   List package contents
  help       Show help message
  info       Show package information
  list       List built packages
  search     Search for packages
  update     Update repositories
  version    Show rad version"""
    helpBootstrap =
      """
USAGE:
  rad bootstrap [ COMMAND ]

COMMANDS:
  clean      Clean bootstrap cache
  help       Show help message
  1, stage1  Bootstrap stage 1 (toolchain)
  2, stage2  Bootstrap stage 2 (cross)
  3, stage3  Bootstrap stage 3 (native)"""
    version =
      """
rad version 0.1.0

Licensed under the Mozilla Public License Version 2.0 (MPL-2.0)
Copyright © 2018-2025 Firas Khana"""

  if paramCount() < 1:
    quit(help, QuitFailure)

  var p = initOptParser()

  p.next()

  case p.kind
  of cmdEnd:
    quit(help, QuitFailure)
  else:
    lock()

    case p.key
    of "bootstrap":
      p.next()

      case p.kind
      of cmdEnd:
        exit(helpBootstrap, QuitFailure)
      else:
        case p.key
        of "clean":
          cleanBootstrap()

          echo "clean complete"
        of "--help", "help":
          echo helpBootstrap
        of "1", "stage1", "toolchain":
          require()
          setEnvArch()
          setEnvBootstrap()
          cleanBootstrap()
          prepareBootstrap()

          let stage1 = parsePackage("toolchain").run.split()
          buildPackages(stage1, resolve = false, stage = toolchain)

          echo ""
          echo "stage 1 (toolchain) complete"
        of "2", "stage2", "cross":
          setEnvArch()
          setEnvBootstrap()
          setEnvCross()
          prepareCross()

          let stage2 = parsePackage("cross").run.split()
          buildPackages(stage2, resolve = false, stage = cross)

          echo ""
          echo "stage 2 (cross) complete"
        of "3", "stage3", "native":
          setEnvArch()
          setEnvNative()

          let stage3 = parsePackage("native").run.split()
          buildPackages(stage3, resolve = false)

          echo ""
          echo "stage 3 (native) complete"
        else:
          exit(helpBootstrap, QuitFailure)
    of "build":
      setEnvArch()
      setEnvNative()
      cleanPackages()

      buildPackages(remainingArgs(p))

      echo ""
      echo "build complete"
    of "clean":
      cleanPackages()

      echo "clean complete"
    of "contents":
      listContents(remainingArgs(p))
    of "--help", "help":
      echo help
    of "info":
      showInfo(remainingArgs(p))
    of "list":
      listPackages()
    of "search":
      searchPackages(remainingArgs(p))
    of "update":
      echo ""
      echo "update complete"
    of "--version", "version":
      echo version
    else:
      exit(help, QuitFailure)

    exit()
