/* RISC-V Architecture */
OUTPUT_ARCH( "riscv" )

/* Starts at the symbol "_start" */
ENTRY( _start )

/*
    Writable (w), Executable (x) and Allocatable (a)
    Not Readonly (r), Not Initialized (i)
    ORIGIN is at 0x8000_0000
    LENGTH is 512M of RAM
*/
MEMORY {
    ram (wxa!ri) : ORIGIN = 0x80000000, LENGTH = 512M
}

/* Program Headers */
PHDRS {
    text PT_LOAD;
    data PT_LOAD;
    bss PT_LOAD;
}

/*
    Set Sections in RAM
    Provide variables to the kernel
*/
SECTIONS {
    /* CPU Instructions, begin with .text.init */
    .text : ALIGN(4K) {
        PROVIDE(__text_start = .);
        *(.text.init);
        *(.text);
        *(.text.*);
        PROVIDE(__text_end = .);
    } >ram AT>ram :text

    /* Exported, pointer to start of the global constants and variables */
    PROVIDE(__global_pointer$ = .);

    /* Global (readonly) constants */
    .rodata : ALIGN(4K) {
        PROVIDE(__rodata_start = .);
        *(.rodata);
        *(.rodata.*);
        PROVIDE(__rodata_end = .);
    } >ram AT>ram :text

    /* Global initialized variables */
    .data : ALIGN(4K) {
        PROVIDE(__data_start = .);
        *(.sdata);
        *(.sdata.*);
        *(.data);
        *(.data.*);
        PROVIDE(__data_end = .);
    } >ram AT>ram :data

    /* Global uninitialized variables */
    .bss : ALIGN(4K) {
        PROVIDE(__bss_start = .);
        *(.sbss);
        *(.sbss.*);
        *(.bss);
        *(.bss.*);
        PROVIDE(__bss_end = .);
    } >ram AT>ram :bss

    /* Exported labels */

    PROVIDE(__memory_size = LENGTH(ram));
    PROVIDE(__memory_start = ORIGIN(ram));
    PROVIDE(__memory_end = __memory_start + __memory_size);

    PROVIDE(__stack_size = 768K);
    PROVIDE(__stack_start = __bss_end);
    PROVIDE(__stack_end = __stack_start + __stack_size);

    PROVIDE(__heap_start = __stack_end);
    PROVIDE(__heap_end = __memory_end);
    PROVIDE(__heap_size = __heap_end - __heap_start);
}
