switch("d", "danger")
switch("d", "lto")
switch("d", "release")
switch("d", "useMalloc")
switch("d", "strip")
switch("threads", "on")
switch("opt", "speed")
switch("os", "linux")
switch("cpu", "amd64")
switch("t", "-pipe -g0 -O2 -fdevirtualize-at-ltrans -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -flto=auto -flto-compression-level=19 -fuse-linker-plugin -fstack-protector-strong -fstack-clash-protection -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -march=x86-64-v3 -mfpmath=sse -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2")
switch("l", "-fuse-ld=mold -Wl,-s,--as-needed,--compress-debug-sections=zstd,--gc-sections,--hash-style=gnu,-z,now,-z,noexecstack,-z,relro -pipe -Wno-stringop-overflow -g0 -O2 -fdevirtualize-at-ltrans -fsched-pressure -fno-semantic-interposition -fipa-pta -fgraphite-identity -floop-nest-optimize -flto=auto -flto-compression-level=19 -fuse-linker-plugin -fstack-protector-strong -fstack-clash-protection -fuse-ld=mold -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-plt -march=x86-64-v3 -mfpmath=sse -mabi=sysv -malign-data=cacheline -mtls-dialect=gnu2")
switch("mm", "orc")
