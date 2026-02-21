switch("d", "release")
switch("d", "useMalloc")
switch("d", "lto")
switch("d", "strip")
switch("os", "linux")
switch("cpu", "amd64")
switch(
  "passC",
  "-pipe -Os -fgcse-las -flto=auto -fuse-linker-plugin -ffunction-sections -fdata-sections -fstack-protector-strong -fstack-clash-protection -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-ident -fno-plt -march=x86-64-v3 -mtls-dialect=gnu2",
)
switch(
  "passL",
  "-Wl,-O1,-s,-z,defs,-z,noexecstack,-z,now,-z,pack-relative-relocs,-z,relro,-z,separate-code,-z,text,--as-needed,--gc-sections,--no-keep-memory,--relax,--sort-common,--enable-new-dtags,--hash-style=gnu,--build-id=none -Wno-stringop-overflow -pipe -Os -fgcse-las -flto=auto -fuse-linker-plugin -ffunction-sections -fdata-sections -fstack-protector-strong -fstack-clash-protection -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-ident -fno-plt -march=x86-64-v3 -mtls-dialect=gnu2",
)
