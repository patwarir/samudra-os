#!/bin/sh

set -eu

cd ./wasm/example-hello-world/
cargo build --release

cd ../../

TARGET=debug make

./scripts/qemu.sh ./out/samudra-os.elf
