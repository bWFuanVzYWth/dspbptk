#include "dspbptk.h"

#ifdef DEBUG
long long int last_time = 0;
#endif

// time
uint64_t get_timestamp(void) {
    struct timespec t;
    clock_gettime(0, &t);
    return (uint64_t)t.tv_sec * 1000000000 + (uint64_t)t.tv_nsec;
}

// TODO free
size_t file_to_blueprint(const char* file_name, char** p_blueprint) {
    FILE* fp = fopen(file_name, "r");
    if(fp == NULL)
        return 0;
    fseek(fp, 0, SEEK_END);
    size_t length = ftell(fp);
    fseek(fp, 0, SEEK_SET);
    *p_blueprint = (char*)calloc(length + 1, 1);
    fread(*p_blueprint, 1, length, fp);
    fclose(fp);
    return length;
}

int blueprint_to_file(const char* file_name, const char* blueprint) {
    FILE* fp = fopen(file_name, "w");
    fprintf(fp, "%s", blueprint);
    fclose(fp);
    return 0;
}

// TODO 蓝图格式检查
int blueprint_to_data(bp_data_t* p_bp_data, const char* blueprint) {
    debug_reset();
    if(blueprint == NULL)
        return -1;
    debug("start");

    // 复制
    size_t bp_len = strlen(blueprint);
    char* str = calloc(bp_len, sizeof(char));
    strcpy(str, blueprint);
    debug("strcpy");

    // 分割蓝图
    const unsigned char* head = strtok(str, "\"");
    const unsigned char* base64 = strtok(NULL, "\"");
    const unsigned char* tail = strtok(NULL, "\"");
    debug("strtok");

    // base64 to gzip
    // TODO 非avx256兼容
    size_t base64_len = strlen(base64);
    size_t gzip_len = tb64declen(base64, base64_len);
    unsigned char* gzip = calloc(gzip_len, 1);
    tb64v256dec(base64, base64_len, gzip);
    debug("base64 dec");

    // gzip to raw
    // TODO 测试能否直接从gzip中读取raw的大小，优化内存占用
    p_bp_data->raw_len = BP_LEN;
    p_bp_data->raw = calloc(BP_LEN, 1);
    struct libdeflate_decompressor* p_decompressor = libdeflate_alloc_decompressor();
    libdeflate_gzip_decompress(p_decompressor, gzip, gzip_len, p_bp_data->raw, BP_LEN, &(p_bp_data->raw_len));
    libdeflate_free_decompressor(p_decompressor);
    debug("gzip dec");
    // 解析

    // free
    free(gzip);
    free(str);

    return 0;
}

void free_bp_data(bp_data_t* p_bp_data) {
    free(p_bp_data->raw);
}