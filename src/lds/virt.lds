/* RISC-V Architecture, Little Endian */
OUTPUT_ARCH("riscv")
OUTPUT_FORMAT("elf64-littleriscv")

/* Starts at the symbol "_start" */
ENTRY(_start)

/* Memory Regions: Allocatable (a), Writable (w) and Executable (x) */
MEMORY {
    ram (awx) : ORIGIN = 0x80000000, LENGTH = 1024M
}

/* Program Headers (Segments) */
PHDRS {
    text PT_LOAD FLAGS(5); /* RX */
    rodata PT_LOAD FLAGS(4); /* R */
    data PT_LOAD FLAGS(6); /* RW */
    bss PT_LOAD FLAGS(6); /* RW */
}

/* Sections in RAM and Kernel Variables */
SECTIONS {
    . = ORIGIN(ram);

    /* CPU Instructions */
    .text : ALIGN(0x1000) {
        /* Initialization */
        KEEP(*(.text.init .text.init.*));
        *(.text .text.*);

        . = ALIGN(8);

        PROVIDE(etext = .);
    } >ram AT>ram :text

    /* Large Global (readonly) Constants */
    .rodata : ALIGN(0x1000) {
        *(.rodata .rodata.*);
    } >ram AT>ram :rodata

    . = ALIGN(16);

    /* Pointer to make data access faster for small (ro)data */
    __global_pointer$ = . + 0x800;

    /* Small Global (readonly) Constants */
    .srodata : ALIGN(8) {
        *(.srodata .srodata.*);
    } >ram AT>ram :rodata

    /* Small Initialized Variables */
    .sdata : ALIGN(8) {
        *(.sdata .sdata.*);
    } >ram AT>ram :data

    /* Large Initialized Variables */
    .data : ALIGN(0x1000) {
        *(.data .data.*);
    } >ram AT>ram :data

    /* Small and Large Global Uninitialized Variables */
    /* These sections occupy no space in the ELF file but reserve space in RAM */
    .bss (NOLOAD) : ALIGN(0x1000) {
        __bss_start = .;

        *(.sbss .sbss.*);
        *(SCOMMON);

        *(.bss .bss.*);
        *(COMMON);

        . = ALIGN(8);

        __bss_end = .;
    } >ram AT>ram :bss

    /* Thread Local Storage */
    .tls : {
        *(.tdata .tdata.*);
        *(.tbss .tbss.*);
    }

    ASSERT((SIZEOF(.tls) == 0), "ERROR: Cannot have thread local storage!")

    . = ALIGN(0x1000);

    __memory_start = .;

    /DISCARD/ : {}
}
