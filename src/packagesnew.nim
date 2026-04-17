# SPDX-License-Identifier: MPL-2.0

# Copyright © 2018-2026 Firas Khana

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import
  std/[
    algorithm, deques, times, os, osproc, posix, sequtils, sets, strformat, streams,
    strtabs, strutils, tables,
  ]
import utils

const pkgCache = "/var/cache/rad/pkg"
const srcCache = "/var/cache/rad/src"
const coreRepo = "/var/lib/rad/repo/core"
const radLog = "/var/log/rad"
const radTmp = "/var/tmp/rad"

type Keys = enum
  ver
  url
  sum
  bld
  run
  opt
  lic

type Package = object
  ver, url, sum, bld, run, opt, lic: string

type State = enum
  visiting
  visited

type Stage* = enum
  stageToolchain
  stageCross
  stageNative

type BuildOptions* = object
  jobs*: int
  makeJobs*: int
  keepGoing*: bool
  verbose*: bool
  stage*: Stage
  sysroot*: string
  cacheDir*: string
  srcDir*: string
  tmpDir*: string

type BuildStatus = enum
  pending
  running
  done
  failed

type BuildNode = object
  name: string
  status: BuildStatus
  deps: seq[string] # direct deps
  rdeps: seq[string] # reverse deps (packages that depend on this)
  indegree: int # number of deps not yet built
  exitCode: int
  output: string

proc cleanCache*(preserveSrc: bool = true) =
  if dirExists(radTmp):
    try:
      removeDir(radTmp)
    except OSError:
      raise newException(IOError, radTmp & ": unable to clean temporary directory")
  try:
    createDir(radTmp)
  except OSerror:
    raise newException(IOError, radTmp & ": unable to create temporary directory")

  if not preserveSrc:
    if dirExists(srcCache):
      try:
        removeDir(srcCache)
      except OSError:
        raise newException(IOError, srcCache & ": unable to clean source cache")
    try:
      createDir(srcCache)
    except:
      raise newException(IOError, srcCache & ": unable to create source cache")

proc parseInfo*(nom: string): Package =
  let info = coreRepo / nom / "info"

  var seen: set[Keys]
  var num = 0

  try:
    for line in lines(info):
      inc num

      let line = line.strip()
      if line.isEmptyOrWhitespace() or line.startsWith('#'):
        continue

      let pos = line.find('=')
      if pos == -1:
        raise newException(ValueError, &"{info}:{num}: missing '='")

      if pos > 0 and line[pos - 1] in Whitespace:
        raise
          newException(ValueError, &"{info}:{num}: whitespace before '=' not allowed")
      if pos < line.high() and line[pos + 1] in Whitespace:
        raise
          newException(ValueError, &"{info}:{num}: whitespace after '=' not allowed")

      let lhs = line[0 ..< pos].strip()
      if lhs.isEmptyOrWhitespace():
        raise newException(ValueError, &"{info}:{num}: missing key before '='")

      let key =
        try:
          parseEnum[Keys](lhs)
        except ValueError:
          raise newException(ValueError, &"{info}:{num}: invalid key '{lhs}'")

      if key in seen:
        raise newException(ValueError, &"{info}:{num}: duplicate key '{lhs}'")
      seen.incl(key)

      let rhs = line[pos + 1 .. ^1]

      let firstQuote = rhs.find('\'')
      if firstQuote == -1:
        raise newException(ValueError, &"{info}:{num}: missing single quote")

      let secondQuote = rhs.find('\'', firstQuote + 1)
      if secondQuote == -1:
        raise newException(ValueError, &"{info}:{num}: missing single quote")

      let val = rhs[firstQuote + 1 .. secondQuote - 1]

      let after = rhs[secondQuote + 1 .. ^1].strip()
      if after != "" and not after.startsWith('#'):
        raise newException(
          ValueError, &"{info}:{num}: trailing characters after value: {after}"
        )

      case key
      of ver:
        result.ver = val
      of url:
        result.url = val
      of sum:
        result.sum = val
      of bld:
        result.bld = val
      of run:
        result.run = val
      of opt:
        result.opt = val
      of lic:
        result.lic = val
  except IOError:
    raise newException(IOError, info & ": unable to parse")

# Topologically sorted DFS with cycle detection and package cache
proc resolveDeps(
    name: string,
    resolved: var seq[string],
    state: var Table[string, State],
    cache: var Table[string, Package],
) =
  if name in state:
    case state[name]
    of visiting:
      raise newException(ValueError, name & ": cyclic dependency detected")
    of visited:
      return

  state[name] = visiting

  if name notin cache:
    cache[name] = parseInfo(name)

  # Use cache (O(1) lookup) to avoid repeated calls to `parseInfo` (O(n))
  let pkg = cache[name]

  if not pkg.bld.isEmptyOrWhitespace():
    for dep in pkg.bld.splitWhitespace().deduplicate():
      resolveDeps(dep, resolved, state, cache)

  state[name] = visited
  resolved &= name

proc sortPackages*(packages: openArray[string]): seq[string] =
  var state: Table[string, State]
  var cache: Table[string, Package]

  for name in packages.deduplicate():
    resolveDeps(name, result, state, cache)

proc fetchPackages*(packages: openArray[string]) =
  for idx, nom in packages:
    let package = parseInfo(nom)
    # printContent(idx, nom, package.ver, "fetch")
    # echo &"""{idx + 1:<8}{nom:24}{ver:24}{cmd:8}""" & now().format("hh:mm tt")

    if package.url.isEmptyOrWhitespace():
      continue

    let src = srcCache / nom
    let tmp = radTmp / nom

    if package.sum.isEmptyOrWhitespace():
      if not dirExists(src):
        try:
          gitCloneRepo(package.url, src)
        except OSError:
          echo nom & ": unable to clone package"

      if package.ver != "latest":
        try:
          gitCheckoutRepo(src, package.ver)
        except OSError:
          echo nom & ": unable to checkout package"

      copyDirWithPermissions(src, tmp)
      continue

    let archive = src / lastPathPart(package.url)

    if not verifyFile(archive, package.sum):
      removeDir(src)
      createDir(src)

      try:
        downloadFile(package.url, src)
      except OSError:
        echo nom & ": unable to download package"

      if not verifyFile(archive, package.sum):
        echo nom & ": unable to verify checksum: " & package.sum
        quit(QuitFailure)

    createDir(tmp)

    try:
      extractTar(archive, tmp)
    except OSError:
      echo nom & ": unable to extract source tarball"

proc genContents*(dir, contents: string) =
  if not dirExists(dir):
    raise newException(IOError, dir & ": no such directory")

  var entries: seq[string]

  for entry in walkDirRec(
    dir, yieldFilter = {pcFile .. pcLinkToDir}, relative = true, skipSpecial = true
  ):
    let kind = getFileInfo(dir / entry, followSymlink = false).kind
    if kind in {pcDir, pcLinkToDir}:
      entries &= entry & '/'
    else:
      entries &= entry

  entries.sort()

  var f: File
  if not open(f, contents, fmWrite):
    raise newException(IOError, contents & ": unable to open file")

  try:
    for line in entries:
      f &= line & '\n'
  finally:
    f.close()

proc buildGraph(
    pkgs: seq[string], cache: var Table[string, Package]
): Table[string, BuildNode] =
  # create nodes
  for name in pkgs:
    result[name] = BuildNode(name: name, status: pending, deps: @[], rdeps: @[])

  # fill deps and dependents
  for name in pkgs:
    let pkg = cache.getOrDefault(name, parseInfo(name))
    let deps =
      if not pkg.bld.isEmptyOrWhitespace():
        pkg.bld.splitWhitespace().deduplicate()
      else:
        @[]
    result[name].deps = deps
    for dep in deps:
      if dep in result:
        result[dep].rdeps &= name

  # set indegree = number of deps
  for name, node in result.mpairs:
    node.indegree = node.deps.len()

proc runBuild(pkg: string, opts: BuildOptions) =
  let pkgInfo = parseInfo(pkg)
  let scriptDir = coreRepo / pkg
  let scriptFile =
    case opts.stage
    of stageNative:
      scriptDir / "build"
    of stageCross:
      scriptDir / "build-cross"
    of stageToolchain:
      scriptDir / "build-toolchain"

  if not fileExists(scriptFile):
    raise newException(IOError, scriptFile & ": no such file")

  # Source directory (already fetched)
  let srcDir = opts.srcDir / pkg
  if not dirExists(srcDir):
    raise newException(IOError, pkg & ": " & srcDir & ": no such directory")

  # Create a unique working directory
  let workDir =
    opts.tmpDir / "build-" & pkg & "-" & $getTime().toUnix() & "-" & $getpid()
  createDir(workDir)

  # Prepare environment
  var env = newStringTable()
  env["PATH"] = "/usr/bin"
  if opts.stage != stageNative:
    env["PATH"] = opts.sysroot / "bin" & ":" & env["PATH"]
  env["MAKEFLAGS"] =
    if "no-parallel" notin pkgInfo.opt:
      "-j " & $opts.makeJobs & " -O"
    else:
      "-j1"
  env["DESTDIR"] = workDir / "dest"
  env["CPPFLAGS"] = "-I" & opts.sysroot / "usr/include"
  env["CFLAGS"] = "-I" & opts.sysroot / "usr/include"
  env["LDFLAGS"] = "-L" & opts.sysroot / "usr/lib"
  env["PKG_CONFIG_PATH"] = opts.sysroot / "usr/lib/pkgconfig"

  if opts.stage != stageNative:
    env["CC"] = "clang"
    env["CXX"] = "clang++"

  env["RAD_SYSROOT"] = opts.sysroot
  env["RAD_STAGE"] = $opts.stage
  env["RAD_VERBOSE"] = $opts.verbose

  let cmd = &"sh -efu -c 'cd \"$1\"; . \"$2\"; for f in prepare configure build package; do command -v $f >/dev/null && $f; done' -- {quoteShell(srcDir)} {quoteShell(scriptFile)}"

  let (output, exitCode) = execCmdEx(
    cmd, env = env, workingDir = workDir, options = {poStdErrToStdOut, poUsePath}
  )
  if exitCode != 0:
    raise newException(OSError, output)

  let archiveDir = opts.cacheDir / pkg
  createDir(archiveDir)
  let archive = archiveDir / (pkg & "-" & pkgInfo.ver & ".tar.zst")
  createTar(archive, workDir / "dest")
  genContents(workDir / "dest", archiveDir / "contents")

  # removeDir(workDir)

  quit(QuitSuccess)

proc buildPackages*(packages: openArray[string], opts: BuildOptions) =
  let sorted = sortPackages(packages)

  # Parse all packages once
  var pkgCache: Table[string, Package]
  for name in sorted:
    pkgCache[name] = parseInfo(name)

  echo "Building graph"
  var graph = buildGraph(sorted, pkgCache)

  echo "Fetching now"
  # sequential (can be made parallel later)
  fetchPackages(packages)

  echo "Setting Deque"
  # Initial ready queue (indegree == 0)
  var ready = initDeque[string]()
  for name, node in graph:
    if node.indegree == 0:
      ready.addLast(name)

  echo "Tracking PID"
  # Track using PID as key
  var pidToProc: Table[int, Process] # pid -> Process object
  var pidToPkg: Table[int, string] # pid -> package name
  var pidToWorkDir: Table[int, string] # to clean up later
  var outputs: Table[string, string] # accumulate output per package

  echo "Main loop"
  while ready.len() > 0 or pidToProc.len() > 0:
    # Start new workers up to jobs limit
    while pidToProc.len() < opts.jobs and ready.len() > 0:
      let pkg = ready.popFirst
      graph[pkg].status = BuildStatus.running

        # Create unique working directory for this build
      let workDir =
        opts.tmpDir / "build-" & pkg & "-" & $getTime().toUnix() & "-" &
        $getCurrentProcessId()
      createDir(workDir)
      let destDir = workDir / "dest"
      createDir(destDir) # will be used as DESTDIR

      let srcDir = opts.srcDir / pkg
      let scriptDir = coreRepo / pkg
      let scriptFile =
        case opts.stage
        of stageNative:
          scriptDir / "build"
        of stageCross:
          scriptDir / "build-cross"
        of stageToolchain:
          scriptDir / "build-toolchain"

      if not fileExists(scriptFile):
        raise newException(IOError, scriptFile & ": no such file")
      if not dirExists(srcDir):
        raise newException(IOError, pkg & ": source not fetched: " & srcDir)

      # Build environment for the child shell
      var env = newStringTable()
      env["PATH"] = "/usr/bin"
      if opts.stage != stageNative:
        env["PATH"] = opts.sysroot / "bin" & ":" & env["PATH"]

      let pkgInfo = pkgCache[pkg]
      env["MAKEFLAGS"] =
        if "no-parallel" notin pkgInfo.opt:
          "-j " & $opts.makeJobs & " -O"
        else:
          "-j1"
      env["DESTDIR"] = destDir
      env["CPPFLAGS"] = "-I" & opts.sysroot / "usr/include"
      env["CFLAGS"] = "-I" & opts.sysroot / "usr/include"
      env["LDFLAGS"] = "-L" & opts.sysroot / "usr/lib"
      env["PKG_CONFIG_PATH"] = opts.sysroot / "usr/lib/pkgconfig"

      if opts.stage != stageNative:
        env["CC"] = "clang"
        env["CXX"] = "clang++"
        env["AR"] = "llvm-ar"
        env["NM"] = "llvm-nm"
        env["RANLIB"] = "llvm-ranlib"

      env["RAD_SYSROOT"] = opts.sysroot
      env["RAD_STAGE"] = $opts.stage
      env["RAD_VERBOSE"] = $opts.verbose

      let cmd = &"""
        cd {quoteShell(srcDir)} && \
        . {quoteShell(scriptFile)} && \
        for f in prepare configure build package; do \
          command -v $$f >/dev/null && $$f; \
        done
      """

      # Start the shell process
      let process = startProcess(
        "sh",
        args = ["-efu", "-c", cmd],
        env = env,
        workingDir = workDir,
        options = {poStdErrToStdOut, poUsePath},
      )
      let pid = process.processID
      pidToProc[pid] = process
      pidToPkg[pid] = pkg
      pidToWorkDir[pid] = workDir

    echo "Check for running processes"
    var finished: seq[int]
    for pid, process in pidToProc:
      echo "Peek at exit code"
      let status = process.peekExitCode()
      if status >= 0:
        # Process finished
        let pkg = pidToPkg[pid]
        let workDir = pidToWorkDir[pid]
        echo "Process finished - capturing output"
        let output = process.outputStream.readAll()
        outputs[pkg] = output
        graph[pkg].exitCode = status
        graph[pkg].output = output

        if status == 0:
          graph[pkg].status = BuildStatus.done
          echo "Extracting Tarball"
          let archiveDir = opts.cacheDir / pkg
          createDir(archiveDir)
          let archive = opts.cacheDir / pkg / pkg & "-" & pkgCache[pkg].ver & ".tar.zst"
          try:
            createTar(archive, workDir / "dest")
            genContents(workDir / "dest", archiveDir / "contents")
          except:
            # If archiving fails, treat as build failure
            graph[pkg].status = BuildStatus.failed
            stderr.write &"Failed to create archive for {pkg}\n"
            if not opts.keepGoing:
              # Terminate other processes and exit
              for otherPid, otherProc in pidToProc:
                if otherPid != pid:
                  otherProc.terminate()
              sleep(100)
              for otherPid, otherProc in pidToProc:
                if otherProc.running:
                  otherProc.kill()
              stderr.write output & "\n"
              quit(QuitFailure)
          # Extract into sysroot so dependents can find it
          # extractTar(archive, opts.sysroot)
          echo "Update dependents"
          for dep in graph[pkg].rdeps:
            graph[dep].indegree.dec
            if graph[dep].indegree == 0:
              ready.addLast(dep)
        else:
          # Build failed
          graph[pkg].status = BuildStatus.failed
          if not opts.keepGoing:
            stderr.write &"Build failed for {pkg}\n"
            if opts.verbose:
              stderr.write output & "\n"
            # Terminate all remaining workers
            for otherPid, otherProc in pidToProc:
              if otherPid != pid:
                otherProc.terminate()
            sleep(100)
            for otherPid, otherProc in pidToProc:
              if otherProc.running:
                otherProc.kill()
            quit(QuitFailure)

        # Cleanup
        process.close()
        finished.add(pid)
        removeDir(workDir)

    echo "Remove finished processes"

    # Remove finished processes from tracking tables
    for pid in finished:
      pidToProc.del(pid)
      pidToPkg.del(pid)
      pidToWorkDir.del(pid)

    sleep(50) # small delay to avoid busy‑waiting

  # Final report
  var failed: seq[string]
  for name, node in graph:
    if node.status == BuildStatus.failed:
      failed.add(name)
  if failed.len() > 0:
    echo "Failed packages: ", failed.join(" ")
    quit(QuitFailure)
  else:
    echo "All packages built successfully."

proc isEmpty*(dir: string): bool =
  if not dirExists(dir):
    raise newException(IOError, dir & ": no such directory")

  try:
    for entry in walkDir(dir):
      return false
  except OSError:
    raise newException(IOError, dir & ": unable to read directory: ")
  return true

proc showInfo*(packages: openArray[string]) =
  for nom in packages.deduplicate():
    let package = parseInfo(nom)
    echo &"""
name      :: {nom}
version   :: {package.ver}
source    :: {package.url}
checksum  :: {package.sum}
bld deps  :: {package.bld}
run deps  :: {package.run}
options   :: {package.opt}
"""

proc listPackages*() =
  if not dirExists(pkgCache):
    return

  var packages: seq[string]
  for (kind, package) in walkDir(pkgCache, relative = true, skipSpecial = true):
    if kind in {pcDir, pcLinkToDir}:
      packages &= package

  packages.sort()

  showInfo(packages)

proc listContents*(packages: openArray[string]) =
  for package in packages.deduplicate():
    let dir = pkgCache / package
    if not dirExists(dir):
      raise newException(IOError, dir & ": no such directory")

    try:
      for line in lines(dir / "contents"):
        echo &"/{line}"
    except IOError:
      echo package & ": unable to read contents"

proc searchPackages*(patterns: openArray[string]) =
  if not dirExists(coreRepo):
    raise newException(IOError, coreRepo & ": no such directory")

  var matches = initHashSet[string]()

  for (kind, name) in walkDir(coreRepo, relative = true, skipSpecial = true):
    if kind notin {pcDir, pcLinkToDir}:
      continue

    for pattern in patterns:
      if pattern.isEmptyOrWhitespace():
        continue
      if name.toLowerAscii().contains(pattern.strip().toLowerAscii()):
        matches.incl(name)
        break

  if matches.len() == 0:
    return

  showInfo(matches.toSeq().sorted())
