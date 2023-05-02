#ifndef TYPES_H
#define TYPES_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

typedef uintptr_t uint_t;
typedef intptr_t sint_t;

typedef unsigned char uchar_t;
typedef uchar_t * string_t;

typedef uintptr_t ptr_t;

#define null NULL

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // TYPES_H
