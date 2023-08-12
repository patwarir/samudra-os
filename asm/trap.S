.section .text

.global k_trap_vector
.type k_trap_vector, @function
k_trap_vector:
    la t0, poweroff
    jalr t0
