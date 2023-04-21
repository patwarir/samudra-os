#ifndef SYS_H
#define SYS_H

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/* Extern Assembly constants */

extern const uintptr_t STACK_START, STACK_END;

/* Extern Assembly functions */

extern void halt(void);

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // SYS_H
