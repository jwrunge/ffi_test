#!/bin/bash

rustup target add wasm32-wasi
rustc main.rs --target wasm32-wasi
