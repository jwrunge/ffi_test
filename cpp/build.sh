#!/bin/bash

# Build C library
g++ -c main.cpp                # Create intermediate object file
ar rcs hellocpp.lib main.o     # Create static library -- Rust looks for *.lib

# ../tools/wasi-sdk-20.0/bin/clang main.c -o main.wasm
