#ifndef LIBDSBPBTK
#define LIBDSBPBTK

#ifdef __cplusplus
extern "C" {
#endif

#define _USE_MATH_DEFINES

#include <inttypes.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "chromiumbase64/chromiumbase64.h"
#include "libdeflate/libdeflate.h"

#include "md5f.h"
#include "splitmix64.h"
#include "vec.h"

// 可选的宏

// #define DSPBPTK_NO_WARNING
// #define DSPBPTK_NO_ERROR
// #define DSPBPTK_DEBUG

////////////////////////////////////////////////////////////////////////////////
// dspbptk errorlevel
////////////////////////////////////////////////////////////////////////////////

typedef enum {
    no_error = 0,
    out_of_memory,
    not_blueprint,
    blueprint_head_broken,
    blueprint_base64_broken,
    blueprint_gzip_broken,
    blueprint_data_broken,
    blueprint_md5f_broken
} dspbptk_error_t;

////////////////////////////////////////////////////////////////////////////////
// dspbptk defines
////////////////////////////////////////////////////////////////////////////////

#define MD5F_LENGTH 32
#define GAMEVERSION_MAX_LENGTH 4096
#define SHORTDESC_MAX_LENGTH 4096
#define DESC_MAX_LENGTH 4096
#define BLUEPRINT_MAX_LENGTH 134217728  // 128mb. 1048576 * 61 * 3/4 = 85284181.333 < 134217728.

#define OBJ_NULL (-1)

////////////////////////////////////////////////////////////////////////////////
// dspbptk struct
////////////////////////////////////////////////////////////////////////////////

typedef int8_t i8_t;
typedef int16_t i16_t;
typedef int32_t i32_t;
typedef int64_t i64_t;

typedef struct {
    i64_t index;
    i64_t parentIndex;
    i64_t tropicAnchor;
    i64_t areaSegments;
    i64_t anchorLocalOffsetX;
    i64_t anchorLocalOffsetY;
    i64_t width;
    i64_t height;
} area_t;

typedef struct {
    i64_t num;
    i64_t index;
    i64_t areaIndex;
    vec4 localOffset;
    vec4 localOffset2;
    f64_t yaw;
    f64_t yaw2;
    f64_t tilt;
    i64_t itemId;
    i64_t modelIndex;
    i64_t tempOutputObjIdx;
    i64_t tempInputObjIdx;
    i64_t outputToSlot;
    i64_t inputFromSlot;
    i64_t outputFromSlot;
    i64_t inputToSlot;
    i64_t outputOffset;
    i64_t inputOffset;
    i64_t recipeId;
    i64_t filterId;
    size_t numParameters;
    i64_t* parameters;
} building_t;

typedef struct {
    // head
    i64_t layout;
    i64_t icons[5];
    i64_t time;
    char* gameVersion;
    char* shortDesc;
    char* desc;
    // base64
    i64_t version;
    i64_t cursorOffsetX;
    i64_t cursorOffsetY;
    i64_t cursorTargetArea;
    i64_t dragBoxSizeX;
    i64_t dragBoxSizeY;
    i64_t primaryAreaIdx;
    size_t numAreas;
    area_t* areas;
    size_t numBuildings;
    building_t* buildings;
    // md5f
    char* md5f;
} blueprint_t;

typedef struct {
    void* buffer0;                                   // 用于蓝图编码解码
    void* buffer1;                                   // 用于蓝图编码解码
    void* buffer_string;                             // 涉及文件读写时延迟申请
    size_t string_length;                            // 上一次编码/解码的蓝图字符串长度
    struct libdeflate_compressor* p_compressor;      // gzip压缩
    struct libdeflate_decompressor* p_decompressor;  // gzip解压
} dspbptk_coder_t;

////////////////////////////////////////////////////////////////////////////////
// dspbptk function
////////////////////////////////////////////////////////////////////////////////

/**
 * @brief 蓝图解析。将蓝图字符串解析成blueprint_t
 *
 * @param blueprint 解析后的蓝图数据。使用结束后必须调用free_blueprint(blueprint)释放内存。
 * @param string 解析前的蓝图字符串
 * @param string_length 解析前的蓝图字符串的长度，可以strlen(string)
 * @return dspbptk_error_t 错误代码
 */
dspbptk_error_t blueprint_decode(dspbptk_coder_t* coder, blueprint_t* blueprint, const char* string, size_t string_length);

// 同上，但是文件
dspbptk_error_t blueprint_decode_file(dspbptk_coder_t* coder, blueprint_t* blueprint, FILE* fp);

/**
 * @brief 蓝图编码。将blueprint_t编码成蓝图字符串
 *
 * @param blueprint 编码前的蓝图数据
 * @param string 编码后的蓝图字符串
 * @return dspbptk_error_t 错误代码
 */
dspbptk_error_t blueprint_encode(dspbptk_coder_t* coder, const blueprint_t* blueprint, char* string);

// 同上，但是文件
dspbptk_error_t blueprint_encode_file(dspbptk_coder_t* coder, const blueprint_t* blueprint, FILE* fp);

/**
 * @brief 释放blueprint_t结构体中的内存
 *
 * @param blueprint 需要释放内存的结构体
 */
void dspbptk_free_blueprint(blueprint_t* blueprint);

////////////////////////////////////////////////////////////////////////////////
// dspbptk init coder
////////////////////////////////////////////////////////////////////////////////

/**
 * @brief 初始化一个蓝图编码/解码器，请注意多线程不能使用同一个coder
 *
 * @param coder 待初始化的编码/解码器，使用结束后必须调用dspbptk_free_coder(coder)释放内存
 */
void dspbptk_init_coder(dspbptk_coder_t* coder);

////////////////////////////////////////////////////////////////////////////////
// dspbptk free coder
////////////////////////////////////////////////////////////////////////////////

/**
 * @brief 释放coder使用的内存
 *
 * @param coder 待释放内存的编码/解码器
 */
void dspbptk_free_coder(dspbptk_coder_t* coder);

////////////////////////////////////////////////////////////////////////////////
// dspbptk API
////////////////////////////////////////////////////////////////////////////////

typedef struct {
    i64_t id;
    i32_t index;
} index_t;

void dspbptk_blueprint_init(blueprint_t* blueprint);

/**
 * @brief 生成从拓展标准的建筑id到原版标准的建筑index的查找表
 *
 * @param blueprint 使用拓展标准建筑id的蓝图
 * @param id_lut 查找表，假定id_lut有足够的空间
 */
void generate_lut(const blueprint_t* blueprint, index_t* id_lut);

/**
 * @brief 从查找表中获取index
 *
 * @param ObjIdx 拓展标准的建筑id
 * @param id_lut 查找表，需要先使用generate_lut函数自动生成
 * @param BUILDING_NUM 蓝图的建筑总数
 * @return i32_t 原版标准的建筑index
 */
i32_t get_idx(const i64_t* ObjIdx, const index_t* id_lut, size_t BUILDING_NUM);

/**
 * @brief 调整蓝图中的建筑数量
 *
 * @param blueprint 指向需要被调整的蓝图的指针
 * @param N 新的建筑数量
 */
void dspbptk_resize(blueprint_t* blueprint, size_t N);

/**
 * @brief 批量复制建筑，并给这些建筑重新编号
 *
 * @param dst 目标位置
 * @param src 起始位置
 * @param N 从目标位置起，复制N个建筑
 * @param index_offset 给新的建筑的index加上index_offset，用于处理蓝图中的建筑编号
 */
void dspbptk_building_copy(building_t* dst, const building_t* src, size_t N, size_t index_offset);

/**
 * @brief 从三维空间中单位球面上的坐标转换到蓝图中的经纬度坐标
 *
 * @param rct 三维空间中单位球面上的坐标
 * @param sph 蓝图中的经纬度坐标
 */
void rct_to_sph(const vec4 rct, vec4 sph);

/**
 * @brief 从蓝图中的经纬度坐标转换到三维空间中单位球面上的坐标，!!!注意z轴将被丢弃!!!
 *
 * @param sph 蓝图中的经纬度坐标
 * @param rct 三维空间中单位球面上的坐标
 */
void sph_to_rct(const vec4 sph, vec4 rct);

/**
 * @brief 通过目标地点构造旋转矩阵
 *
 * @param vec 目标地点的坐标：三维空间中单位球面上的坐标
 * @param rot 旋转矩阵
 */
void set_rot_mat(const vec4 vec, mat4x4 rot);

/**
 * @brief 根据旋转矩阵在球面上平移一个建筑
 *
 * @param building 待平移的建筑
 * @param rot 旋转矩阵，如果没有的话可以用set_rot_mat(vec, rot)构造
 */
void dspbptk_building_localOffset_rotation(building_t* building, mat4x4 rot);

#ifdef __cplusplus
}
#endif

#endif