.section .rodata

message_entered_trap_vector: .string "Entered k_trap_vector!\r\n"

.section .text

.global k_trap_vector
k_trap_vector:
    # TODO: Make sure to push a0 and lr
    la a0, message_entered_trap_vector
    la t0, uart_put_c_string
    jalr t0

    mret
