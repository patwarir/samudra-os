# Name of the bare-metal RV64GC C compiler
CC=riscv64-elf-gcc

# Architecture: RV64GC, ABI: Hard float (lp64d)
CFLAGS=-march=rv64gc -mabi=lp64d -mcmodel=medany
# Static, no standard library
CFLAGS+=-static -ffreestanding -nostartfiles -nodefaultlibs -nostdlib -fno-exceptions
CFLAGS+=-Wall -Wextra -Wpedantic -std=c17

# CXXFLAGS=$(CFLAGS) -fno-rtti -std=c++17

ifeq ($(TARGET),debug)
	CFLAGS+=-O0 -g
else
	CFLAGS+=-O3
endif

INCLUDES=$(wildcard include/*.h)
CFLAGS_INCLUDE_PATH=-Iinclude

CFLAGS_LINKER_SCRIPT=-Tlds/virt.lds

SOURCES_ASM=$(wildcard asm/*.S)
SOURCES_C=$(wildcard c/*.c)

ifeq ($(TARGET),debug)
	SOURCES_RUST=target/riscv64gc-unknown-none-elf/debug
else
	SOURCES_RUST=target/riscv64gc-unknown-none-elf/release
endif

OS_LIBS=-L$(SOURCES_RUST)
CFLAGS_LIBS=-lsamudra_kernel

OUT=../out/samudra-os.elf

all:
	mkdir -p ../out
ifeq ($(TARGET),debug)
	cargo build
else
	cargo build --release
endif
	$(CC) $(CFLAGS) $(CFLAGS_LINKER_SCRIPT) $(CFLAGS_INCLUDE_PATH) $(INCLUDES) -o $(OUT) $(SOURCES_ASM) $(OS_LIBS) $(CFLAGS_LIBS) $(SOURCES_C)

.PHONY: clean
clean:
	cargo clean
	rm -f $(OUT)
