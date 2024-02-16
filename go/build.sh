#!/bin/bash

#GOOS=wasip1 GOARCH=wasm go build -o main.wasm
# tinygo build -target=wasi -o main.wasm main.go

go build -buildmode=c-archive -o hellogo.lib main.go