#include "libdspbptk.h"

// 原版游戏蓝图中的空建筑被定义为-1
#define OBJ_NULL ((int32_t)-1)

// 枚举了一系列的偏移值，通过这些偏移值移动指针即可快速访问蓝图中的特定数据

// 二进制流的头部数据块中各元素的偏移值定义
typedef enum {
    bin_offset_version = 0,
    bin_offset_cursorOffset_x = bin_offset_version + 4,
    bin_offset_cursorOffset_y = bin_offset_cursorOffset_x + 4,
    bin_offset_cursorTargetArea = bin_offset_cursorOffset_y + 4,
    bin_offset_dragBoxSize_x = bin_offset_cursorTargetArea + 4,
    bin_offset_dragBoxSize_y = bin_offset_dragBoxSize_x + 4,
    bin_offset_primaryAreaIdx = bin_offset_dragBoxSize_y + 4,
    BIN_OFFSET_AREA_NUM = bin_offset_primaryAreaIdx + 4,
    BIN_OFFSET_AREA_ARRAY = BIN_OFFSET_AREA_NUM + 1
}bin_offset_t;

// 区域数据块中各元素的偏移值
typedef enum {
    area_offset_index = 0,
    area_offset_parentIndex = area_offset_index + 1,
    area_offset_tropicAnchor = area_offset_parentIndex + 1,
    area_offset_areaSegments = area_offset_tropicAnchor + 2,
    area_offset_anchorLocalOffsetX = area_offset_areaSegments + 2,
    area_offset_anchorLocalOffsetY = area_offset_anchorLocalOffsetX + 2,
    area_offset_width = area_offset_anchorLocalOffsetY + 2,
    area_offset_height = area_offset_width + 2,
    AREA_OFFSET_AREA_NEXT = area_offset_height + 2,
    AREA_OFFSET_BUILDING_ARRAY = AREA_OFFSET_AREA_NEXT + 4
}area_offset_t;

// 建筑数据块中各元素的偏移值
typedef enum {
    building_offset_index = 0,
    building_offset_areaIndex = building_offset_index + 4,
    building_offset_localOffset_x = building_offset_areaIndex + 1,
    building_offset_localOffset_y = building_offset_localOffset_x + 4,
    building_offset_localOffset_z = building_offset_localOffset_y + 4,
    building_offset_localOffset_x2 = building_offset_localOffset_z + 4,
    building_offset_localOffset_y2 = building_offset_localOffset_x2 + 4,
    building_offset_localOffset_z2 = building_offset_localOffset_y2 + 4,
    building_offset_yaw = building_offset_localOffset_z2 + 4,
    building_offset_yaw2 = building_offset_yaw + 4,
    building_offset_itemId = building_offset_yaw2 + 4,
    building_offset_modelIndex = building_offset_itemId + 2,
    building_offset_tempOutputObjIdx = building_offset_modelIndex + 2,
    building_offset_tempInputObjIdx = building_offset_tempOutputObjIdx + 4,
    building_offset_outputToSlot = building_offset_tempInputObjIdx + 4,
    building_offset_inputFromSlot = building_offset_outputToSlot + 1,
    building_offset_outputFromSlot = building_offset_inputFromSlot + 1,
    building_offset_inputToSlot = building_offset_outputFromSlot + 1,
    building_offset_outputOffset = building_offset_inputToSlot + 1,
    building_offset_inputOffset = building_offset_outputOffset + 1,
    building_offset_recipeId = building_offset_inputOffset + 1,
    building_offset_filterId = building_offset_recipeId + 2,
    building_offset_num = building_offset_filterId + 2,
    building_offset_parameters = building_offset_num + 2
}building_offset_t;





// 这些函数用于解耦dspbptk与底层库依赖，如果需要更换底层库时只要换掉这几个函数里就行

/**
 * @brief 通用的base64编码。用于解耦dspbptk与更底层的base64库
 *
 * @param in 编码前的二进制流
 * @param inlen 编码前的二进制流长度
 * @param out 编码后的二进制流
 * @return size_t 编码后的二进制流长度
 */
size_t base64_enc(const void* in, size_t inlen, char* out) {
    return tb64v256enc((unsigned char*)in, inlen, (unsigned char*)out);
}

/**
 * @brief 通用的base64解码。用于解耦dspbptk与更底层的base64库
 *
 * @param in 解码前的二进制流
 * @param inlen 解码前的二进制流长度
 * @param out 解码后的二进制流
 * @return size_t 当返回值<=0时表示解码错误；当返回值>=1时，表示成功并返回解码后的二进制流长度
 */
size_t base64_dec(const char* in, size_t inlen, void* out) {
    return tb64v256dec((unsigned char*)in, inlen, (unsigned char*)out);
}

/**
 * @brief 返回base64解码后的准确长度。用于解耦dspbptk与更底层的base64库
 */
size_t base64_declen(const char* base64, size_t base64_length) {
    return tb64declen((unsigned char*)base64, base64_length);
}

// 这些函数用于解耦dspbptk与gzip库
// TODO 更多压缩选项
/**
 * @brief 通用的gzip压缩。用于解耦dspbptk与更底层的gzip库
 *
 * @param in 压缩前的二进制流
 * @param in_nbytes 压缩前的二进制流长度
 * @param out 压缩后的二进制流
 * @return size_t 压缩后的二进制流长度
 */
size_t gzip_enc(const unsigned char* in, size_t in_nbytes, unsigned char** out) {
    size_t out_nbytes_avail = BLUEPRINT_MAX_LENGTH;
    *out = calloc(out_nbytes_avail, 1);
    struct libdeflate_compressor* p_compressor = libdeflate_alloc_compressor(12);
    size_t gzip_length = libdeflate_gzip_compress(p_compressor, in, in_nbytes, *out, out_nbytes_avail);
    libdeflate_free_compressor(p_compressor);
    return gzip_length;
}

/**
 * @brief 通用的gzip解压。用于解耦dspbptk与更底层的gzip库
 *
 * @param in 解压前的二进制流
 * @param in_nbytes 解压前的二进制流的长度
 * @param out 解压后的二进制流
 * @return size_t 当返回值<=3时，表示解压错误；当返回值>=4时，表示解压成功并返回解压后的二进制流长度
 */
size_t gzip_dec(const unsigned char* in, size_t in_nbytes, unsigned char* out) {
    size_t actual_out_nbytes_ret;
    struct libdeflate_decompressor* p_decompressor = libdeflate_alloc_decompressor();
    enum libdeflate_result result = libdeflate_gzip_decompress(p_decompressor, in, in_nbytes, out, BLUEPRINT_MAX_LENGTH, &actual_out_nbytes_ret);
    libdeflate_free_decompressor(p_decompressor);
    if(result != LIBDEFLATE_SUCCESS)
        return (size_t)result;
    else
        return actual_out_nbytes_ret;
}





// I/O 与文件交互

size_t file_to_blueprint(const char* file_name, char** p_blueprint) {
    FILE* fp = fopen(file_name, "r");
    if(fp == NULL)
        return 0;
    fseek(fp, 0, SEEK_END);
    size_t length = ftell(fp);
    fseek(fp, 0, SEEK_SET);
    // 获取文件长度后快速读入内存
    *p_blueprint = (char*)calloc(length + 1, 1);
    fread(*p_blueprint, 1, length, fp);
    fclose(fp);
    return length;
}

dspbptk_err_t blueprint_to_file(const char* file_name, const char* blueprint) {
    FILE* fp = fopen(file_name, "w");
    if(fp == NULL)
        return cannot_write;
    // 获取字符串长度后快速写入文件
    size_t length = strlen(blueprint);
    fwrite(blueprint, 1, length, fp);
    fclose(fp);
    return no_error;
}





// API 封装一些读写蓝图中的数据块的常用操作，可以自己拓展

size_t get_area_num(void* p_area_num) {
    return (size_t) * ((int8_t*)p_area_num);
}

void set_area_num(void* p_area_num, size_t num) {
    *((int8_t*)p_area_num) = (int8_t)num;
}

size_t get_building_num(void* p_building_num) {
    return (size_t) * ((int32_t*)p_building_num);
}

void set_building_num(void* p_building_num, size_t num) {
    *((int32_t*)p_building_num) = (int32_t)num;
}

size_t get_building_size(void* p_building) {
    int16_t* p_num = (int16_t*)((unsigned char*)p_building + building_offset_num);
    return (size_t)(building_offset_parameters + 4 * (*p_num));
}

int16_t get_building_itemID(void* p_building) {
    return *((int16_t*)(p_building + building_offset_itemId));
}

int32_t get_building_index(void* p_building) {
    return *((int32_t*)(p_building + building_offset_index));
}

void set_building_index(void* p_building, int32_t index) {
    *((int32_t*)(p_building + building_offset_index)) = (int32_t)index;
}

void set_building_tempOutputObjIdx(void* p_building, int32_t index) {
    *((int32_t*)(p_building + building_offset_tempOutputObjIdx)) = (int32_t)index;
}

void set_building_tempInputObjIdx(void* p_building, int32_t index) {
    *((int32_t*)(p_building + building_offset_tempInputObjIdx)) = (int32_t)index;
}










/**
 * @brief 检查字符串的头部的蓝图标识
 *
 * @param str 待检查的字符串
 * @return int 是否为蓝图
 */
int is_blueprint(const char* str) {
    if(str == NULL)
        return 0;
    return !memcmp(str, "BLUEPRINT:", 10);
}

// TODO 蓝图格式检查
// TODO 返回值枚举
dspbptk_err_t blueprint_to_data(bp_data_t* p_bp_data, const char* blueprint) {
    // 检查当前字符串是否是蓝图
    if(!is_blueprint(blueprint))
        return not_a_blueprint;

    // 复制蓝图字符串，否则会破坏原蓝图的数据
    char* str = calloc(strlen(blueprint) + 1, 1);
    if(str == NULL)
        return out_of_memory;
    strcpy(str, blueprint);

    // 把蓝图分割成 head|base64|md5f_str 三部分
    const char* head = strtok(str, "\"");
    const char* base64 = strtok(NULL, "\"");
    // const char* md5f_str = strtok(NULL, "\"");
    const size_t head_length = strlen(head);
    const size_t base64_length = strlen(base64);

    // base64 to gzip
    size_t gzip_length = base64_declen(base64, base64_length);
    unsigned char* gzip = (unsigned char*)calloc(gzip_length, 1);
    if(gzip == NULL)
        return out_of_memory;
    gzip_length = base64_dec(base64, base64_length, gzip);
    if(gzip_length <= 0)
        return broken_blueprint;

    // gzip to bin
    const size_t bin_length = *((int32_t*)(gzip + gzip_length - 4));
    if(bin_length <= 3)
        return broken_blueprint;
    p_bp_data->bin = calloc(bin_length, 1);
    if(p_bp_data->bin == NULL)
        return out_of_memory;
    gzip_dec(gzip, gzip_length, p_bp_data->bin);

    // 解析蓝图头部明文段的数据
    p_bp_data->shortDesc = calloc(head_length + 1, 1);
    if(p_bp_data->shortDesc == NULL)
        return out_of_memory;
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
    unsigned char* ptr = p_bp_data->bin;
    // 解析区域数组
    ptr += BIN_OFFSET_AREA_NUM;
    p_bp_data->area_num = get_area_num(ptr);
    p_bp_data->area = calloc(p_bp_data->area_num, sizeof(void*));
    if(p_bp_data->area == NULL)
        return out_of_memory;
    ptr += BIN_OFFSET_AREA_ARRAY - BIN_OFFSET_AREA_NUM;
    for(int i = 0; i < p_bp_data->area_num; i++) {
        p_bp_data->area[i] = ptr;
        ptr += AREA_OFFSET_AREA_NEXT;
    }
    // 解析建筑数组
    p_bp_data->building_num = get_building_num(ptr);
    p_bp_data->building = calloc(p_bp_data->building_num, sizeof(void*));
    if(p_bp_data->building == NULL)
        return out_of_memory;
    ptr += AREA_OFFSET_BUILDING_ARRAY - AREA_OFFSET_AREA_NEXT;
    for(int i = 0; i < p_bp_data->building_num; i++) {
        p_bp_data->building[i] = ptr;
        ptr += get_building_size(ptr);
    }

    // free
    free(gzip);
    free(str);

    // 能运行到这里通常没有很要命的问题
    return no_error;
}






typedef struct {
    int32_t id;
    int32_t index;
}id_t;

int cmp_building(const void* p_a, const void* p_b) {
    void* a = *((void**)p_a);
    void* b = *((void**)p_b);
    return get_building_itemID(a) - get_building_itemID(b);
}

int cmp_id(const void* p_a, const void* p_b) {
    id_t* a = ((id_t*)p_a);
    id_t* b = ((id_t*)p_b);
    return a->id - b->id;
}

int cmp_index(const void* p_a, const void* p_b) {
    id_t* a = ((id_t*)p_a);
    id_t* b = ((id_t*)p_b);
    return a->index - b->index;
}

void re_index(int32_t* ObjIdx, id_t* id_lut, size_t building_num) {
    if(*ObjIdx == OBJ_NULL)
        return;
    id_t* p_id = bsearch(ObjIdx, id_lut, building_num, sizeof(id_t), cmp_id);
    if(p_id == NULL) {
        fprintf(stderr, "Warning: index %d no found! Reindex index to OBJ_NULL(-1).\n", *ObjIdx);
        *ObjIdx = OBJ_NULL;
    }
    else {
        *ObjIdx = p_id->index;
    }
}

dspbptk_err_t data_to_blueprint(const bp_data_t* p_bp_data, char* blueprint) {
    // 指针指向蓝图明文段
    char* blueprint_ptr = blueprint;

    // 输出蓝图头
    sprintf(blueprint_ptr, "BLUEPRINT:0,%llu,%llu,%llu,%llu,%llu,%llu,0,%llu,%llu.%llu.%llu.%llu,%s\"",
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

    // 指针移动到base64段
    size_t head_length = strlen(blueprint);
    blueprint_ptr += head_length;

    // data to bin
    unsigned char* bin = calloc(BLUEPRINT_MAX_LENGTH, 1);
    if(bin == NULL)
        return out_of_memory;

    // 指针指向二进制流头部，写入
    unsigned char* bin_ptr = bin;
    memcpy(bin_ptr, p_bp_data->bin, BIN_OFFSET_AREA_NUM);

    // 写入有区域总数
    bin_ptr += BIN_OFFSET_AREA_NUM;
    set_area_num(bin_ptr, p_bp_data->area_num);

    // 写入区域数组
    bin_ptr += BIN_OFFSET_AREA_ARRAY - BIN_OFFSET_AREA_NUM;
    for(int i = 0; i < p_bp_data->area_num; i++) {
        memcpy(bin_ptr, p_bp_data->area[i], AREA_OFFSET_AREA_NEXT);
        bin_ptr += AREA_OFFSET_AREA_NEXT;
    }

    // 写入建筑总数
    set_building_num(bin_ptr, p_bp_data->building_num);

#if 1 // 对建筑按建筑类型排序，有利于进一步压缩，非必要步骤
    qsort(p_bp_data->building, p_bp_data->building_num, sizeof(void*), cmp_building);
#endif

    // 重新生成index
    id_t* id_lut = (id_t*)calloc(p_bp_data->building_num, sizeof(id_t));
    for(int i = 0; i < p_bp_data->building_num; i++) {
        id_lut[i].id = get_building_index(p_bp_data->building[i]);
        id_lut[i].index = i;
    }
    qsort(id_lut, p_bp_data->building_num, sizeof(id_t), cmp_id);

    // 写入建筑数组
    bin_ptr += AREA_OFFSET_BUILDING_ARRAY - AREA_OFFSET_AREA_NEXT;
    for(int32_t i = 0; i < p_bp_data->building_num; i++) {
        // 写入单个建筑
        size_t building_size = get_building_size(p_bp_data->building[i]);
        memcpy(bin_ptr, p_bp_data->building[i], building_size);
        // 重新编码index
        re_index((int32_t*)(bin_ptr + building_offset_index), id_lut, p_bp_data->building_num);
        re_index((int32_t*)(bin_ptr + building_offset_tempOutputObjIdx), id_lut, p_bp_data->building_num);
        re_index((int32_t*)(bin_ptr + building_offset_tempInputObjIdx), id_lut, p_bp_data->building_num);
        // 移动指针
        bin_ptr += building_size;
    }
    free(id_lut);

    // bin to gzip
    size_t bin_length = bin_ptr - bin;
    unsigned char* gzip = NULL;
    size_t gzip_length = gzip_enc(bin, bin_length, &gzip);

    // gzip to base64
    size_t base64_length = base64_enc(gzip, gzip_length, blueprint_ptr);

    // md5f
    blueprint_ptr += base64_length;
    char md5f_hex[33] = { 0 };
    md5f(md5f_hex, blueprint, head_length + base64_length);
    sprintf(blueprint_ptr, "\"%s\0", md5f_hex);

    // free
    free(bin);
    free(gzip);

    // 能运行到这里通常没有很要命的问题
    return no_error;
}

// TODO 还没写完
size_t data_to_json(const bp_data_t* p_bp_data, char** p_json) {
    // Create a mutable doc
    yyjson_mut_doc* doc = yyjson_mut_doc_new(NULL);
    yyjson_mut_val* root = yyjson_mut_obj(doc);
    yyjson_mut_doc_set_root(doc, root);

    // 只能生成蓝图头，以后再写
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
    free(p_bp_data->area);
    free(p_bp_data->building);
}