#include "str.h"

uint_t slen(const string_t s) {
    uint_t len = 0;
    while (*s != null_char) {
        ++len;
    }
    return len;
}
