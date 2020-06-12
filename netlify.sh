#!/usr/bin/env bash

set -e


if [ -f prebuild/index.html ]; then
   mkdir dist
   cp prebuild/* dist/
fi
if [ ! -f prebuild/index.html]; then

   shopt -s dotglob

   mkdir target

   if [ -d .cache ]; then
      mv .cache/* target/
   fi



   curl https://sh.rustup.rs -sSf | sh -s - --default-toolchain stable -y
   source ~/.cargo/env

   cargo install wasm-pack

   npm run build

   ls -l dist/

   if [ ! -d .cache ]; then
      mkdir .cache
   fi
   mv target/* .cache/
fi