#!/usr/bin/env bash

set -e

curl https://sh.rustup.rs -sSf | sh -s - --default-toolchain nightly -y
source ~/.cargo/env

cargo install wasm-pack

npm run build

ls -l dist/
