#!/bin/bash

if [[ $1 = "dev" ]]; then
    wasm-pack build --target web --dev --out-dir dist --no-pack --no-opt
elif [[ $1 = "pkg" ]]; then
    wasm-pack build --no-pack --target web     --release --out-dir pkg/web
    wasm-pack build --no-pack --target bundler --release --out-dir pkg/bundler
    wasm-pack build --no-pack --target nodejs  --release --out-dir pkg/node
    cp ./package.json ./pkg/package.json
    cp ./readme.md ./pkg/readme.md
fi
