#ifndef UART_H
#define UART_H

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/* Extern Rust functions */

extern void uart_put_c_uchar(uchar_t c);
extern void uart_put_c_string(const string_t s);
extern void uart_put_uint(uint_t i);
extern void uart_put_uint_hex(uint_t i);
extern void uart_put_sint(sint_t i);

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // UART_H
