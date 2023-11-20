#!/bin/bash
export WASI_VERSION=20
export WASI_VERSION_FULL=${WASI_VERSION}.0

file=wasi-sdk-${WASI_VERSION_FULL}-linux.tar.gz
if [ -f "$file" ] ; then
    rm "$file"
fi

wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-${WASI_VERSION}/wasi-sdk-${WASI_VERSION_FULL}-linux.tar.gz
tar xvf "$file"
