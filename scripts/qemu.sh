#!/bin/sh

set -eu

QEMU="${QEMU:-qemu-system-riscv64}"

# Set the ISA, cores/HARTs and RAM
# Redirect IO to QEMU
$QEMU -machine virt -cpu rv64,g=true,c=true,v=true,zba=true,zbb=true,zbc=true,zbs=true,Zfh=true -smp 4 -m 8192M -d guest_errors,unimp,int -nographic -serial mon:stdio -bios none -kernel $1
