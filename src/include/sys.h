#ifndef SYS_H
#define SYS_H

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/* Extern Assembly functions */

extern void __attribute__((noreturn)) halt(void);

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // SYS_H
