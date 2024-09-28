# Disable compressed instructions
.option norvc

.section .text.init

# Startup

.global _start
.type _start, @function
_start:
    # Prevent global pointer from assuming gp is already set
.option push
.option norelax
    la gp, __global_pointer$
.option pop

    # Disable paging
    csrw satp, zero

    # Store the Device Tree
    csrw mscratch, a1

    # Set stack pointer for each HART
    csrr t0, mhartid
    # Stack grows downwards
    addi t0, t0, 1
    ld t1, (STACK_SIZE_PER_HART)
    mul t1, t1, t0
    la t0, __memory_start
    add t0, t0, t1
    mv sp, t0
    mv fp, sp

    # Set machine status to machine mode
    li t0, 0b11 << 11
    csrw mstatus, t0

    # Set execution to the "k_main" function
    la t0, k_main
    csrw mepc, t0

    # Set the kernel trap vector to "k_trap_vector"
    la t0, k_trap_vector
    csrw mtvec, t0

    # Set return from "k_main" to "k_halt"
    # Should never happen
    la ra, k_halt

    # Call "k_main"
    mret
