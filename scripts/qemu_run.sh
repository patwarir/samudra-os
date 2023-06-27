#!/bin/sh

# The name of the QEMU RV64GC executable
QEMU="qemu-system-riscv64"

# 4 Cores/HARTs
# 512M RAM
# Redirect IO to QEMU StdIO
$QEMU -machine virt -cpu rv64,zba=true,zbb=true,zbc=true,zbs=true -smp 4 -m 512M -nographic -serial mon:stdio -bios none -kernel $1 -d guest_errors,unimp
