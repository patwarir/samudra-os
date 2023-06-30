#ifndef SYS_H
#define SYS_H

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/* Extern Assembly functions */

extern void __attribute__((noreturn)) halt(void);

/* Extern Rust functions */

extern void __attribute__((noreturn)) poweroff(void);
extern void __attribute__((noreturn)) reboot(void);

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // SYS_H
