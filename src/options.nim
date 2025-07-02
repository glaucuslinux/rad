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
  build      Build packages
  clean      Clean cache
  contents   List package contents
  help       Display this help message
  info       Show package information
  install    Install packages
  list       List installed packages
  orphan     List orphaned packages
  remove     Remove packages
  search     Search for packages
  update     Update repositories
  upgrade    Upgrade packages
  version    Display rad version"""
    helpBootstrap =
      """
USAGE:
  rad bootstrap [ COMMAND ]

COMMANDS:
  clean      Clean cache
  cross      Bootstrap cross glaucus (stage 2)
  native     Bootstrap native glaucus (stage 3)
  toolchain  Bootstrap toolchain (stage 1)"""
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
        of "cross":
          setEnvArch()
          setEnvBootstrap()
          setEnvCross()
          prepareCross()

          buildPackages(
            parsePackage("cross").run.split(), resolve = false, stage = $cross
          )

          echo ""
          echo "cross complete"
        of "native":
          setEnvArch()
          setEnvNative()

          buildPackages(parsePackage("native").run.split(), resolve = false)

          echo ""
          echo "native complete"
        of "toolchain":
          require()
          setEnvArch()
          setEnvBootstrap()
          cleanBootstrap()
          prepareBootstrap()

          buildPackages(
            parsePackage("toolchain").run.split(), resolve = false, stage = $toolchain
          )

          echo ""
          echo "toolchain complete"
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
    of "install":
      setEnvArch()
      setEnvNative()
      cleanPackages()
      installPackages(remainingArgs(p))

      echo ""
      echo "install complete"
    of "list":
      listPackages()
    of "orphan":
      listOrphans()
    of "remove":
      removePackages(remainingArgs(p))

      echo ""
      echo "remove complete"
    of "search":
      searchPackages(remainingArgs(p))
    of "update":
      echo ""
      echo "update complete"
    of "upgrade":
      echo ""
      echo "upgrade complete"
    of "--version", "version":
      echo version
    else:
      exit(help, QuitFailure)

    exit()
