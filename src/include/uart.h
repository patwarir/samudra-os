#ifndef UART_H
#define UART_H

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/* Extern Rust constants */

extern const string_t C_NEWLINE;

/* Extern Rust functions */

extern void uart_put_c_uchar(uchar_t c);
extern void uart_put_c_string(const string_t s);
extern void uart_put_uint(uint_t i);
extern void uart_put_uint_hex(uint_t i);
extern void uart_put_sint(sint_t i);

/* Macros */

#define println_uchar(c) { uart_put_c_uchar(c); uart_put_c_string(C_NEWLINE); }
#define println_string(s) { uart_put_c_string(s); uart_put_c_string(C_NEWLINE); }
#define println_uint(i) { uart_put_uint(i); uart_put_c_string(C_NEWLINE); }
#define println_uint_hex(i) { uart_put_uint_hex(i); uart_put_c_string(C_NEWLINE); }
#define println_sint(i) { uart_put_sint(i); uart_put_c_string(C_NEWLINE); }

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // UART_H
