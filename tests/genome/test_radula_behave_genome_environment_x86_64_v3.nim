# Copyright (c) 2018-2023, Firas Khalil Khana
# Distributed under the terms of the ISC License

import std/[
  os,
  strutils
]

import
  ../../src/constants,
  ../../src/genome

radula_behave_genome_environment(RADULA_GENOME_X86_64_V3)

echo "ARCH    :: ", getEnv(RADULA_ENVIRONMENT_GENOME)
echo "CARCH   :: ", getEnv(RADULA_ENVIRONMENT_GENOME_CERATA)
echo "GCARCH  :: ", getEnv(RADULA_ENVIRONMENT_GENOME_GCC_CONFIGURATION)
echo "LARCH   :: ", getEnv(RADULA_ENVIRONMENT_GENOME_LINUX)
echo "LCARCH  :: ", getEnv(RADULA_ENVIRONMENT_GENOME_LINUX_CONFIGURATION)
echo "LIARCH  :: ", getEnv(RADULA_ENVIRONMENT_GENOME_LINUX_IMAGE)
echo "MARCH   :: ", getEnv(RADULA_ENVIRONMENT_GENOME_MUSL)
echo "MLARCH  :: ", getEnv(RADULA_ENVIRONMENT_GENOME_MUSL_LINKER)

echo ""

echo "BLD     :: ", getEnv(RADULA_ENVIRONMENT_TUPLE_BUILD)
echo "TGT     :: ", getEnv(RADULA_ENVIRONMENT_TUPLE_TARGET)

doAssert getEnv(RADULA_ENVIRONMENT_GENOME) == "x86-64"
doAssert getEnv(RADULA_ENVIRONMENT_GENOME_CERATA) == "--with-gcc-arch=x86-64"
doAssert getEnv(RADULA_ENVIRONMENT_GENOME_GCC_CONFIGURATION) == "--with-arch=x86-64-v3 --with-tune=generic"
doAssert getEnv(RADULA_ENVIRONMENT_GENOME_LINUX) == "x86_64"
doAssert getEnv(RADULA_ENVIRONMENT_GENOME_LINUX_CONFIGURATION) == "x86_64_"
doAssert getEnv(RADULA_ENVIRONMENT_GENOME_LINUX_IMAGE) == "arch/x86/boot/bzImage"
doAssert getEnv(RADULA_ENVIRONMENT_GENOME_MUSL) == "x86_64"
doAssert getEnv(RADULA_ENVIRONMENT_GENOME_MUSL_LINKER) == "x86_64"

doAssert getEnv(RADULA_ENVIRONMENT_TUPLE_BUILD) == radula_behave_genome_tuple()[0].strip()
doAssert getEnv(RADULA_ENVIRONMENT_TUPLE_TARGET) == "x86_64-glaucus-linux-musl"
