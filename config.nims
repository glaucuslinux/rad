switch("d", "release")
switch("d", "useMalloc")
switch("d", "lto")
switch("d", "strip")
switch("os", "linux")
switch("cpu", "amd64")
switch(
  "passC",
  "-Os -foptimize-strlen -fgcse-las -flive-range-shrinkage -flto=auto -flto-compression-level=3 -fuse-linker-plugin -ffunction-sections -fdata-sections -fstack-protector-strong -fstack-clash-protection -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-ident -fno-plt -march=x86-64-v3 -malign-data=abi -mtls-dialect=gnu2",
)
switch(
  "passL",
  "-Wl,-O1,-s,-z,noexecstack,-z,now,-z,pack-relative-relocs,-z,relro,-z,separate-code,-z,start-stop-gc,-z,x86-64-v3,--as-needed,--gc-sections,--no-keep-memory,--relax,--sort-common,--enable-new-dtags,--hash-style=gnu,--reduce-memory-overheads,--build-id=none -Wno-stringop-overflow -Os -foptimize-strlen -fgcse-las -flive-range-shrinkage -flto=auto -flto-compression-level=3 -fuse-linker-plugin -ffunction-sections -fdata-sections -fstack-protector-strong -fstack-clash-protection -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-ident -fno-plt -march=x86-64-v3 -malign-data=abi -mtls-dialect=gnu2",
)
