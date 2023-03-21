#include "libdspbptk.h"

// 定义了一系列的偏移值，通过这些偏移值即可使用指针快速访问蓝图中的特定数据
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
size_t gzip_enc(const unsigned char* in, size_t inlen, unsigned char** out) {
    size_t gzip_len;

    size_t out_nbytes_avail = BLUEPRINT_MAX_LENGTH;
    *out = calloc(out_nbytes_avail, 1);
    struct libdeflate_compressor* p_compressor = libdeflate_alloc_compressor(12);
    gzip_len = libdeflate_gzip_compress(p_compressor, in, inlen, *out, out_nbytes_avail);
    libdeflate_free_compressor(p_compressor);

    return gzip_len;
}

size_t gzip_dec(const unsigned char* in, size_t inlen, unsigned char* out) {
    size_t bin_len;
    struct libdeflate_decompressor* p_decompressor = libdeflate_alloc_decompressor();
    libdeflate_gzip_decompress(p_decompressor, in, inlen, out, BLUEPRINT_MAX_LENGTH, &bin_len);
    libdeflate_free_decompressor(p_decompressor);
    return bin_len;
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

dspbptk_err_t blueprint_to_file(const char* file_name, const char* blueprint) {
    FILE* fp = fopen(file_name, "w");
    if(fp == NULL)
        return cannot_write;
    fwrite(blueprint, 1, strlen(blueprint), fp);
    fclose(fp);
    return no_error;
}

size_t get_area_num(void* p_area_num) {
    return (size_t) * ((int8_t*)p_area_num);
}

void set_area_num(void* p_area_num, size_t n) {
    *((int8_t*)p_area_num) = (int8_t)n;
}

size_t get_building_num(void* p_building_num) {
    return (size_t) * ((int32_t*)p_building_num);
}

void set_building_num(void* p_building_num, size_t n) {
    *((int32_t*)p_building_num) = (int32_t)n;
}

size_t get_building_size(void* p_building) {
    int16_t* p_num = (int16_t*)((unsigned char*)p_building + building_offset_num);
    return (size_t)(building_offset_parameters + 4 * (*p_num));
}

int16_t get_building_itemID(void* p_building) {
    return *((int16_t*)(p_building + building_offset_itemId));
}

// TODO 蓝图格式检查
// TODO 返回值枚举
dspbptk_err_t blueprint_to_data(bp_data_t* p_bp_data, const char* blueprint) {
    if(blueprint == NULL)
        return null_ptr;

    const char* HEADER = "BLUEPRINT:";
    if(memcmp(blueprint, HEADER, 10))
        return not_a_blueprint;

    // 复制蓝图字符串，否则会破坏原蓝图的数据
    size_t bp_len = strlen(blueprint);
    char* str = calloc(bp_len, 1);
    if(str == NULL)
        return out_of_memory;
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
    if(gzip == NULL)
        return out_of_memory;
    base64_dec(base64, base64_len, gzip);

    // gzip to bin
    size_t bin_len = BLUEPRINT_MAX_LENGTH;
    p_bp_data->bin = calloc(BLUEPRINT_MAX_LENGTH, 1);
    if(p_bp_data->bin == NULL)
        return out_of_memory;
    bin_len = gzip_dec(gzip, gzip_len, p_bp_data->bin);

    // 解析头
    p_bp_data->shortDesc = calloc(head_len, 1);
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

    return no_error;
}

// TODO 检查蓝图是不是野指针，是否被初始化为0
int data_to_blueprint(const bp_data_t* p_bp_data, char* blueprint) {
    char* blueprint_ptr = blueprint;

    // 输出蓝图头
    sprintf(blueprint, "BLUEPRINT:0,%llu,%llu,%llu,%llu,%llu,%llu,0,%llu,%llu.%llu.%llu.%llu,%s\"",
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
    size_t head_len = strlen(blueprint);
    blueprint_ptr += head_len;

    // data to bin
    unsigned char* bin = calloc(BLUEPRINT_MAX_LENGTH, 1);
    unsigned char* ptr = bin;
    // 生成建筑头
    memcpy(ptr, p_bp_data->bin, BIN_OFFSET_AREA_NUM);
    // 生成区域数组
    ptr += BIN_OFFSET_AREA_NUM;
    set_area_num(ptr, p_bp_data->area_num);
    ptr += BIN_OFFSET_AREA_ARRAY - BIN_OFFSET_AREA_NUM;
    for(int i = 0; i < p_bp_data->area_num; i++) {
        memcpy(ptr, p_bp_data->area[i], AREA_OFFSET_AREA_NEXT);
        ptr += AREA_OFFSET_AREA_NEXT;
    }
    // 生成建筑数组
    set_building_num(ptr, p_bp_data->building_num);
    ptr += AREA_OFFSET_BUILDING_ARRAY - AREA_OFFSET_AREA_NEXT;
    // TODO 自动重设index
    for(int i = 0; i < p_bp_data->building_num; i++) {
        size_t building_size = get_building_size(p_bp_data->building[i]);
        memcpy(ptr, p_bp_data->building[i], building_size);
        ptr += building_size;
    }
    size_t bin_len = ptr - bin;

    // bin to gzip
    unsigned char* gzip = NULL;
    size_t gzip_len = gzip_enc(bin, bin_len, &gzip);

    // gzip to base64
    size_t base64_len = base64_enc(gzip, gzip_len, blueprint_ptr);
    blueprint_ptr += base64_len;

    // md5f
    char md5f_hex[33] = { 0 };
    md5f(md5f_hex, blueprint, head_len + base64_len);
    sprintf(blueprint_ptr, "\"%s", md5f_hex);

    free(bin);
    free(gzip);

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
    free(p_bp_data->area);
    free(p_bp_data->building);
}