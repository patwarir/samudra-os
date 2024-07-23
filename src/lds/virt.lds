/* RISC-V Architecture, Little Endian */
OUTPUT_ARCH("riscv")
OUTPUT_FORMAT("elf64-littleriscv")

/* Starts at the symbol "_start" */
ENTRY(_start)

/* Allocatable (a), Writable (w) and Executable (x) */
MEMORY {
    ram (awx) : ORIGIN = 0x80000000, LENGTH = 1024M
}

/* Program Headers */
PHDRS {
    text PT_LOAD;
    data PT_LOAD;
    bss PT_LOAD;
}

/*
    Set sections in RAM
    Provide variables to the kernel
*/
SECTIONS {
    /* CPU instructions */
    .text : ALIGN(4K) {
        /* Initialization */
        KEEP(*(.text.init .text.init.*));
        *(.text .text.*);

        PROVIDE(etext = .);
    } >ram AT>ram :text

    /* Global (readonly) constants and initialized variables */
    .data : ALIGN(4K) {
        /* Pointer to make data access faster for small (ro)data */
        __global_pointer$ = . + 0x800;

        *(.srodata .srodata.*);
        *(.sdata .sdata.*);

        *(.rodata .rodata.*);
        *(.data .data.*);
    } >ram AT>ram :data

    /* Global uninitialized variables */
    .bss : ALIGN(4K) {
        __bss_start = .;

        *(.sbss .sbss.*);
        *(.bss .bss.*);

        __bss_end = .;
    } >ram AT>ram :bss

    /* Thread-local storage */
    .tls : {
        *(.tdata .tdata.*);
        *(.tbss .tbss.*);
    }

    ASSERT((SIZEOF(.tls) == 0), "ERROR: Cannot have thread-local storage")

    . = ALIGN(4K);

    /* Exports & Discards */

    __memory_start = .;

    /DISCARD/ : {
        *(.eh_frame .eh_frame.*);
        *(.comment .comment.*);
        *(.note .note.*);
    }
}
