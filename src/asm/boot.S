# Disable compressed instructions
.option norvc

.section rodata

# message_entered_halt: .string "Entered halt!\r\n"

.section .data

# Allocatable and executable
.section .text.init, "ax"

# Exported functions

.global _start
_start:
    # Halt if not on HART #0
    csrr t0, mhartid
    bnez t0, halt

    # Make sure SATP is zero
    csrw satp, zero

    # Set global pointer
    # Prevent linker from assuming it's already set
.option push
.option norelax
    la gp, __global_pointer$
.option pop

    # Zero all bytes in the BSS section
    la t0, __bss_start
    la t1, __bss_end
_start_bss_zero_loop:
    sd zero, 0(t0)
    addi t0, t0, 8
    blt t0, t1, _start_bss_zero_loop

    # Set stack pointer and frame pointer
    la sp, __stack_end
    mv fp, sp

    # Set machine status to machine mode
    li t0, 0b11 << 11
    csrw mstatus, t0

    # Set execution begin to the "k_main" function
    la t0, k_main
    csrw mepc, t0

    # Set the kernel trap vector
    la t0, k_trap_vector
    csrw mtvec, t0

    # Disable interrupts
    # li t0, zero
    # csrw mie, t0 # TODO: Check this

    # Set return from "k_main" to be halt
    # Should never happen
    la ra, halt

    # Call "k_main"
    mret

# Infinitely halts
.global halt
halt:
    # TODO: Make sure to push a0 and lr
    # la a0, message_entered_halt
    # la t0, uart_put_c_string
    # jalr t0

    # Disable interrupts
    # li t0, zero
    # csrw mie, t0 # TODO: Check this

    wfi
    j halt
