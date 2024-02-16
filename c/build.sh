#!/bin/bash

# Build C library
gcc -c main.c               # Create intermediate object file
ar rcs helloc.lib main.o     # Create static library -- Rust looks for *.lib

# ../tools/wasi-sdk-20.0/bin/clang main.c -o main.wasm
