#!/usr/bin/env bash

set -e

shopt -s dotglob

mkdir -p node_modules/rustcache
mkdir target

mv node_modules/rustcache/* target


curl https://sh.rustup.rs -sSf | sh -s - --default-toolchain stable -y
source ~/.cargo/env

cargo install wasm-pack

npm run build

ls -l dist/

mkdir -p node_modules/rustcache
mv target/* node_modules/rustcache