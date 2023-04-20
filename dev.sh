#!/bin/sh

cd ./src/

make clean
TARGET=debug make

cd ../

./scripts/qemu_run.sh ./out/samudra-os.elf
