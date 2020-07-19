#!/usr/bin/env bash

set -e



   shopt -s dotglob

   mkdir target

   if [ -d .cache ]; then
      mv .cache/* target/
   fi



   curl https://sh.rustup.rs -sSf | sh -s - --default-toolchain stable -y
   source ~/.cargo/env

   curl -L https://github.com/rustwasm/wasm-pack/releases/download/v0.9.1/wasm-pack-v0.9.1-x86_64-unknown-linux-musl.tar.gz --output wasm-pack-v0.9.1-x86_64-unknown-linux-musl.tar.gz
   ls -l
   tar -zxvf wasm-pack-v0.9.1-x86_64-unknown-linux-musl.tar.gz
   export PATH="$PATH:$PWD/wasm-pack-v0.9.1-x86_64-unknown-linux-musl"

   npm run build

   rm dist/*.map

   ls -l dist/

   if [ ! -d .cache ]; then
      mkdir .cache
   fi
   mv target/* .cache/
