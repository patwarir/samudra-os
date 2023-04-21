#include "sys.h"
#include "uart.h"

/* Exported functions */

void call_me_from_rust(void) {
    uart_put_c_string("Hello, World from C!\r\n");

    uart_put_c_string("Stack start: 0x");
    uart_put_uint_hex(STACK_START);
    uart_put_c_string(NEWLINE);

    uart_put_c_string("Stack end: 0x");
    uart_put_uint_hex(STACK_END);
    uart_put_c_string(NEWLINE);
}

void c_calls_halt(void) {
    halt();
}
