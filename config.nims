switch("d", "lto")
switch("d", "release")
switch("d", "useMalloc")
switch("d", "strip")
switch("os", "linux")
switch("cpu", "amd64")
switch(
  "t",
  "-pipe -g0 -O2 -fdevirtualize-at-ltrans -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -flto=auto -flto-compression-level=19 -fuse-linker-plugin -fstack-protector-strong -fstack-clash-protection -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -march=x86-64-v3 -mfpmath=sse -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2",
)
switch(
  "l",
  "-Wl,-O1,-s,-z,noexecstack,-z,now,-z,pack-relative-relocs,-z,relro,-z,x86-64-v3,--as-needed,--gc-sections,--sort-common,--hash-style=gnu,--compress-debug-sections=zstd -pipe -Wno-stringop-overflow -g0 -O2 -fdevirtualize-at-ltrans -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -flto=auto -flto-compression-level=19 -fuse-linker-plugin -fstack-protector-strong -fstack-clash-protection -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -march=x86-64-v3 -mfpmath=sse -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2",
)
