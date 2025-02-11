# Copyright (c) 2018-2025, Firas Khalil Khana
# Distributed under the terms of the ISC License

import std/[os, parseopt], arch, bootstrap, cerata, constants, tools

proc options*() =
  if paramCount() < 1:
    quit($Rad, QuitFailure)

  var p = initOptParser()

  p.next()

  case p.kind
  of cmdArgument, cmdEnd:
    quit($Rad, QuitFailure)
  of cmdLongOption, cmdShortOption:
    lock()

    case p.key
    of "b", "bootstrap":
      p.next()

      case p.kind
      of cmdEnd:
        exit($Bootstrap, QuitFailure)
      of cmdArgument, cmdLongOption, cmdShortOption:
        case p.key
        of "c", "clean":
          setEnvBootstrap()
          cleanBootstrap()

          echo "clean complete"
        of "d", "distclean":
          setEnvBootstrap()
          distcleanBootstrap()

          echo "distclean complete"
        of "h", "help":
          echo Bootstrap
        of "n", "native":
          setEnvArch()
          setEnvNativeDirs()
          setEnvNativeTools()
          buildCerata(Native, $native, false)

          echo ""
          echo "native complete"
        of "t", "toolchain":
          require()
          setEnvArch(toolchain)
          setEnvBootstrap()
          cleanBootstrap()
          prepareBootstrap()
          buildCerata(Toolchain, $toolchain, false)

          echo ""
          echo "toolchain complete"
        of "x", "cross":
          setEnvArch(cross)
          setEnvBootstrap()
          setEnvCrossTools()
          prepareCross()
          buildCerata(Cross, $cross, false)

          echo ""
          echo "cross complete"
        else:
          exit($Bootstrap, QuitFailure)
    of "c", "cerata":
      p.next()

      case p.kind
      of cmdEnd:
        exit($Cerata, QuitFailure)
      of cmdArgument, cmdLongOption, cmdShortOption:
        case p.key
        of "b", "build":
          setEnvArch()
          setEnvNativeDirs()
          setEnvNativeTools()
          cleanCerata()
          buildCerata(remainingArgs(p))

          echo ""
          echo "build complete"
        of "c", "clean":
          cleanCerata()

          echo "clean complete"
        of "d", "distclean":
          distcleanCerata()

          echo "distclean complete"
        of "h", "help":
          echo Cerata
        of "i", "install":
          installCerata(remainingArgs(p))

          echo ""
          echo "install complete"
        of "l", "list":
          listCerata()
        of "p", "print":
          printCerata(remainingArgs(p))
        of "r", "remove":
          removeCerata(remainingArgs(p))

          echo ""
          echo "remove complete"
        of "s", "search":
          searchCerata(remainingArgs(p))
        of "u", "upgrade":
          echo ""
          echo "upgrade complete"
        else:
          exit($Cerata, QuitFailure)
    of "h", "help":
      echo Rad
    of "v", "version":
      echo version
    else:
      exit($Rad, QuitFailure)

    exit()
