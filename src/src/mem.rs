use crate::uart;

extern "C" {
    static K_MEM_STACK_START: usize;
    static K_MEM_STACK_END: usize;
    static K_MEM_HEAP_START: usize;
    static K_MEM_HEAP_END: usize;
}

pub fn print_mem_values() {
    unsafe {
        uart::uart_put_str("Kernel memory stack start: 0x");
        uart::uart_put_uint_hex(K_MEM_STACK_START);
        uart::uart_put_nl();

        uart::uart_put_str("Kernel memory stack end: 0x");
        uart::uart_put_uint_hex(K_MEM_STACK_END);
        uart::uart_put_nl();

        uart::uart_put_str("Kernel memory heap start: 0x");
        uart::uart_put_uint_hex(K_MEM_HEAP_START);
        uart::uart_put_nl();

        uart::uart_put_str("Kernel memory heap end: 0x");
        uart::uart_put_uint_hex(K_MEM_HEAP_END);
        uart::uart_put_nl();
    }
}
