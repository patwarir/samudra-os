ifneq ($(SYSROOT),)
	CFLAGS+=--sysroot=$(SYSROOT)
endif

CFLAGS+=-march=rv64gcvh_zba_zbb_zbc_zbs_zfh -mabi=lp64d -mcmodel=medany
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

ifeq ($(OUT_DIR),)
	OUT_DIR=./out
endif

ifeq ($(TMP_DIR),)
	TMP_DIR=./tmp
endif

OUT_FILE=$(OUT_DIR)/samudra-os.elf

ifeq ($(OBJDUMP_FLAGS),)
	OBJDUMP_FLAGS=-dC
endif

ifeq ($(READELF_FLAGS),)
	READELF_FLAGS=-a
endif

ifeq ($(WASMDUMP_FLAGS),)
	WASMDUMP_FLAGS=print -p
endif

ifeq ($(TARGET_FILE),)
	TARGET_FILE=$(OUT_FILE)
endif

.PHONY: all
all:
	mkdir -p $(OUT_DIR)
ifeq ($(TARGET),debug)
	cargo build
else
	cargo build --release
endif
	$(CC) $(CFLAGS) $(CFLAGS_LINKER_SCRIPT) $(CFLAGS_INCLUDE_PATH) -o $(OUT_FILE) $(SOURCES_ASM) $(OS_LIBS) $(CFLAGS_LIBS) $(SOURCES_C)

.PHONY: clean
clean:
	-cargo clean
	-rm -r $(OUT_DIR) 2> /dev/null

.PHONY: objdump
objdump:
	mkdir -p $(TMP_DIR)
	$(OBJDUMP) $(OBJDUMP_FLAGS) $(TARGET_FILE) > $(TMP_DIR)/objdump.txt

.PHONY: readelf
readelf:
	mkdir -p $(TMP_DIR)
	$(READELF) $(READELF_FLAGS) $(TARGET_FILE) > $(TMP_DIR)/readelf.txt

.PHONY: wasmdump
wasmdump:
	mkdir -p $(TMP_DIR)
	$(WASMDUMP) $(WASMDUMP_FLAGS) $(TARGET_FILE) > $(TMP_DIR)/wasmdump.txt
