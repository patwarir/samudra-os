#!/bin/bash

set -euxo pipefail

OBJDUMP="${OBJDUMP:-objdump}"
READELF="${READELF:-readelf}"

KERNEL_PATH="${KERNEL_PATH:-./target/riscv64gc-unknown-none-elf/release/samudra-kernel}"
OUTPUT_DIR="${OUTPUT_DIR:-./tmp}"

$OBJDUMP -drwC $KERNEL_PATH > $OUTPUT_DIR/objdump.txt
$READELF -a $KERNEL_PATH > $OUTPUT_DIR/readelf.txt
