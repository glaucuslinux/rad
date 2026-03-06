# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2026 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import std/[parseopt, strformat, strutils, sequtils, parseutils]

const help = """
USAGE:
  rad [OPTIONS] COMMAND [ARGUMENTS]

OPTIONS:
  -h, --help         Show help message
  -n, --no-parallel  Disable parallel build
  -jN, --jobs=N      Specify the number of make jobs
  -v, --verbose      Enable verbose build
  -V, --version      Show rad version

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

var args: seq[string] = @[]
var verboseRequested = false
var noParallelRequested = false
var jobs = 0
var unknownFlags: seq[string] = @[]

for kind, key, val in getopt():
  case kind
  of cmdEnd:
    break
  of cmdLongOption, cmdShortOption:
    case key
    of "h", "help":
      quit(help, QuitSuccess)
    of "V", "version":
      quit(version, QuitSuccess)
    of "v", "verbose":
      verboseRequested = true
    of "n", "no-parallel", "noparallel":
      noParallelRequested = true
    of "j", "jobs":
      if val == "":
        quit("-j/--jobs requires a value (e.g., -j5 or --jobs=5)", QuitFailure)
      try:
        jobs = parseInt(val)
        if jobs <= 0:
          quit(&"invalid value: {jobs}", QuitFailure)
      except ValueError:
        quit(&"invalid value: {jobs}", QuitFailure)
    else:
      unknownFlags.add(key)
  of cmdArgument:
    args.add(key)

if unknownFlags.len() > 0:
  let msg =
    if unknownFlags.len == 1:
      "invalid option: " & unknownFlags[0]
    else:
      "invalid options: " & unknownFlags.join(", ")
  quit(msg, QuitFailure)

if verboseRequested:
  echo "do something"
if noParallelRequested:
  echo "do something else"

# No arguments?
if args.len == 0:
  quit(help, QuitFailure)

# First argument is the subcommand
let cmd = args[0]
let cmdArgs = args[1..^1]

case cmd
of "build":
  # lock()
  # cleanCache()
  # buildPackages(remainingArgs(cmdArgs))
  echo ""
  echo "build complete"
of "clean":
  # lock()
  # cleanCache()
  echo "clean complete"
of "contents":
  # listContents(remainingArgs(cmdArgs))
  echo "placeholder"
of "help":
  echo help
of "info":
  # showInfo(remainingArgs(cmdArgs))
  echo "placeholder"
of "list":
  # listPackages()
  echo "placeholder"
of "search":
  # searchPackages(remainingArgs(cmdArgs))
  echo "placeholder"
of "update":
  # lock()
  echo ""
  echo "update complete"
of "version":
  echo version
else:
  quit(&"invalid command: {cmd}", QuitFailure)
