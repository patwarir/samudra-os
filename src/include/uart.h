#ifndef UART_H
#define UART_H

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/* Extern Rust constants */

extern const string_t NEWLINE;

/* Extern Rust functions */

extern void uart_put_char(uchar_t c);
extern void uart_put_c_string(const string_t s);
extern void uart_put_uint(uintptr_t i);
extern void uart_put_uint_hex(uintptr_t i);
extern void uart_put_int(intptr_t i);

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // UART_H
