#!/bin/sh

set -eu

SPIKE="${SPIKE:-spike}"

# Set the ISA, cores/HARTs and RAM
$SPIKE -p4 -m8192 -d --isa=rv64gcv_zba_zbb_zbc_zbs_zfh $1
