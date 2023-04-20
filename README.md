# Samudra OS

A prototype RISCV64GC operating system written in Rust.

## Dependencies

* A QEMU emulator (`qemu-system-riscv64`) in `$PATH`
* Rust bare-metal (`riscv64gc-unknown-none-elf`) toolchain
* C bare-metal (`riscv64-elf-gcc`) toolchain

## Quickstart

Run `./dev.sh`

## References

Thanks to:
* [The Adventures of OS: Making a RISC-V Operating System using Rust](https://osblog.stephenmarz.com/index.html)
* [maRVelOS](https://github.com/DonaldKellett/marvelos)
* [RISC-V from scratch 1: Introduction, toolchain setup, and hello world!](https://twilco.github.io/riscv-from-scratch/2019/03/10/riscv-from-scratch-1.html)
* [Xv6, a simple Unix-like teaching operating system](https://pdos.csail.mit.edu/6.828/2022/xv6.html)
