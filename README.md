# Samudra OS

A prototype RISC-V operating system written in Rust that supports kernel Wasm modules.

## Specifications

Supports:

* RISC-V (ISA: `riscv64gcv_zba_zbb_zbc_zbs_zfh`, ABI: `lp64d`)
* Wasm (ISA: `wasm32_bulk-memory_multivalue_mutable-globals_nontrapping-fptoint_reference-types_sign-ext`)

## Dependencies

* QEMU
* Spike
* Rust bare-metal toolchain
* C bare-metal toolchain
* Wasm toolchain

## Quickstart

Run `SYSROOT={{your C bare-metal toolchain sysroot}} ./scripts/dev.sh`

## References

Thanks to:

* [The Adventures of OS: Making a RISC-V Operating System using Rust](https://osblog.stephenmarz.com/index.html)
* [maRVelOS](https://github.com/DonaldKellett/marvelos)
* [RISC-V from scratch 1: Introduction, toolchain setup, and hello world!](https://twilco.github.io/riscv-from-scratch/2019/03/10/riscv-from-scratch-1.html)
* [RISC-v Bytes](https://danielmangum.com/categories/risc-v-bytes/)
* [Serial Programming/8250 UART Programming](https://en.wikibooks.org/wiki/Serial_Programming/8250_UART_Programming)
* [Xv6, a simple Unix-like teaching operating system](https://pdos.csail.mit.edu/6.828/2022/xv6.html)
* [os-tutorial](https://github.com/cfenollosa/os-tutorial)
* [Writing an OS in Rust ](https://github.com/phil-opp/blog_os)
