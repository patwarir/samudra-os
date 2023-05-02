#ifndef STR_H
#define STR_H

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#define null_char '\0'

uint_t slen(const string_t s);

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // STR_H