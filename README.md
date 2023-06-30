# Samudra OS

A prototype RISC-V operating system written in Rust.

## Specifications

Supports:

* RISC-V (`riscv64imafdc_zifencei_zicsr_zba_zbb_zbc_zbs`)

## Dependencies

* QEMU (`qemu-system-riscv64`)
* Rust bare-metal toolchain (`riscv64gc-unknown-none-elf`)
* C bare-metal toolchain (`riscv64-elf-gcc`)

## Quickstart

Run `./dev.sh`

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
