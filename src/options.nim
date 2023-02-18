# Copyright (c) 2018-2023, Firas Khalil Khana
# Distributed under the terms of the ISC License

from os import paramCount
import parseopt

import ceras
import constants

proc radula_options*() =
    if paramCount() < 1:
        echo RADULA_HELP
        quit(1)

    var
        p = initOptParser()
    for kind, key, value in getOpt():
        p.next()
        case kind
        of cmdArgument, cmdEnd:
            echo RADULA_HELP
            quit(1)
        of cmdLongOption, cmdShortOption:
            case p.key
            of "b", "behave":
                case value
                of "b", "bootstrap":
                    p.next()
                    case p.key
                    of "h", "help":
                        echo RADULA_HELP_BEHAVE_BOOTSTRAP
                    of "l", "list":
                        echo RADULA_HELP_BEHAVE_BOOTSTRAP_LIST
                    of "r", "require":
                        echo "Checking if host has all required packages..."
                    else:
                        echo RADULA_HELP_BEHAVE_BOOTSTRAP
                        quit(1)
                    quit(0)
                else:
                    echo RADULA_HELP_BEHAVE
                    quit(1)
            of "c", "ceras":
                radula_behave_ceras_print(remainingArgs(p))
                quit(0)
            of "h", "help":
                echo RADULA_HELP
                quit(0)
            of "v", "version":
                echo RADULA_HELP_VERSION
                quit(0)
            else:
                echo RADULA_HELP
                quit(1)
