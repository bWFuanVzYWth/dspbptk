#ifndef MD5F
#define MD5F

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>

void md5f(char* md5f_hex, const char* stream, size_t stream_len);

#ifdef __cplusplus
}
#endif

#endif