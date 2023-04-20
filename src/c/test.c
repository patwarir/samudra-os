#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "sys.h"
#include "uart.h"

/* Exported functions */

void call_me_from_rust(void) {
    uart_put_c_string("Hello, World from C!\r\n");

    uart_put_c_string("Stack start: ");
    uart_put_uint(STACK_START);
    uart_put_c_string("\r\n");

    uart_put_c_string("Stack end: ");
    uart_put_uint(STACK_END);
    uart_put_c_string("\r\n");
}

void c_calls_halt(void) {
    halt();
}
