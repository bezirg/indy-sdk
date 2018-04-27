
TOOLCHAIN=$HOME/x86_64_api_26_toolchain

export PATH=$TOOLCHAIN/bin:$PATH
export PKG_CONFIG_ALLOW_CROSS=1
export X86_64_LINUX_ANDROID_OPENSSL_DIR="/usr/local/ssl"
export CC="x86_64-linux-android-clang" 
export LDFLAGS="-L ../libindy/target/x86_64-linux-android/debug"
echo "cargo build" $*
cargo build $*


