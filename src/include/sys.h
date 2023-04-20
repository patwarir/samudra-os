#ifndef SYS_H
#define SYS_H

#include <stdint.h>

/* Extern Assembly constants */

extern const uintptr_t STACK_START, STACK_END;

/* Extern Assembly functions */

extern void halt(void);

#endif // SYS_H
