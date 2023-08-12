.section .rodata

.global K_MEMORY_HEAP_START
K_MEMORY_HEAP_START: .dword __heap_start

.global K_MEMORY_HEAP_END
K_MEMORY_HEAP_END: .dword __heap_end
