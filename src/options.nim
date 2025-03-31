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
        of "clean":
          setEnvBootstrap()
          cleanBootstrap()

          echo "clean complete"
        of "cross":
          setEnvArch(cross)
          setEnvBootstrap()
          setEnvCrossTools()
          prepareCross()
          buildCerata(Cross, $cross, false)

          echo ""
          echo "cross complete"
        of "distclean":
          setEnvBootstrap()
          distcleanBootstrap()

          echo "distclean complete"
        of "help":
          echo Bootstrap
        of "native":
          setEnvArch()
          setEnvNativeDirs()
          setEnvNativeTools()
          buildCerata(Native, $native, false)

          echo ""
          echo "native complete"
        of "toolchain":
          require()
          setEnvArch(toolchain)
          setEnvBootstrap()
          cleanBootstrap()
          prepareBootstrap()
          buildCerata(Toolchain, $toolchain, false)

          echo ""
          echo "toolchain complete"
        else:
          exit($Bootstrap, QuitFailure)
    of "c", "cerata":
      p.next()

      case p.kind
      of cmdEnd:
        exit($Cerata, QuitFailure)
      of cmdArgument, cmdLongOption, cmdShortOption:
        case p.key
        of "build":
          setEnvArch()
          setEnvNativeDirs()
          setEnvNativeTools()
          cleanCerata()
          buildCerata(remainingArgs(p))

          echo ""
          echo "build complete"
        of "clean":
          cleanCerata()

          echo "clean complete"
        of "distclean":
          distcleanCerata()

          echo "distclean complete"
        of "help":
          echo Cerata
        of "info":
          showInfo(remainingArgs(p))
        of "install":
          installCerata(remainingArgs(p))

          echo ""
          echo "install complete"
        of "list":
          listCerata()
        of "remove":
          removeCerata(remainingArgs(p))

          echo ""
          echo "remove complete"
        of "search":
          searchCerata(remainingArgs(p))
        of "update":
          echo ""
          echo "update complete"
        of "upgrade":
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
