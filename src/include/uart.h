#ifndef UART_H
#define UART_H

#include <stdint.h>

/* Extern Rust functions */

extern void uart_put_c_string(const unsigned char *const s);
extern void uart_put_uint(uintptr_t i);

#endif // UART_H
