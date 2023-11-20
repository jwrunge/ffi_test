# Notes on FFI and WASM/WASI

Great resource compiling C and Rust to WASI: [WASI-tutorial](https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-tutorial.md)

* Goes over details of file access sandboxing and how to give file access in wasmtime

## Get clang compiler that targets WASI

```
cd tools

#if windows
wsl
#/if

./get-wasi-sdk.sh
```

Then, in the folder you want to build in:
```
cd c #assuming you're compiling c/main.c to wasm

#if windows
wsl
#/if

./build.sh      //Builds with wasm-custom clang
```
