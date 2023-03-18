#include "dspbptk.h"

// 内部函数

// 这些函数用于解耦dspbptk与base64库
// TODO base64格式异常处理
// TODO 非avx256兼容
size_t base64_enc(const unsigned char* in, size_t inlen, unsigned char* out) {
    return tb64v256enc(in, inlen, out);
}

size_t base64_dec(const unsigned char* in, size_t inlen, unsigned char* out) {
    return tb64v256dec(in, inlen, out);
}

// 这些函数用于解耦dspbptk与gzip库
// TODO gzip格式异常处理
// TODO 更多压缩选项
size_t gzip_enc(const unsigned char* in, size_t inlen, unsigned char* out) {
    size_t gzip_len;
    ZopfliOptions zopfli_opt;
    ZopfliInitOptions(&zopfli_opt);
    ZopfliCompress(&zopfli_opt, ZOPFLI_FORMAT_GZIP, in, inlen, out, &gzip_len);
    return gzip_len;
}

size_t gzip_dec(const unsigned char* in, size_t inlen, unsigned char* out) {
    size_t bin_len;
    struct libdeflate_decompressor* p_decompressor = libdeflate_alloc_decompressor();
    libdeflate_gzip_decompress(p_decompressor, in, inlen, out, BLUEPRINT_MAX_LENGTH, &bin_len);
    libdeflate_free_decompressor(p_decompressor);
    return bin_len;
}

// time in ns
uint64_t get_timestamp(void) {
    struct timespec t;
    clock_gettime(0, &t);
    return (uint64_t)t.tv_sec * 1000000000 + (uint64_t)t.tv_nsec;
}

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
    if(fp == NULL)
        return -1;
    fprintf(fp, "%s", blueprint);
    fclose(fp);
    return 0;
}

// TODO 蓝图格式检查
int blueprint_to_data(bp_data_t* p_bp_data, const char* blueprint) {
    if(blueprint == NULL)
        return -1;

    // 复制蓝图字符串，否则会破坏原蓝图的数据
    size_t bp_len = strlen(blueprint);
    char* str = calloc(bp_len, 1);
    strcpy(str, blueprint);

    // 把蓝图分割成 头|base64|md5f字符串 三部分
    const unsigned char* head = strtok(str, "\"");
    const unsigned char* base64 = strtok(NULL, "\"");
    const unsigned char* md5f_str = strtok(NULL, "\"");
    size_t head_len = strlen(head);
    size_t base64_len = strlen(base64);

    // base64 to gzip
    size_t gzip_len = tb64declen(base64, base64_len);
    unsigned char* gzip = calloc(gzip_len, 1);
    base64_dec(base64, base64_len, gzip);

    // gzip to bin
    p_bp_data->bin_len = BLUEPRINT_MAX_LENGTH;
    p_bp_data->bin = calloc(BLUEPRINT_MAX_LENGTH, 1);
    p_bp_data->bin_len = gzip_dec(gzip, gzip_len, p_bp_data->bin);

    // 解析头
    p_bp_data->shortDesc = calloc(head_len, 1);
    sscanf(head, "BLUEPRINT:0,%llu,%llu,%llu,%llu,%llu,%llu,0,%llu,%llu.%llu.%llu.%llu,%s",
        &p_bp_data->layout,
        &p_bp_data->icons[0],
        &p_bp_data->icons[1],
        &p_bp_data->icons[2],
        &p_bp_data->icons[3],
        &p_bp_data->icons[4],
        &p_bp_data->time,
        &p_bp_data->gameVersion[0],
        &p_bp_data->gameVersion[1],
        &p_bp_data->gameVersion[2],
        &p_bp_data->gameVersion[3],
        p_bp_data->shortDesc
    );

    // 解析二进制流

    // free
    free(gzip);
    free(str);

    return 0;
}

int data_to_blueprint(const bp_data_t* p_bp_data, char* blueprint) {
    char* head = calloc(BLUEPRINT_MAX_LENGTH, 1);
    char* base64 = calloc(BLUEPRINT_MAX_LENGTH, 1);
    char* for_md5f = calloc(BLUEPRINT_MAX_LENGTH, 1);
    char* md5f_str = calloc(33, 1);

    // 编码
    sprintf(head, "BLUEPRINT:0,%llu,%llu,%llu,%llu,%llu,%llu,0,%llu,%llu.%llu.%llu.%llu,%s",
        p_bp_data->layout,
        p_bp_data->icons[0],
        p_bp_data->icons[1],
        p_bp_data->icons[2],
        p_bp_data->icons[3],
        p_bp_data->icons[4],
        p_bp_data->time,
        p_bp_data->gameVersion[0],
        p_bp_data->gameVersion[1],
        p_bp_data->gameVersion[2],
        p_bp_data->gameVersion[3],
        p_bp_data->shortDesc
    );

    // bin to gzip
    unsigned char* gzip;
    size_t gzip_len = gzip_enc(p_bp_data->bin, p_bp_data->bin_len, &gzip);

    // gzip to base64
    size_t base64_len = base64_enc(gzip, gzip_len, base64);

    // md5f
    sprintf(for_md5f, "%s\"%s", head, base64);
    char md5f_hex[33] = { 0 };
    md5f(md5f_hex, for_md5f, strlen(for_md5f));
    sprintf(blueprint, "%s\"%s", for_md5f, md5f_hex);
    // puts(md5f_hex); // for debug


    free(head);
    free(base64);
    free(for_md5f);
    free(md5f_str);

    return 0;
}

// TODO 还没写完
int data_to_json(const bp_data_t* p_bp_data, char** p_json) {
    // Create a mutable doc
    yyjson_mut_doc* doc = yyjson_mut_doc_new(NULL);
    yyjson_mut_val* root = yyjson_mut_obj(doc);
    yyjson_mut_doc_set_root(doc, root);

    // 蓝图头
    // TODO 从蓝图头的结构自动生成
    yyjson_mut_obj_add_uint(doc, root, "layout", p_bp_data->layout);
    yyjson_mut_val* icons = yyjson_mut_arr_with_uint(doc, p_bp_data->icons, 5);
    yyjson_mut_obj_add_val(doc, root, "icons", icons);
    yyjson_mut_obj_add_uint(doc, root, "time", p_bp_data->time);
    yyjson_mut_val* gameVersion = yyjson_mut_arr_with_uint(doc, p_bp_data->gameVersion, 4);
    yyjson_mut_obj_add_val(doc, root, "gameVersion", gameVersion);
    yyjson_mut_obj_add_str(doc, root, "shortDesc", p_bp_data->shortDesc);

    // To string, minified
    *p_json = yyjson_mut_write(doc, 0, NULL);

    // Free the doc
    yyjson_mut_doc_free(doc);

    return 0;
}

void free_bp_data(bp_data_t* p_bp_data) {
    free(p_bp_data->shortDesc);
    free(p_bp_data->bin);
}