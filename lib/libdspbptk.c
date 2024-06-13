#include "enum_offset.h"
#include "libdspbptk.h"

////////////////////////////////////////////////////////////////////////////////
// 这些函数用于解耦dspbptk与底层库依赖，如果需要更换底层库时只要换掉这几个函数里就行
////////////////////////////////////////////////////////////////////////////////

/**
 * @brief 通用的base64编码。用于解耦dspbptk与更底层的base64库。假定out有足够的空间。
 *
 * @param in 编码前的二进制流
 * @param inlen 编码前的二进制流长度
 * @param out 编码后的二进制流
 * @return size_t 编码后的二进制流长度
 */
size_t base64_enc(const void* in, size_t inlen, char* out) {
    return chromium_base64_encode(out, in, inlen);
}

/**
 * @brief 通用的base64解码。用于解耦dspbptk与更底层的base64库。假定out有足够的空间。
 *
 * @param in 解码前的二进制流
 * @param inlen 解码前的二进制流长度
 * @param out 解码后的二进制流
 * @return size_t 当返回值<=0时表示解码错误；当返回值>=1时，表示成功并返回解码后的二进制流长度
 */
size_t base64_dec(const char* in, size_t inlen, void* out) {
    return chromium_base64_decode(out, in, inlen);
}

/**
 * @brief 返回base64解码后的准确长度。用于解耦dspbptk与更底层的base64库
 */
size_t base64_declen(const char* base64, size_t base64_length) {
    return chromium_base64_decode_len(base64_length);
}

/**
 * @brief 通用的gzip压缩。用于解耦dspbptk与更底层的gzip库
 *
 * @param in 压缩前的二进制流
 * @param in_nbytes 压缩前的二进制流长度
 * @param out 压缩后的二进制流
 * @return size_t 压缩后的二进制流长度
 */
size_t gzip_enc(dspbptk_coder_t* coder, const unsigned char* in, size_t in_nbytes, unsigned char* out) {
    size_t gzip_length = libdeflate_gzip_compress(coder->p_compressor, in, in_nbytes, out, 0xffffffff);
    return gzip_length;
}

/**
 * @brief 通用的gzip解压。用于解耦dspbptk与更底层的gzip库。假定out有足够的空间。
 *
 * @param in 解压前的二进制流
 * @param in_nbytes 解压前的二进制流的长度
 * @param out 解压后的二进制流
 * @return size_t 当返回值<=3时，表示解压错误；当返回值>=4时，表示解压成功并返回解压后的二进制流长度
 */
size_t gzip_dec(dspbptk_coder_t* coder, const unsigned char* in, size_t in_nbytes, unsigned char* out) {
    size_t actual_out_nbytes_ret;
    enum libdeflate_result result = libdeflate_gzip_decompress(
        coder->p_decompressor, in, in_nbytes, out, 0xffffffff, &actual_out_nbytes_ret);
    if (result != LIBDEFLATE_SUCCESS)
        return (size_t)result;
    else
        return actual_out_nbytes_ret;
}

/**
 * @brief 返回gzip解压后的准确长度。
 *
 * @param in 解压前的二进制流
 * @param in_nbytes 解压前的二进制流的长度
 * @return size_t 解压后的二进制流长度
 */
size_t gzip_declen(const unsigned char* in, size_t in_nbytes) {
    return (size_t) * ((uint32_t*)(in + in_nbytes - 4));
}

////////////////////////////////////////////////////////////////////////////////
// dspbptk decode
////////////////////////////////////////////////////////////////////////////////

size_t blueprint_file_size(FILE* fp) {
    size_t offset_backup = ftell(fp);
    fseek(fp, 0, SEEK_END);
    size_t file_size = ftell(fp);
    fseek(fp, offset_backup, SEEK_SET);
    return file_size;
}

i64_t* dspbptk_calloc_parameters(size_t N) {
    return (i64_t*)calloc(N, sizeof(i64_t));
}

char* dspbptk_calloc_md5f(void) {
    return (char*)calloc(MD5F_LENGTH, sizeof(char));
}

char* dspbptk_calloc_gameversion(void) {
    return (char*)calloc(GAMEVERSION_MAX_LENGTH, sizeof(char));
}

char* dspbptk_calloc_shortdesc(void) {
    return (char*)calloc(SHORTDESC_MAX_LENGTH, sizeof(char));
}

char* dspbptk_calloc_desc(void) {
    return (char*)calloc(DESC_MAX_LENGTH, sizeof(char));
}

area_t* dspbptk_calloc_areas(size_t area_num) {
    return (area_t*)calloc(area_num, sizeof(area_t));
}

building_t* dspbptk_calloc_buildings(size_t building_num) {
    return (building_t*)calloc(building_num, sizeof(building_t));
}

size_t read_bin_head(blueprint_t* blueprint, const unsigned char* ptr_bin) {
    blueprint->version = *((i32_t*)(ptr_bin + bin_offset_version));
    blueprint->cursorOffsetX = *((i32_t*)(ptr_bin + bin_offset_cursorOffsetX));
    blueprint->cursorOffsetY = *((i32_t*)(ptr_bin + bin_offset_cursorOffsetY));
    blueprint->cursorTargetArea = *((i32_t*)(ptr_bin + bin_offset_cursorTargetArea));
    blueprint->dragBoxSizeX = *((i32_t*)(ptr_bin + bin_offset_dragBoxSizeX));
    blueprint->dragBoxSizeY = *((i32_t*)(ptr_bin + bin_offset_dragBoxSizeY));
    blueprint->primaryAreaIdx = *((i32_t*)(ptr_bin + bin_offset_primaryAreaIdx));
    blueprint->numAreas = (size_t) * ((i8_t*)(ptr_bin + bin_offset_numAreas));
    return bin_offset_areas;
}

size_t write_bin_head(const blueprint_t* blueprint, unsigned char* ptr_bin) {
    *((i32_t*)(ptr_bin + bin_offset_version)) = (i32_t)blueprint->version;
    *((i32_t*)(ptr_bin + bin_offset_cursorOffsetX)) = (i32_t)blueprint->cursorOffsetX;
    *((i32_t*)(ptr_bin + bin_offset_cursorOffsetY)) = (i32_t)blueprint->cursorOffsetY;
    *((i32_t*)(ptr_bin + bin_offset_cursorTargetArea)) = (i32_t)blueprint->cursorTargetArea;
    *((i32_t*)(ptr_bin + bin_offset_dragBoxSizeX)) = (i32_t)blueprint->dragBoxSizeX;
    *((i32_t*)(ptr_bin + bin_offset_dragBoxSizeY)) = (i32_t)blueprint->dragBoxSizeY;
    *((i32_t*)(ptr_bin + bin_offset_primaryAreaIdx)) = (i32_t)blueprint->primaryAreaIdx;
    *((i8_t*)(ptr_bin + bin_offset_numAreas)) = (i8_t)blueprint->numAreas;
    return bin_offset_areas;
}

size_t read_area(area_t* area, const unsigned char* ptr_bin) {
    area->index = *((i8_t*)(ptr_bin + area_offset_index));
    area->parentIndex = *((i8_t*)(ptr_bin + area_offset_parentIndex));
    area->tropicAnchor = *((i16_t*)(ptr_bin + area_offset_tropicAnchor));
    area->areaSegments = *((i16_t*)(ptr_bin + area_offset_areaSegments));
    area->anchorLocalOffsetX = *((i16_t*)(ptr_bin + area_offset_anchorLocalOffsetX));
    area->anchorLocalOffsetY = *((i16_t*)(ptr_bin + area_offset_anchorLocalOffsetY));
    area->width = *((i16_t*)(ptr_bin + area_offset_width));
    area->height = *((i16_t*)(ptr_bin + area_offset_height));
    return area_offset_next;
}

size_t write_area(const area_t* area, unsigned char* ptr_bin) {
    *((i8_t*)(ptr_bin + area_offset_index)) = (i8_t)area->index;
    *((i8_t*)(ptr_bin + area_offset_parentIndex)) = (i8_t)area->parentIndex;
    *((i16_t*)(ptr_bin + area_offset_tropicAnchor)) = (i16_t)area->tropicAnchor;
    *((i16_t*)(ptr_bin + area_offset_areaSegments)) = (i16_t)area->areaSegments;
    *((i16_t*)(ptr_bin + area_offset_anchorLocalOffsetX)) = (i16_t)area->anchorLocalOffsetX;
    *((i16_t*)(ptr_bin + area_offset_anchorLocalOffsetY)) = (i16_t)area->anchorLocalOffsetY;
    *((i16_t*)(ptr_bin + area_offset_width)) = (i16_t)area->width;
    *((i16_t*)(ptr_bin + area_offset_height)) = (i16_t)area->height;
    return area_offset_next;
}

size_t read_numBuildings(blueprint_t* blueprint, const unsigned char* ptr_bin) {
    blueprint->numBuildings = (size_t) * ((i32_t*)(ptr_bin));
    return sizeof(int32_t);
}

size_t write_numBuildings(const blueprint_t* blueprint, unsigned char* ptr_bin) {
    *((i32_t*)(ptr_bin)) = (i32_t)blueprint->numBuildings;
    return sizeof(int32_t);
}

size_t read_building(building_t* building, const unsigned char* ptr_bin) {
    i32_t num = *((i32_t*)ptr_bin);

    building->num = 0;
    building->tilt = 0.0;

    switch (num) {
    default:
        building->index            = *((i32_t *)(ptr_bin + building_offset_index));
        building->areaIndex        = *((i8_t *)(ptr_bin + building_offset_areaIndex));
        building->localOffset[0]   = *((f32_t *)(ptr_bin + building_offset_localOffset_x));
        building->localOffset[1]   = *((f32_t *)(ptr_bin + building_offset_localOffset_y));
        building->localOffset[2]   = *((f32_t *)(ptr_bin + building_offset_localOffset_z));
        building->localOffset[3]   = 1.0;
        building->localOffset2[0]  = *((f32_t *)(ptr_bin + building_offset_localOffset_x2));
        building->localOffset2[1]  = *((f32_t *)(ptr_bin + building_offset_localOffset_y2));
        building->localOffset2[2]  = *((f32_t *)(ptr_bin + building_offset_localOffset_z2));
        building->localOffset2[3]  = 1.0;
        building->yaw              = *((f32_t *)(ptr_bin + building_offset_yaw));
        building->yaw2             = *((f32_t *)(ptr_bin + building_offset_yaw2));
        building->itemId           = *((i16_t *)(ptr_bin + building_offset_itemId));
        building->modelIndex       = *((i16_t *)(ptr_bin + building_offset_modelIndex));
        building->tempOutputObjIdx = *((i32_t *)(ptr_bin + building_offset_tempOutputObjIdx));
        building->tempInputObjIdx  = *((i32_t *)(ptr_bin + building_offset_tempInputObjIdx));
        building->outputToSlot     = *((i8_t *)(ptr_bin + building_offset_outputToSlot));
        building->inputFromSlot    = *((i8_t *)(ptr_bin + building_offset_inputFromSlot));
        building->outputFromSlot   = *((i8_t *)(ptr_bin + building_offset_outputFromSlot));
        building->inputToSlot      = *((i8_t *)(ptr_bin + building_offset_inputToSlot));
        building->outputOffset     = *((i8_t *)(ptr_bin + building_offset_outputOffset));
        building->inputOffset      = *((i8_t *)(ptr_bin + building_offset_inputOffset));
        building->recipeId         = *((i16_t *)(ptr_bin + building_offset_recipeId));
        building->filterId         = *((i16_t *)(ptr_bin + building_offset_filterId));
        building->numParameters    = *((i16_t *)(ptr_bin + building_offset_numParameters));
        building->parameters       = dspbptk_calloc_parameters(building->numParameters);
        for (size_t j = 0; j < building->numParameters; j++)
            building->parameters[j] = *((i32_t *)((ptr_bin + building_offset_parameters) + j * sizeof(i32_t)));
        return building_offset_parameters + building->numParameters * sizeof(i32_t);

    case -100:
        building->num              = -100;
        building->index            = *((i32_t *)(ptr_bin + building_neg100_offset_index));
        building->areaIndex        = *((i8_t *)(ptr_bin + building_neg100_offset_areaIndex));
        building->localOffset[0]   = *((f32_t *)(ptr_bin + building_neg100_offset_localOffset_x));
        building->localOffset[1]   = *((f32_t *)(ptr_bin + building_neg100_offset_localOffset_y));
        building->localOffset[2]   = *((f32_t *)(ptr_bin + building_neg100_offset_localOffset_z));
        building->localOffset[3]   = 1.0;
        building->localOffset2[0]  = *((f32_t *)(ptr_bin + building_neg100_offset_localOffset_x2));
        building->localOffset2[1]  = *((f32_t *)(ptr_bin + building_neg100_offset_localOffset_y2));
        building->localOffset2[2]  = *((f32_t *)(ptr_bin + building_neg100_offset_localOffset_z2));
        building->localOffset2[3]  = 1.0;
        building->yaw              = *((f32_t *)(ptr_bin + building_neg100_offset_yaw));
        building->yaw2             = *((f32_t *)(ptr_bin + building_neg100_offset_yaw2));
        building->tilt             = *((f32_t *)(ptr_bin + building_neg100_offset_tilt));
        building->itemId           = *((i16_t *)(ptr_bin + building_neg100_offset_itemId));
        building->modelIndex       = *((i16_t *)(ptr_bin + building_neg100_offset_modelIndex));
        building->tempOutputObjIdx = *((i32_t *)(ptr_bin + building_neg100_offset_tempOutputObjIdx));
        building->tempInputObjIdx  = *((i32_t *)(ptr_bin + building_neg100_offset_tempInputObjIdx));
        building->outputToSlot     = *((i8_t *)(ptr_bin + building_neg100_offset_outputToSlot));
        building->inputFromSlot    = *((i8_t *)(ptr_bin + building_neg100_offset_inputFromSlot));
        building->outputFromSlot   = *((i8_t *)(ptr_bin + building_neg100_offset_outputFromSlot));
        building->inputToSlot      = *((i8_t *)(ptr_bin + building_neg100_offset_inputToSlot));
        building->outputOffset     = *((i8_t *)(ptr_bin + building_neg100_offset_outputOffset));
        building->inputOffset      = *((i8_t *)(ptr_bin + building_neg100_offset_inputOffset));
        building->recipeId         = *((i16_t *)(ptr_bin + building_neg100_offset_recipeId));
        building->filterId         = *((i16_t *)(ptr_bin + building_neg100_offset_filterId));
        building->numParameters    = *((i16_t *)(ptr_bin + building_neg100_offset_numParameters));
        building->parameters = dspbptk_calloc_parameters(building->numParameters);
        for (size_t j = 0; j < building->numParameters; j++)
            building->parameters[j] = *((i32_t *)((ptr_bin + building_neg100_offset_parameters) + j * sizeof(i32_t)));
        return building_neg100_offset_parameters + building->numParameters * sizeof(i32_t);

    }
}

size_t write_building(const building_t* building, unsigned char* ptr_bin, const index_t* id_lut, size_t numBuildings) {
    switch(building->num) {
    default:
        *((i32_t*)(ptr_bin + building_offset_index))            = get_idx(&building->index, id_lut, numBuildings);
        *((i8_t*)(ptr_bin + building_offset_areaIndex))         = (i8_t)building->areaIndex;
        *((f32_t*)(ptr_bin + building_offset_localOffset_x))    = (f32_t)(building->localOffset[0] / building->localOffset[3]);
        *((f32_t*)(ptr_bin + building_offset_localOffset_y))    = (f32_t)(building->localOffset[1] / building->localOffset[3]);
        *((f32_t*)(ptr_bin + building_offset_localOffset_z))    = (f32_t)(building->localOffset[2] / building->localOffset[3]);
        *((f32_t*)(ptr_bin + building_offset_localOffset_x2))   = (f32_t)(building->localOffset2[0] / building->localOffset2[3]);
        *((f32_t*)(ptr_bin + building_offset_localOffset_y2))   = (f32_t)(building->localOffset2[1] / building->localOffset2[3]);
        *((f32_t*)(ptr_bin + building_offset_localOffset_z2))   = (f32_t)(building->localOffset2[2] / building->localOffset2[3]);
        *((f32_t*)(ptr_bin + building_offset_yaw))              = (f32_t)building->yaw;
        *((f32_t*)(ptr_bin + building_offset_yaw2))             = (f32_t)building->yaw2;
        *((i16_t*)(ptr_bin + building_offset_itemId))           = (i16_t)building->itemId;
        *((i16_t*)(ptr_bin + building_offset_modelIndex))       = (i16_t)building->modelIndex;
        *((i32_t*)(ptr_bin + building_offset_tempOutputObjIdx)) = get_idx(&building->tempOutputObjIdx, id_lut, numBuildings);
        *((i32_t*)(ptr_bin + building_offset_tempInputObjIdx))  = get_idx(&building->tempInputObjIdx, id_lut, numBuildings);
        *((i8_t*)(ptr_bin + building_offset_outputToSlot))      = (i8_t)building->outputToSlot;
        *((i8_t*)(ptr_bin + building_offset_inputFromSlot))     = (i8_t)building->inputFromSlot;
        *((i8_t*)(ptr_bin + building_offset_outputFromSlot))    = (i8_t)building->outputFromSlot;
        *((i8_t*)(ptr_bin + building_offset_inputToSlot))       = (i8_t)building->inputToSlot;
        *((i8_t*)(ptr_bin + building_offset_outputOffset))      = (i8_t)building->outputOffset;
        *((i8_t*)(ptr_bin + building_offset_inputOffset))       = (i8_t)building->inputOffset;
        *((i16_t*)(ptr_bin + building_offset_recipeId))         = (i16_t)building->recipeId;
        *((i16_t*)(ptr_bin + building_offset_filterId))         = (i16_t)building->filterId;
        *((i16_t*)(ptr_bin + building_offset_numParameters))    = (i16_t)building->numParameters;
        for (size_t j = 0; j < building->numParameters; j++)
            *((i32_t*)((ptr_bin + building_offset_parameters) + sizeof(i32_t) * j)) = (i32_t)building->parameters[j];
        return building_offset_parameters + building->numParameters * sizeof(i32_t);

    case -100:
        *((i32_t*)(ptr_bin + building_neg100_offset_num)) = -100;
        *((i32_t*)(ptr_bin + building_neg100_offset_index))            = get_idx(&building->index, id_lut, numBuildings);
        *((i8_t*)(ptr_bin + building_neg100_offset_areaIndex))         = (i8_t)building->areaIndex;
        *((f32_t*)(ptr_bin + building_neg100_offset_localOffset_x))    = (f32_t)(building->localOffset[0] / building->localOffset[3]);
        *((f32_t*)(ptr_bin + building_neg100_offset_localOffset_y))    = (f32_t)(building->localOffset[1] / building->localOffset[3]);
        *((f32_t*)(ptr_bin + building_neg100_offset_localOffset_z))    = (f32_t)(building->localOffset[2] / building->localOffset[3]);
        *((f32_t*)(ptr_bin + building_neg100_offset_localOffset_x2))   = (f32_t)(building->localOffset2[0] / building->localOffset2[3]);
        *((f32_t*)(ptr_bin + building_neg100_offset_localOffset_y2))   = (f32_t)(building->localOffset2[1] / building->localOffset2[3]);
        *((f32_t*)(ptr_bin + building_neg100_offset_localOffset_z2))   = (f32_t)(building->localOffset2[2] / building->localOffset2[3]);
        *((f32_t*)(ptr_bin + building_neg100_offset_yaw))              = (f32_t)building->yaw;
        *((f32_t*)(ptr_bin + building_neg100_offset_yaw2))             = (f32_t)building->yaw2;
        *((f32_t*)(ptr_bin + building_neg100_offset_tilt))             = (f32_t)building->tilt;
        *((i16_t*)(ptr_bin + building_neg100_offset_itemId))           = (i16_t)building->itemId;
        *((i16_t*)(ptr_bin + building_neg100_offset_modelIndex))       = (i16_t)building->modelIndex;
        *((i32_t*)(ptr_bin + building_neg100_offset_tempOutputObjIdx)) = get_idx(&building->tempOutputObjIdx, id_lut, numBuildings);
        *((i32_t*)(ptr_bin + building_neg100_offset_tempInputObjIdx))  = get_idx(&building->tempInputObjIdx, id_lut, numBuildings);
        *((i8_t*)(ptr_bin + building_neg100_offset_outputToSlot))      = (i8_t)building->outputToSlot;
        *((i8_t*)(ptr_bin + building_neg100_offset_inputFromSlot))     = (i8_t)building->inputFromSlot;
        *((i8_t*)(ptr_bin + building_neg100_offset_outputFromSlot))    = (i8_t)building->outputFromSlot;
        *((i8_t*)(ptr_bin + building_neg100_offset_inputToSlot))       = (i8_t)building->inputToSlot;
        *((i8_t*)(ptr_bin + building_neg100_offset_outputOffset))      = (i8_t)building->outputOffset;
        *((i8_t*)(ptr_bin + building_neg100_offset_inputOffset))       = (i8_t)building->inputOffset;
        *((i16_t*)(ptr_bin + building_neg100_offset_recipeId))         = (i16_t)building->recipeId;
        *((i16_t*)(ptr_bin + building_neg100_offset_filterId))         = (i16_t)building->filterId;
        *((i16_t*)(ptr_bin + building_neg100_offset_numParameters))    = (i16_t)building->numParameters;
        for (size_t j = 0; j < building->numParameters; j++)
            *((i32_t*)((ptr_bin + building_neg100_offset_parameters) + sizeof(i32_t) * j)) = (i32_t)building->parameters[j];
        return building_neg100_offset_parameters + building->numParameters * sizeof(i32_t);

    }
}

int blueprint_read_head(blueprint_t* blueprint, const char* string) {
    blueprint->gameVersion = dspbptk_calloc_gameversion();
    blueprint->shortDesc = dspbptk_calloc_shortdesc();
    blueprint->desc = dspbptk_calloc_desc();
    // 注意此处读取不包含双引号
    return sscanf(string, "BLUEPRINT:0,%" PRId64 ",%" PRId64 ",%" PRId64 ",%" PRId64 ",%" PRId64 ",%" PRId64 ",0,%" PRId64 ",%[^,],%[^,],%[^\"]",
                  &blueprint->layout,
                  &blueprint->icons[0],
                  &blueprint->icons[1],
                  &blueprint->icons[2],
                  &blueprint->icons[3],
                  &blueprint->icons[4],
                  &blueprint->time,
                  blueprint->gameVersion,
                  blueprint->shortDesc,
                  blueprint->desc);
}

int blueprint_write_head(const blueprint_t* blueprint, char* string) {
    // 注意此处输出包含了双引号
    return sprintf(string, "BLUEPRINT:0,%" PRId64 ",%" PRId64 ",%" PRId64 ",%" PRId64 ",%" PRId64 ",%" PRId64 ",0,%" PRId64 ",%s,%s,%s\"",
                   blueprint->layout,
                   blueprint->icons[0],
                   blueprint->icons[1],
                   blueprint->icons[2],
                   blueprint->icons[3],
                   blueprint->icons[4],
                   blueprint->time,
                   blueprint->gameVersion,
                   blueprint->shortDesc == NULL ? "\0" : blueprint->shortDesc,
                   blueprint->desc == NULL ? "\0" : blueprint->desc);
}

dspbptk_error_t blueprint_decode(dspbptk_coder_t* coder, blueprint_t* blueprint, const char* string, size_t string_length) {
    // 初始化结构体，置零
    memset(blueprint, 0, sizeof(blueprint_t));

    // 记录蓝图字符串的长度
    coder->string_length = string_length;

    // 检查是不是蓝图：蓝图head标识为"BLUEPRINT:"，直接检查即可判定是否为蓝图
#ifndef DSPBPTK_NO_ERROR
    if (string_length < 10)
        return not_blueprint;
    if (memcmp(string, "BLUEPRINT:", 10) != 0)
        return not_blueprint;
#endif

    // 蓝图的结构：head标识 + head明文数据 + '\"' + base64 + '\"' + MD5F
    // 根据双引号即可分割字符串，计算出head和base64的位置和长度
    const char* head = string;
    const char* base64 = strchr(string, (int)'\"') + 1;
    const char* md5f = string + string_length - MD5F_LENGTH;
#ifndef DSPBPTK_NO_ERROR
    if (*(md5f - 1) != '\"')
        return blueprint_md5f_broken;
#endif
    const size_t head_length = (size_t)(base64 - head - 1);
    const size_t base64_length = (size_t)(md5f - base64 - 1);

    // 解析MD5F(并校验)
#ifndef DSPBPTK_NO_WARNING
    char md5f_check[MD5F_LENGTH] = "\0";
    md5f_str(md5f_check, coder->buffer1, string, head_length + 1 + base64_length);
    if (memcmp(md5f, md5f_check, MD5F_LENGTH) != 0)
        fprintf(stderr, "Warning: MD5 abnormal!\nthis:\t%32s\nactual:\t%32s\n", md5f, md5f_check);
#endif
    blueprint->md5f = dspbptk_calloc_md5f();
    memcpy(blueprint->md5f, md5f, MD5F_LENGTH);

    // 解析head明文数据
    int argc = blueprint_read_head(blueprint, string);
#ifndef DSPBPTK_NO_ERROR
    if(argc != 10) {
        fprintf(stderr, "Error: Head broken! argc=%d.\n", argc);
        return blueprint_head_broken;
    }
#endif

    // base64 >> gzip
    size_t gzip_length = base64_declen(base64, base64_length);
    void* gzip = coder->buffer0;
    gzip_length = base64_dec(base64, base64_length, gzip);
#ifndef DSPBPTK_NO_ERROR
    if (gzip_length <= 0)
        return blueprint_base64_broken;
#endif

    // gzip >> bin
    size_t bin_length = gzip_declen(gzip, gzip_length);
    unsigned char* bin = coder->buffer1;
    bin_length = gzip_dec(coder, gzip, gzip_length, bin);
#ifndef DSPBPTK_NO_ERROR
    if (bin_length <= 3)
        return blueprint_gzip_broken;
#endif

    // 用于操作二进制流的指针
    unsigned char* ptr_bin = bin;

    // 解析二进制流的头
    ptr_bin += read_bin_head(blueprint, ptr_bin);
    blueprint->areas = dspbptk_calloc_areas(blueprint->numAreas);

    // 解析区域数组
    for (size_t i = 0; i < blueprint->numAreas; i++)
        ptr_bin += read_area(&blueprint->areas[i], ptr_bin);

    // 解析建筑数量
    ptr_bin += read_numBuildings(blueprint, ptr_bin);
    blueprint->buildings = dspbptk_calloc_buildings(blueprint->numBuildings);

    // 解析建筑数组
    for (size_t i = 0; i < blueprint->numBuildings; i++)
        ptr_bin += read_building(&blueprint->buildings[i], ptr_bin);

    return no_error;
}

dspbptk_error_t blueprint_decode_file(dspbptk_coder_t* coder, blueprint_t* blueprint, FILE* fp) {
    if (coder->buffer_string == NULL)
        coder->buffer_string = calloc(BLUEPRINT_MAX_LENGTH, 1);
    size_t string_length = blueprint_file_size(fp);
    fread(coder->buffer_string, 1, string_length, fp);
    return blueprint_decode(coder, blueprint, coder->buffer_string, string_length);
}

////////////////////////////////////////////////////////////////////////////////
// dspbptk encode
////////////////////////////////////////////////////////////////////////////////

int cmp_id(const void* p_a, const void* p_b) {
    index_t* a = ((index_t*)p_a);
    index_t* b = ((index_t*)p_b);
    return a->id - b->id;
}

void generate_lut(const blueprint_t* blueprint, index_t* id_lut) {
    for (size_t i = 0; i < blueprint->numBuildings; i++) {
        id_lut[i].id = blueprint->buildings[i].index;
        id_lut[i].index = i;
    }
    qsort(id_lut, blueprint->numBuildings, sizeof(index_t), cmp_id);
}

i32_t get_idx(const i64_t* ObjIdx, const index_t* id_lut, size_t numBuildings) {
    if (*ObjIdx == OBJ_NULL)
        return OBJ_NULL;
    index_t* p_id = bsearch(ObjIdx, id_lut, numBuildings, sizeof(index_t), cmp_id);
    if (p_id != NULL)
        return p_id->index;

#ifndef DSPBPTK_NO_WARNING  // 正常来说走不到这里，走到了说明index存在异常
    fprintf(stderr, "Warning: index %" PRId64 " no found! Reindex index to OBJ_NULL(-1).\n", *ObjIdx);
#endif
    return OBJ_NULL;
}

dspbptk_error_t blueprint_encode(dspbptk_coder_t* coder, const blueprint_t* blueprint, char* string) {
    // 初始化用于操作的几个指针
    unsigned char* bin = coder->buffer0;
    char* ptr_str = string;
    unsigned char* ptr_bin = bin;

    // 输出head
    int head_length = blueprint_write_head(blueprint, ptr_str) - 1;
    ptr_str += head_length + 1;

    // 编码bin head
    ptr_bin += write_bin_head(blueprint, ptr_bin);

    // 编码区域数组
    for (size_t i = 0; i < blueprint->numAreas; i++)
        ptr_bin += write_area(&blueprint->areas[i], ptr_bin);

    // 编码建筑总数
    ptr_bin += write_numBuildings(blueprint, ptr_bin);

    // 重新生成index
    index_t* id_lut = (index_t*)coder->buffer1;
    generate_lut(blueprint, id_lut);

    // 编码建筑数组
    for (size_t i = 0; i < blueprint->numBuildings; i++)
        ptr_bin += write_building(&blueprint->buildings[i], ptr_bin, id_lut, blueprint->numBuildings);

    // 计算二进制流长度
    size_t bin_length = (size_t)(ptr_bin - bin);
    void* gzip = coder->buffer1;

    // bin >> gzip
    size_t gzip_length = gzip_enc(coder, bin, bin_length, gzip);

    // gzip >> base64
    size_t base64_length = base64_enc(gzip, gzip_length, ptr_str);
    ptr_str += base64_length;

    // 计算md5f
    *ptr_str = '\"';
    ptr_str += 1;
    md5f_str(ptr_str, coder->buffer1, string, head_length + 1 + base64_length);
    ptr_str += 32;

    coder->string_length = (size_t)(ptr_str - string);

    return no_error;
}

dspbptk_error_t blueprint_encode_file(dspbptk_coder_t* coder, const blueprint_t* blueprint, FILE* fp) {
    if (coder->buffer_string == NULL)
        coder->buffer_string = calloc(BLUEPRINT_MAX_LENGTH, 1);
    dspbptk_error_t error_level = blueprint_encode(coder, blueprint, coder->buffer_string);
    fwrite(coder->buffer_string, 1, coder->string_length, fp);
    return error_level;
}

////////////////////////////////////////////////////////////////////////////////
// dspbptk free blueprint
////////////////////////////////////////////////////////////////////////////////

void dspbptk_free_blueprint(blueprint_t* blueprint) {
    free(blueprint->gameVersion);
    free(blueprint->shortDesc);
    free(blueprint->desc);
    free(blueprint->md5f);
    free(blueprint->areas);
    for (size_t i = 0; i < blueprint->numBuildings; i++)
        if (blueprint->buildings[i].numParameters > 0)
            free(blueprint->buildings[i].parameters);
    free(blueprint->buildings);
}

////////////////////////////////////////////////////////////////////////////////
// dspbptk alloc coder
////////////////////////////////////////////////////////////////////////////////

void dspbptk_init_coder(dspbptk_coder_t* coder) {
    coder->buffer0 = calloc(BLUEPRINT_MAX_LENGTH, 1);
    coder->buffer1 = calloc(BLUEPRINT_MAX_LENGTH, 1);
    coder->buffer_string = NULL;
    coder->p_compressor = libdeflate_alloc_compressor(12);
    coder->p_decompressor = libdeflate_alloc_decompressor();
}

////////////////////////////////////////////////////////////////////////////////
// dspbptk free coder
////////////////////////////////////////////////////////////////////////////////

void dspbptk_free_coder(dspbptk_coder_t* coder) {
    free(coder->buffer0);
    free(coder->buffer1);
    if (coder->buffer_string != NULL)
        free(coder->buffer_string);
    libdeflate_free_compressor(coder->p_compressor);
    libdeflate_free_decompressor(coder->p_decompressor);
}

////////////////////////////////////////////////////////////////////////////////
// dspbptk api
////////////////////////////////////////////////////////////////////////////////

void dspbptk_blueprint_init(blueprint_t* blueprint) {
    // blueprint data
    memset(blueprint, 0, sizeof(blueprint_t));
    blueprint->version = 1;
    blueprint->cursorOffsetX = 0;
    blueprint->cursorOffsetY = 0;
    blueprint->cursorTargetArea = 0;
    blueprint->dragBoxSizeX = 1;
    blueprint->dragBoxSizeY = 1;
    blueprint->primaryAreaIdx = 0;

    // area data
    blueprint->numAreas = 1;
    blueprint->areas = calloc(1, sizeof(area_t));
    blueprint->areas[0].index = 0;
    blueprint->areas[0].parentIndex = OBJ_NULL;
    blueprint->areas[0].tropicAnchor = 0;
    blueprint->areas[0].areaSegments = 200;
    blueprint->areas[0].anchorLocalOffsetX = 0;
    blueprint->areas[0].anchorLocalOffsetY = 0;
    blueprint->areas[0].width = 1;
    blueprint->areas[0].height = 1;
}

void dspbptk_resize(blueprint_t* blueprint, size_t N) {
    blueprint->numBuildings = N;
    blueprint->buildings = realloc(blueprint->buildings, sizeof(building_t) * N);
}

void dspbptk_building_copy(building_t* dst, const building_t* src, size_t N, size_t index_offset) {
    memcpy(dst, src, sizeof(building_t) * N);
    for (size_t i = 0; i < N; i++) {
        dst[i].index += index_offset;
        if (dst[i].tempOutputObjIdx != OBJ_NULL)
            dst[i].tempOutputObjIdx += index_offset;
        if (dst[i].tempInputObjIdx != OBJ_NULL)
            dst[i].tempInputObjIdx += index_offset;
        if (src[i].numParameters > 0) {
            dst[i].parameters = dspbptk_calloc_parameters(dst[i].numParameters);
            memcpy(dst[i].parameters, src[i].parameters, src[i].numParameters * sizeof(int32_t));
        }
    }
}

void rct_to_sph(const vec4 rct, vec4 sph) {
    sph[2] = 0.0;
    sph[3] = 1.0;
    double x = 0.0;
    double y = 0.0;

    y = asin(rct[2]) * (250.0 / M_PI_2);
    x = acos(rct[1] / sqrt(1.0 - rct[2] * rct[2])) * ((rct[0] >= 0.0) ? (250.0 / M_PI_2) : (-250.0 / M_PI_2));

    // 可能出现除0错误或者超过反三角函数定义域，必须处理这些特殊情况
    sph[1] = isfinite(y) ? y : (rct[2] >= 0.0 ? 250.0 : -250.0);
    sph[0] = isfinite(x) ? x : (rct[1] >= 0.0 ? 0.0 : -500.0);

#ifndef DSPBPTK_NO_WARNING
    if (!isfinite(x) || !isfinite(y))
        fprintf(stderr, "Math warning: %1.15lf,%1.15lf,%1.15lf -> %1.15lf,%1.15lf -> %1.15lf,%1.15lf\n", rct[0], rct[1], rct[2], x, y, sph[0], sph[1]);
#endif
}

void sph_to_rct(const vec4 sph, vec4 rct) {
    rct[2] = sin(sph[1] * (M_PI / 500.0));
    const double r = sqrt(1.0 - rct[2] * rct[2]);
    rct[0] = sin(sph[0] * (M_PI / 500.0)) * r;
    rct[1] = cos(sph[0] * (M_PI / 500.0)) * r;
}

void set_rot_mat(const vec4 rct_vec, mat4x4 rot) {
    rot[1][0] = rct_vec[0];
    rot[1][1] = rct_vec[1];
    rot[1][2] = rct_vec[2];
    rot[2][0] = 0.0;
    rot[2][1] = 0.0;
    rot[2][2] = 1.0;
    vec3_cross(rot[0], rot[1], rot[2]);
    vec3_cross(rot[2], rot[0], rot[1]);
    // 这里的normalize不能去掉，因为cross得到的不是单位向量
    vec3_normalize(rot[0]);
    vec3_normalize(rot[1]);
    vec3_normalize(rot[2]);
}

void dspbptk_building_localOffset_rotation(building_t* building, mat4x4 rot) {
    // z不参与坐标转换计算，单独处理避免精度丢失
    double z_old = building->localOffset[2];
    double z2_old = building->localOffset2[2];

    vec4 rct_offset_old;
    vec4 rct_offset2_old;
    sph_to_rct(building->localOffset, rct_offset_old);
    sph_to_rct(building->localOffset2, rct_offset2_old);

    vec4 rct_offset_new;
    vec4 rct_offset2_new;
    vec3_dot_mat3x3(rct_offset_new, rct_offset_old, rot);
    vec3_dot_mat3x3(rct_offset2_new, rct_offset2_old, rot);

    rct_to_sph(rct_offset_new, building->localOffset);
    rct_to_sph(rct_offset2_new, building->localOffset2);

    building->localOffset[2] = z_old;
    building->localOffset2[2] = z2_old;
}
