# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2025 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/[os, parseopt, strutils]
import bootstrap, packages, utils

proc options*() =
  const help =
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

  const helpBootstrap =
    """
USAGE:
  rad bootstrap [ COMMAND ]

COMMANDS:
  clean      Clean bootstrap cache
  help       Show help message
  1, stage1  Bootstrap stage 1 (toolchain)
  2, stage2  Bootstrap stage 2 (cross)
  3, stage3  Bootstrap stage 3 (native)"""

  const version =
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
        of "help", "--help":
          echo helpBootstrap
        of "1", "stage1", "toolchain":
          bootstrapToolchain()

          echo ""
          echo "stage 1 (toolchain) complete"
        of "2", "stage2", "cross":
          bootstrapCross()

          echo ""
          echo "stage 2 (cross) complete"
        of "3", "stage3", "native":
          buildPackages(parseInfo("native").run.split(), true)

          echo ""
          echo "stage 3 (native) complete"
        else:
          exit(helpBootstrap, QuitFailure)
    of "build":
      cleanCache()
      buildPackages(remainingArgs(p))

      echo ""
      echo "build complete"
    of "clean":
      cleanCache()

      echo "clean complete"
    of "contents":
      listContents(remainingArgs(p))
    of "help", "--help":
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
    of "version", "--version":
      echo version
    else:
      exit(help, QuitFailure)

    exit()
