version = "0.1.0"
author = "Firas Khana <firasuke@glaucuslinux.org>"
description = "glaucus package manager"
license = "MPL-2.0"

skipFiles = @["LICENSE", "README.md"]
srcDir = "src"

bin = @["rad"]

requires "nim >= 2.3.1"
requires "tomlSerialization >= 0.2.14"
