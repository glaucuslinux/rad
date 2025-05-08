# Copyright (c) 2018-2025, Firas Khalil Khana
# Distributed under the terms of the ISC License

import std/[os, parseopt], arch, bootstrap, cerata, constants, tools

proc options*() =
  if paramCount() < 1:
    quit($Rad, QuitFailure)

  var p = initOptParser()

  p.next()

  case p.kind
  of cmdEnd:
    quit($Rad, QuitFailure)
  else:
    lock()

    case p.key
    of "bootstrap":
      p.next()

      case p.kind
      of cmdEnd:
        exit($Bootstrap, QuitFailure)
      else:
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
          buildCerata(Cross, resolve = false, stage = $cross)

          echo ""
          echo "cross complete"
        of "distclean":
          setEnvBootstrap()
          distcleanBootstrap()

          echo "distclean complete"
        of "native":
          setEnvArch()
          setEnvNativeDirs()
          setEnvNativeTools()
          buildCerata(Native, resolve = false)

          echo ""
          echo "native complete"
        of "toolchain":
          require()
          setEnvArch(toolchain)
          setEnvBootstrap()
          cleanBootstrap()
          prepareBootstrap()
          buildCerata(Toolchain, resolve = false, stage = $toolchain)

          echo ""
          echo "toolchain complete"
        else:
          exit($Bootstrap, QuitFailure)
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
    of "contents":
      listContents(remainingArgs(p))
    of "distclean":
      distcleanCerata()

      echo "distclean complete"
    of "--help", "help":
      echo Rad
    of "info":
      showInfo(remainingArgs(p))
    of "install":
      setEnvArch()
      setEnvNativeDirs()
      setEnvNativeTools()
      cleanCerata()
      installCerata(remainingArgs(p))

      echo ""
      echo "install complete"
    of "list":
      listCerata()
    of "orphan":
      listOrphans()
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
    of "--version", "version":
      echo version
    else:
      exit($Rad, QuitFailure)

    exit()
