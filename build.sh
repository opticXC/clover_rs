#! /usr/bin/bash

export OPENSSL_STATIC=yes
export OPENSSL_LIB_DIR=/usr/lib64
export OPENSSL_INCLUDE_DIR=/usr/include

export RUSTFLAGS="-C target-feature=+crt-static"

cargo build --target x86_64-unknown-linux-gnu $1
