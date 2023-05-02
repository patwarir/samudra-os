#include "sys.h"
#include "uart.h"

void call_me_from_rust(void) {
    println_string("Hello, World from C!");

    uart_put_c_string("Stack start: 0x");
    uart_put_uint_hex(STACK_START);
    uart_put_c_string(C_NEWLINE);

    uart_put_c_string("Stack end: 0x");
    uart_put_uint_hex(STACK_END);
    uart_put_c_string(C_NEWLINE);
}

void __attribute__((noreturn)) c_calls_halt(void) {
    halt();
}

void c_raises_interrupt(void) {
    __asm__ volatile ("ecall");
}
