.section .rodata

.global K_MEMORY_HEAP_START
.type K_MEMORY_HEAP_START, @object
K_MEMORY_HEAP_START: .dword __heap_start

.global K_MEMORY_HEAP_END
.type K_MEMORY_HEAP_END, @object
K_MEMORY_HEAP_END: .dword __heap_end

.end
