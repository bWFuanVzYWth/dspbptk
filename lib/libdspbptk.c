#include "libdspbptk.h"
#include "enum_offset.h"

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
    return tb64v256enc((unsigned char*)in, inlen, (unsigned char*)out);
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
    return tb64v256dec((unsigned char*)in, inlen, (unsigned char*)out);
}

/**
 * @brief 返回base64解码后的准确长度。用于解耦dspbptk与更底层的base64库
 */
size_t base64_declen(const char* base64, size_t base64_length) {
    return tb64declen((unsigned char*)base64, base64_length);
}

/**
 * @brief 通用的gzip压缩。用于解耦dspbptk与更底层的gzip库
 *
 * @param in 压缩前的二进制流
 * @param in_nbytes 压缩前的二进制流长度
 * @param out 压缩后的二进制流
 * @return size_t 压缩后的二进制流长度
 */
size_t gzip_enc(const unsigned char* in, size_t in_nbytes, unsigned char* out) {
    struct libdeflate_compressor* p_compressor = libdeflate_alloc_compressor(12);
    size_t gzip_length = libdeflate_gzip_compress(
        p_compressor, in, in_nbytes, out, BLUEPRINT_MAX_LENGTH);
    libdeflate_free_compressor(p_compressor);
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
size_t gzip_dec(const unsigned char* in, size_t in_nbytes, unsigned char* out) {
    size_t actual_out_nbytes_ret;
    struct libdeflate_decompressor* p_decompressor = libdeflate_alloc_decompressor();
    enum libdeflate_result result = libdeflate_gzip_decompress(
        p_decompressor, in, in_nbytes, out, BLUEPRINT_MAX_LENGTH, &actual_out_nbytes_ret);
    libdeflate_free_decompressor(p_decompressor);
    if(result != LIBDEFLATE_SUCCESS)
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

dspbptk_error_t blueprint_decode(blueprint_t* blueprint, const char* string) {

    // 获取输入的字符串的长度
    const size_t string_length = strlen(string);

    // 检查是不是蓝图
#ifndef DSPBPTK_NO_ERROR
    if(string_length < 10)
        return not_blueprint;
    if(memcmp(string, "BLUEPRINT:", 10) != 0)
        return not_blueprint;
#endif

    // 根据双引号标记字符串
    const char* head = string;
    const char* base64 = strchr(string, (int)'\"') + 1;
    const char* md5f = string + string_length - MD5F_LENGTH;
#ifndef DSPBPTK_NO_ERROR
    if(*(md5f - 1) != '\"')
        return blueprint_md5f_broken;
#endif
    const size_t head_length = (size_t)(base64 - head - 1);
    const size_t base64_length = (size_t)(md5f - base64 - 1);
    DBG(base64_length);

    // 解析md5f(并校验)
#ifndef DSPBPTK_NO_WARNING
    char md5f_check[MD5F_LENGTH + 1] = "\0";
    md5f_str(md5f_check, string, head_length + 1 + base64_length);
    if(memcmp(md5f, md5f_check, MD5F_LENGTH) != 0)
        fprintf(stderr, "Warning: MD5 abnormal!\nthis:\t%s\nactual:\t%s\n", md5f, md5f_check);
#endif
    blueprint->md5f = (char*)calloc(32 + 1, sizeof(char));
    memcpy(blueprint->md5f, md5f, 32);

    // 解析head
    blueprint->shortDesc = (char*)calloc(SHORTDESC_MAX_LENGTH + 1, sizeof(char));
    int argument_count = sscanf(string, "BLUEPRINT:0,%"PRId64",%"PRId64",%"PRId64",%"PRId64",%"PRId64",%"PRId64",0,%"PRId64",%"PRId64".%"PRId64".%"PRId64".%"PRId64",%[^\"]",
        &blueprint->layout,
        &blueprint->icons[0],
        &blueprint->icons[1],
        &blueprint->icons[2],
        &blueprint->icons[3],
        &blueprint->icons[4],
        &blueprint->time,
        &blueprint->gameVersion[0],
        &blueprint->gameVersion[1],
        &blueprint->gameVersion[2],
        &blueprint->gameVersion[3],
        blueprint->shortDesc
    );
#ifndef DSPBPTK_NO_ERROR
    if(argument_count != 12)
        return blueprint_head_broken;
#endif

    // 解析base64
    {
        // base64解码
        size_t gzip_length = base64_declen(base64, base64_length);
        DBG(gzip_length);
        void* gzip = calloc(gzip_length, 1);
    #ifndef DSPBPTK_NO_ERROR
        if(gzip == NULL)
            return out_of_memory;
    #endif
        gzip_length = base64_dec(base64, base64_length, gzip);
        DBG(gzip_length);
    #ifndef DSPBPTK_NO_ERROR
        if(gzip_length <= 0)
            return blueprint_base64_broken;
    #endif

        // gzip解压
        size_t bin_length = gzip_declen(gzip, gzip_length);
        DBG(bin_length);
        void* bin = calloc(bin_length, 1);
    #ifndef DSPBPTK_NO_ERROR
        if(bin == NULL)
            return out_of_memory;
    #endif
        bin_length = gzip_dec(gzip, gzip_length, bin);
        DBG(bin_length);
    #ifndef DSPBP_NO_CHECK
        if(bin_length <= 3)
            return blueprint_gzip_broken;
    #endif

        // 解析二进制流
        {
            // 用于操作二进制流的指针
            void* ptr_bin = bin;

            // 解析二进制流的头
        #define BIN_HEAD_DECODE(name, type)\
            blueprint->name = (i64_t)*((type*)(ptr_bin + bin_offset_##name));
            BIN_HEAD_DECODE(version, i32_t);
            BIN_HEAD_DECODE(cursorOffset_x, i32_t);
            BIN_HEAD_DECODE(cursorOffset_y, i32_t);
            BIN_HEAD_DECODE(cursorTargetArea, i32_t);
            BIN_HEAD_DECODE(dragBoxSize_x, i32_t);
            BIN_HEAD_DECODE(dragBoxSize_y, i32_t);
            BIN_HEAD_DECODE(primaryAreaIdx, i32_t);

            // 解析区域数量
            const size_t AREA_NUM = (size_t) * ((i8_t*)(ptr_bin + BIN_OFFSET_AREA_NUM));
            blueprint->AREA_NUM = AREA_NUM;
            blueprint->area = (area_t*)calloc(AREA_NUM, sizeof(area_t));
        #ifndef DSPBP_NO_CHECK
            if(blueprint->area == NULL)
                return out_of_memory;
        #endif
            DBG(AREA_NUM);

            // 解析区域数组
            ptr_bin += BIN_OFFSET_AREA_ARRAY;
            for(size_t i = 0; i < AREA_NUM; i++) {
            #define AREA_DECODE(name, type)\
                blueprint->area[i].name = (i64_t)*((type*)(ptr_bin + area_offset_##name));
                AREA_DECODE(index, i8_t);
                AREA_DECODE(parentIndex, i8_t);
                AREA_DECODE(tropicAnchor, i16_t);
                AREA_DECODE(areaSegments, i16_t);
                AREA_DECODE(anchorLocalOffsetX, i16_t);
                AREA_DECODE(anchorLocalOffsetY, i16_t);
                AREA_DECODE(width, i16_t);
                AREA_DECODE(height, i16_t);
                ptr_bin += AREA_OFFSET_AREA_NEXT;
            }

            // 解析建筑数量
            const size_t BUILDING_NUM = (size_t) * ((i32_t*)(ptr_bin));
            blueprint->BUILDING_NUM = BUILDING_NUM;
            blueprint->building = (building_t*)calloc(BUILDING_NUM, sizeof(building_t));
        #ifndef DSPBP_NO_CHECK
            if(blueprint->building == NULL)
                return out_of_memory;
        #endif
            DBG(BUILDING_NUM);

            // 解析建筑数组
            ptr_bin += sizeof(int32_t);
            for(size_t i = 0; i < BUILDING_NUM; i++) {
            #define BUILDING_DECODE(name, type)\
                blueprint->building[i].name = (i64_t)*((type*)(ptr_bin + building_offset_##name));
                BUILDING_DECODE(index, i32_t);
                BUILDING_DECODE(areaIndex, i8_t);
                // 把建筑坐标转换成齐次坐标
                blueprint->building[i].localOffset.x = (f64_t) * ((f32_t*)(ptr_bin + building_offset_localOffset_x));
                blueprint->building[i].localOffset.y = (f64_t) * ((f32_t*)(ptr_bin + building_offset_localOffset_y));
                blueprint->building[i].localOffset.z = (f64_t) * ((f32_t*)(ptr_bin + building_offset_localOffset_z));
                blueprint->building[i].localOffset.w = (f64_t)1.0;
                blueprint->building[i].localOffset2.x = (f64_t) * ((f32_t*)(ptr_bin + building_offset_localOffset_x2));
                blueprint->building[i].localOffset2.y = (f64_t) * ((f32_t*)(ptr_bin + building_offset_localOffset_y2));
                blueprint->building[i].localOffset2.z = (f64_t) * ((f32_t*)(ptr_bin + building_offset_localOffset_z2));
                blueprint->building[i].localOffset2.w = (f64_t)1.0;
                blueprint->building[i].yaw = (f64_t) * ((f32_t*)(ptr_bin + building_offset_yaw));
                blueprint->building[i].yaw2 = (f64_t) * ((f32_t*)(ptr_bin + building_offset_yaw2));
                BUILDING_DECODE(itemId, i16_t);
                BUILDING_DECODE(modelIndex, i16_t);
                BUILDING_DECODE(tempOutputObjIdx, i32_t);
                BUILDING_DECODE(tempInputObjIdx, i32_t);
                BUILDING_DECODE(outputToSlot, i8_t);
                BUILDING_DECODE(inputFromSlot, i8_t);
                BUILDING_DECODE(outputFromSlot, i8_t);
                BUILDING_DECODE(inputToSlot, i8_t);
                BUILDING_DECODE(outputOffset, i8_t);
                BUILDING_DECODE(inputOffset, i8_t);
                BUILDING_DECODE(recipeId, i16_t);
                BUILDING_DECODE(filterId, i16_t);

                // DBG(blueprint->building[i].itemId);

                // 解析建筑的参数列表长度
                const size_t PARAMETERS_NUM = (i64_t) * ((i16_t*)(ptr_bin + building_offset_num));
                blueprint->building[i].num = PARAMETERS_NUM;

                // 解析建筑的参数列表
                if(PARAMETERS_NUM > 0) {
                    blueprint->building[i].parameters = (i64_t*)calloc(PARAMETERS_NUM, sizeof(i64_t));
                #ifndef DSPBP_NO_CHECK
                    if(blueprint->building[i].parameters == NULL)
                        return out_of_memory;
                #endif
                }
                else {
                    blueprint->building[i].parameters = NULL;
                }
                ptr_bin += building_offset_parameters;
                for(size_t j = 0; j < PARAMETERS_NUM; j++)
                    blueprint->building[i].parameters[j] = (i64_t) * ((i32_t*)(ptr_bin + j * sizeof(i32_t)));
                ptr_bin += PARAMETERS_NUM * sizeof(i32_t);
            }
        }

        free(gzip);
        free(bin);
    }

    return no_error;
}



////////////////////////////////////////////////////////////////////////////////
// dspbptk encode
////////////////////////////////////////////////////////////////////////////////

typedef struct {
    i64_t id;
    i64_t index;
}index_t;

int cmp_building(const void* p_a, const void* p_b) {
    building_t* a = (building_t*)p_a;
    building_t* b = (building_t*)p_b;
    return a->itemId - b->itemId;
}

int cmp_id(const void* p_a, const void* p_b) {
    index_t* a = ((index_t*)p_a);
    index_t* b = ((index_t*)p_b);
    return a->id - b->id;
}

void re_index(i64_t* ObjIdx, index_t* id_lut, size_t BUILDING_NUM) {
    if(*ObjIdx == OBJ_NULL)
        return;
    index_t* p_id = bsearch(ObjIdx, id_lut, BUILDING_NUM, sizeof(index_t), cmp_id);
    if(p_id == NULL) {
    #ifndef DSPBPTK_NO_WARNING
        fprintf(stderr, "Warning: index %"PRId64" no found! Reindex index to OBJ_NULL(-1).\n", *ObjIdx);
    #endif
        * ObjIdx = OBJ_NULL;
    }
    else {
        *ObjIdx = p_id->index;
    }
}

dspbptk_error_t blueprint_encode(const blueprint_t* blueprint, char* string) {

    // 申请内存，初始化用于操作的两个指针
    void* bin = calloc(BLUEPRINT_MAX_LENGTH, 1);
    void* gzip = calloc(BLUEPRINT_MAX_LENGTH, 1);
    char* ptr_str = string;
    void* ptr_bin = bin;

    // 输出head
    sprintf(string, "BLUEPRINT:0,%"PRId64",%"PRId64",%"PRId64",%"PRId64",%"PRId64",%"PRId64",0,%"PRId64",%"PRId64".%"PRId64".%"PRId64".%"PRId64",%s\"",
        blueprint->layout,
        blueprint->icons[0],
        blueprint->icons[1],
        blueprint->icons[2],
        blueprint->icons[3],
        blueprint->icons[4],
        blueprint->time,
        blueprint->gameVersion[0],
        blueprint->gameVersion[1],
        blueprint->gameVersion[2],
        blueprint->gameVersion[3],
        blueprint->shortDesc
    );
    size_t head_length = strlen(string) - 1;
    ptr_str += head_length + 1;

    // 编码bin head
#define BIN_HEAD_ENCODE(name, type)\
    *((type*)(ptr_bin + bin_offset_##name)) = (type)blueprint->name;
    BIN_HEAD_ENCODE(version, i32_t);
    BIN_HEAD_ENCODE(cursorOffset_x, i32_t);
    BIN_HEAD_ENCODE(cursorOffset_y, i32_t);
    BIN_HEAD_ENCODE(cursorTargetArea, i32_t);
    BIN_HEAD_ENCODE(dragBoxSize_x, i32_t);
    BIN_HEAD_ENCODE(dragBoxSize_y, i32_t);
    BIN_HEAD_ENCODE(primaryAreaIdx, i32_t);


    // 编码区域总数
    *((i8_t*)(ptr_bin + BIN_OFFSET_AREA_NUM)) = (i8_t)blueprint->AREA_NUM;
    DBG(*((i8_t*)(ptr_bin + BIN_OFFSET_AREA_NUM)));

    // 编码区域数组
    ptr_bin += BIN_OFFSET_AREA_ARRAY;
    for(size_t i = 0; i < blueprint->AREA_NUM; i++) {
    #define AREA_ENCODE(name, type)\
        *((type*)(ptr_bin + area_offset_##name)) = (type)blueprint->area[i].name;
        AREA_ENCODE(index, i8_t);
        AREA_ENCODE(parentIndex, i8_t);
        AREA_ENCODE(tropicAnchor, i16_t);
        AREA_ENCODE(areaSegments, i16_t);
        AREA_ENCODE(anchorLocalOffsetX, i16_t);
        AREA_ENCODE(anchorLocalOffsetY, i16_t);
        AREA_ENCODE(width, i16_t);
        AREA_ENCODE(height, i16_t);
        ptr_bin += AREA_OFFSET_AREA_NEXT;
    }

    // 编码建筑总数
    *((i32_t*)(ptr_bin)) = (i32_t)blueprint->BUILDING_NUM;
    DBG(*((i32_t*)(ptr_bin)));

#ifndef DSPBPTK_DONT_SORT_BUILDING
    // 对建筑按建筑类型排序，有利于进一步压缩，非必要步骤
    qsort(blueprint->building, blueprint->BUILDING_NUM, sizeof(building_t), cmp_building);
#endif

    // 重新生成index
    index_t* id_lut = (index_t*)calloc(blueprint->BUILDING_NUM, sizeof(index_t));
    for(size_t i = 0; i < blueprint->BUILDING_NUM; i++) {
        id_lut[i].id = blueprint->building[i].index;
        id_lut[i].index = i;
    }
    qsort(id_lut, blueprint->BUILDING_NUM, sizeof(index_t), cmp_id);
    for(size_t i = 0; i < blueprint->BUILDING_NUM; i++) {
        re_index(&blueprint->building[i].index, id_lut, blueprint->BUILDING_NUM);
        re_index(&blueprint->building[i].tempOutputObjIdx, id_lut, blueprint->BUILDING_NUM);
        re_index(&blueprint->building[i].tempInputObjIdx, id_lut, blueprint->BUILDING_NUM);
    }

    // 编码建筑数组
    ptr_bin += sizeof(i32_t);
    for(size_t i = 0; i < blueprint->BUILDING_NUM; i++) {
    #define BUILDING_ENCODE(name, type)\
        {*((type*)(ptr_bin + building_offset_##name)) = (type)blueprint->building[i].name;}
        BUILDING_ENCODE(index, i32_t);
        BUILDING_ENCODE(areaIndex, i8_t);
        f64_t w = blueprint->building[i].localOffset.w;
        *((f32_t*)(ptr_bin + building_offset_localOffset_x)) = (f32_t)(blueprint->building[i].localOffset.x / w);
        *((f32_t*)(ptr_bin + building_offset_localOffset_y)) = (f32_t)(blueprint->building[i].localOffset.y / w);
        *((f32_t*)(ptr_bin + building_offset_localOffset_z)) = (f32_t)(blueprint->building[i].localOffset.z / w);
        f64_t w2 = blueprint->building[i].localOffset2.w;
        *((f32_t*)(ptr_bin + building_offset_localOffset_x2)) = (f32_t)(blueprint->building[i].localOffset2.x / w2);
        *((f32_t*)(ptr_bin + building_offset_localOffset_y2)) = (f32_t)(blueprint->building[i].localOffset2.y / w2);
        *((f32_t*)(ptr_bin + building_offset_localOffset_z2)) = (f32_t)(blueprint->building[i].localOffset2.z / w2);
        *((f32_t*)(ptr_bin + building_offset_yaw)) = (f32_t)(blueprint->building[i].yaw);
        *((f32_t*)(ptr_bin + building_offset_yaw2)) = (f32_t)(blueprint->building[i].yaw2);
        BUILDING_ENCODE(itemId, i16_t);
        BUILDING_ENCODE(modelIndex, i16_t);
        BUILDING_ENCODE(tempOutputObjIdx, i32_t);
        BUILDING_ENCODE(tempInputObjIdx, i32_t);
        BUILDING_ENCODE(outputToSlot, i8_t);
        BUILDING_ENCODE(inputFromSlot, i8_t);
        BUILDING_ENCODE(outputFromSlot, i8_t);
        BUILDING_ENCODE(inputToSlot, i8_t);
        BUILDING_ENCODE(outputOffset, i8_t);
        BUILDING_ENCODE(inputOffset, i8_t);
        BUILDING_ENCODE(recipeId, i16_t);
        BUILDING_ENCODE(filterId, i16_t);

        // 编码建筑的参数列表长度
        BUILDING_ENCODE(num, i16_t);

        // 编码建筑的参数列表
        ptr_bin += building_offset_parameters;
        for(size_t j = 0; j < blueprint->building[i].num; j++) {
            *((i32_t*)(ptr_bin + sizeof(i32_t) * j)) = (i32_t)blueprint->building[i].parameters[j];
        }
        ptr_bin += sizeof(i32_t) * blueprint->building[i].num;
    }

    // 计算二进制流长度
    size_t bin_length = (size_t)(ptr_bin - bin);
    size_t gzip_length = gzip_enc(bin, bin_length, gzip);
    size_t base64_length = base64_enc(gzip, gzip_length, ptr_str);

    // 计算md5f
    char md5f_hex[MD5F_LENGTH + 1] = "\0";
    md5f_str(md5f_hex, string, head_length + 1 + base64_length);
    ptr_str += base64_length;
    sprintf(ptr_str, "\"%s", md5f_hex);

    free(id_lut);
    free(bin);
    free(gzip);
    return no_error;
}

void free_blueprint(blueprint_t* blueprint) {
    free(blueprint->shortDesc);
    free(blueprint->md5f);
    free(blueprint->area);
    for(size_t i = 0; i < blueprint->BUILDING_NUM; i++) {
        if(blueprint->building[i].num > 0)
            free(blueprint->building[i].parameters);
    }
    free(blueprint->building);
}