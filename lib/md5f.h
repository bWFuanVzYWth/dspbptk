#ifndef MD5F
#define MD5F

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>

void md5f(uint32_t md5f_u32[4], const char* stream, size_t stream_len);
void md5f_str(char* md5f_hex, const char* stream, size_t stream_len);

#ifdef __cplusplus
}
#endif

#endif