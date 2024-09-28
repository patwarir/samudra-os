#!/bin/bash

set -euxo pipefail

QEMU="${QEMU:-qemu-system-riscv64}"
KERNEL="${KERNEL:-./target/riscv64gc-unknown-none-elf/release/samudra-kernel}"

$QEMU -machine virt -cpu rv64,g=true,c=true,v=true,zba=true,zbb=true,zbs=true -smp cpus=4 -m 1024M -d guest_errors,unimp,int -nographic -serial mon:stdio -bios none -kernel $KERNEL
