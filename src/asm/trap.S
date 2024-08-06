.section .text

.global k_trap_vector
.type k_trap_vector, @function
k_trap_vector:
    la t0, k_poweroff
    jalr t0
