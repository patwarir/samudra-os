#!/bin/sh

set -eu

TARGET=debug make

./scripts/qemu.sh ./out/samudra-os.elf
