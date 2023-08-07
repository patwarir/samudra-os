#ifndef TYPES_H
#define TYPES_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

typedef unsigned char char_t;

typedef const char_t * strc_t;
typedef char_t * strm_t;

typedef _Float16 f16_t;
typedef float f32_t;
typedef double f64_t;

#define null_ptr NULL
#define null_char '\0'

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // TYPES_H
