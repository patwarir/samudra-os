CC=$(SYSROOT)/bin/riscv32-unknown-elf-gcc

CFLAGS=--sysroot=$(SYSROOT)
CFLAGS+=-march=rv64gcv_zba_zbb_zbc_zbs_zfh -mabi=lp64d -mcmodel=medany
CFLAGS+=-static -ffreestanding -nostartfiles -nodefaultlibs -nostdlib -nolibc
CFLAGS+=-fno-common -fno-exceptions
CFLAGS+=-Wall -Wextra -std=gnu17

ifeq ($(TARGET),debug)
	CFLAGS+=-O0 -g
else
	CFLAGS+=-O3 -Wl,--gc-sections
endif

CFLAGS_LINKER_SCRIPT=-Tlds/virt.lds
CFLAGS_INCLUDE_PATH=-Iinclude

SOURCES_ASM=$(wildcard asm/*.S)
SOURCES_C=$(wildcard c/*.c)

ifeq ($(TARGET),debug)
	SOURCES_RUST=target/riscv64gc-unknown-none-elf/debug
else
	SOURCES_RUST=target/riscv64gc-unknown-none-elf/release
endif

OS_LIBS=-L$(SOURCES_RUST)
CFLAGS_LIBS=-lsamudra_kernel

OUT=./out/samudra-os.elf

.PHONY: all
all:
	mkdir -p ./out
ifeq ($(TARGET),debug)
	cargo build
else
	cargo build --release
endif
	$(CC) $(CFLAGS) $(CFLAGS_LINKER_SCRIPT) $(CFLAGS_INCLUDE_PATH) -o $(OUT) $(SOURCES_ASM) $(OS_LIBS) $(CFLAGS_LIBS) $(SOURCES_C)

clean:
	-cargo clean
	-rm $(OUT) 2> /dev/null

objdump:
	mkdir -p ./tmp
	$(SYSROOT)/bin/riscv32-unknown-elf-objdump -dC $(OUT) > ./tmp/objdump.txt

readelf:
	mkdir -p ./tmp
	$(SYSROOT)/bin/riscv32-unknown-elf-readelf -a $(OUT) > ./tmp/readelf.txt
