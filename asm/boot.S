# Disable compressed instructions
.option norvc

.section .text.init

# Exported functions

.global _start
.type _start, @function
_start:
    .cfi_startproc
    .cfi_undefined ra

    # Set global pointer
    # Prevent linker from assuming it's already set
.option push
.option norelax
    la gp, __global_pointer$
.option pop

    # Make sure SATP is zero
    csrw satp, zero

    # Halt if not on HART #0
    csrr t0, mhartid
    bnez t0, halt

    # Zero all bytes in the BSS section
    la t0, __bss_start
    la t1, __bss_end
_start_bss_zero_loop:
    sd zero, (t0)
    addi t0, t0, 8
    blt t0, t1, _start_bss_zero_loop

    # Set stack pointer and frame pointer
    la sp, __stack_end
    mv fp, sp

    # TODO: Set stack pointer for other HARTs

    # Set machine status to machine mode and setup FPU + vector
    li t0, 0b11 << 11 | 0b1 << 13 | 0b1 << 9
    csrw mstatus, t0

    # Set execution begin to the "k_main" function
    la t0, k_main
    csrw mepc, t0

    # Set the kernel trap vector
    la t0, k_trap_vector
    csrw mtvec, t0

    # Set return from "k_main" to be halt
    # Should never happen
    la ra, halt

    # Call "k_main"
    mret

    .cfi_endproc

# Infinitely halts
.global halt
.type halt, @function
halt:
    .cfi_startproc

    wfi
    j halt

    .cfi_endproc

.end
