#!/bin/sh

set -eu

QEMU="${QEMU:-qemu-system-riscv64}"

$QEMU -machine virt -cpu rv64,g=true,c=true,v=true,zba=true,zbb=true,zbs=true -smp cpus=4 -m 1024M -d guest_errors,unimp,int -nographic -serial mon:stdio -bios none -kernel $1
