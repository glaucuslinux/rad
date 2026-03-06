# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2026 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/[os, parseopt, strformat, strutils]

const help = """
USAGE:
  rad [OPTIONS] COMMAND [ARGUMENTS]

OPTIONS:
  -h, --help     Show help message
  -V, --version  Show rad version

COMMANDS:
  build          Build packages
  clean          Clean build cache
  contents       List package contents
  info           Show package information
  list           List built packages
  search         Search for packages
  update         Update repositories"""

const version = """
rad version 0.1.0

Licensed under the Mozilla Public License Version 2.0 (MPL-2.0)
Copyright © 2018-2026 Firas Khana"""

var p = initOptParser()
p.next()

case p.kind
of cmdEnd:
  quit(help, QuitFailure)
of cmdLongOption, cmdShortOption:
  case p.key
  of "h", "help":
    quit(help, QuitSuccess)
  of "V", "version":
    quit(version, QuitSuccess)
  else:
    quit(&"invalid option: {p.key}", QuitFailure)
of cmdArgument:
  case p.key
  of "build":
    # lock()
    # cleanCache()
    # buildPackages(remainingArgs(p))
    echo ""
    echo "build complete"
  of "clean":
    # lock()
    # cleanCache()
    echo "clean complete"
  of "contents":
    # listContents(remainingArgs(p))
    echo "placeholder"
  of "help":
    echo help
  of "info":
    # showInfo(remainingArgs(p))
    echo "placeholder"
  of "list":
    # listPackages()
    echo "placeholder"
  of "search":
    # searchPackages(remainingArgs(p))
    echo "placeholder"
  of "update":
    # lock()
    echo ""
    echo "update complete"
  of "version":
    echo version
  else:
    quit(&"invalid command: {p.key}", QuitFailure)
