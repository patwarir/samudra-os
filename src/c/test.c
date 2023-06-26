#include "uart.h"

void call_c_from_rust(void) {
    uart_put_c_string("Hello, World from C!\r\n");
}
