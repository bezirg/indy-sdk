#!/bin/bash

toolchain_dir=${1-$HOME/x86_64_api_26_toolchain}
target_host=${2-x86_64-linux-android}

export PATH=$toolchain_dir/bin:$PATH

# Tell configure what tools to use.
export AR=$target_host-ar
export AS=$target_host-clang
export CC=$target_host-clang
export CXX=$target_host-clang++
export LD=$target_host-ld
export STRIP=$target_host-strip

# Tell configure what flags Android requires.
export CFLAGS="-fPIE -fPIC" 
export LDFLAGS="-pie"


