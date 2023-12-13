#ifndef LIBDSBPBTK
#define LIBDSBPBTK

#ifdef __cplusplus
extern "C" {
#endif

#include <stdlib.h>
#include <inttypes.h>
#include <stdio.h>
#include <string.h>

#include "libdeflate/libdeflate.h"
#include "Turbo-Base64/turbob64.h"

#include "md5f.h"

// 可选的宏

// #define DSPBPTK_NO_WARNING
// #define DSPBPTK_NO_ERROR

// #define DSPBPTK_DEBUG

#ifdef DSPBPTK_DEBUG
#define MSG(x) {puts("Message:\t"x);}
#define DBG(x) {printf("Debug:\t"#x"=%"PRId64"\n",(int64_t)x);}
#else
#define MSG(x)
#define DBG(x)
#endif

    ////////////////////////////////////////////////////////////////////////////
    // dspbptk errorlevel
    ////////////////////////////////////////////////////////////////////////////

    typedef enum {
        no_error = 0,

        out_of_memory,
        not_blueprint,
        blueprint_head_broken,
        blueprint_base64_broken,
        blueprint_gzip_broken,
        blueprint_data_broken,
        blueprint_md5f_broken
    }dspbptk_error_t;



    ////////////////////////////////////////////////////////////////////////////
    // dspbptk defines
    ////////////////////////////////////////////////////////////////////////////

#define MD5F_LENGTH 32
#define SHORTDESC_MAX_LENGTH 4096
#define BLUEPRINT_MAX_LENGTH 134217728  // 128mb. 1048576 * 61 * 3/4 = 85284181.333 < 134217728.

#define OBJ_NULL (-1)



    ////////////////////////////////////////////////////////////////////////////
    // dspbptk struct
    ////////////////////////////////////////////////////////////////////////////

    // TODO 检查数据类型是否合理正确

    typedef int8_t i8_t;
    typedef int16_t i16_t;
    typedef int32_t i32_t;
    typedef int64_t i64_t;

    typedef float f32_t;
    typedef double f64_t;

    typedef struct {
        f64_t x;
        f64_t y;
        f64_t z;
        f64_t w;
    }f64x4_t;

    typedef struct {
        i64_t index;
        i64_t parentIndex;
        i64_t tropicAnchor;
        i64_t areaSegments;
        i64_t anchorLocalOffsetX;
        i64_t anchorLocalOffsetY;
        i64_t width;
        i64_t height;
    }area_t;

    typedef struct {
        i64_t index;
        i64_t areaIndex;
        f64x4_t localOffset;
        f64x4_t localOffset2;
        f64_t yaw;
        f64_t yaw2;
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
        size_t num;
        i64_t* parameters;
    }building_t;

    typedef struct {
        // head
        i64_t layout;
        i64_t icons[5];
        i64_t time;
        i64_t gameVersion[4];
        char* shortDesc;
        // base64
        i64_t version;
        i64_t cursorOffset_x;
        i64_t cursorOffset_y;
        i64_t cursorTargetArea;
        i64_t dragBoxSize_x;
        i64_t dragBoxSize_y;
        i64_t primaryAreaIdx;
        size_t AREA_NUM;
        area_t* area;
        size_t BUILDING_NUM;
        building_t* building;
        // md5f
        char* md5f;
    }blueprint_t;

    typedef struct {
        void* buffer0;
        void* buffer1;
        struct libdeflate_compressor* p_compressor;
        struct libdeflate_decompressor* p_decompressor;
    }dspbptk_coder_t;



    ////////////////////////////////////////////////////////////////////////////
    // dspbptk function
    ////////////////////////////////////////////////////////////////////////////

    /**
     * @brief 蓝图解析。将蓝图字符串解析成blueprint_t
     *
     * @param blueprint 解析后的蓝图数据。使用结束后必须调用free_blueprint(blueprint)释放内存。
     * @param string 解析前的蓝图字符串
     * @return dspbptk_error_t 错误代码
     */
    dspbptk_error_t blueprint_decode(dspbptk_coder_t* coder, blueprint_t* blueprint, const char* string);

    /**
     * @brief 蓝图编码。将blueprint_t编码成蓝图字符串
     *
     * @param blueprint 编码前的蓝图数据
     * @param string 编码后的蓝图字符串
     * @return dspbptk_error_t 错误代码
     */
    dspbptk_error_t blueprint_encode(dspbptk_coder_t* coder, const blueprint_t* blueprint, char* string);

    /**
     * @brief 释放blueprint_t结构体中的内存
     *
     * @param blueprint 需要释放内存的结构体
     */
    void dspbptk_free_blueprint(blueprint_t* blueprint);



    ////////////////////////////////////////////////////////////////////////////
    // dspbptk init coder
    ////////////////////////////////////////////////////////////////////////////

    /**
     * @brief 初始化一个蓝图编码/解码器，请注意多线程不能使用同一个coder
     *
     * @param coder 待初始化的编码/解码器，使用结束后必须调用dspbptk_free_coder(coder)释放内存
     */
    void dspbptk_init_coder(dspbptk_coder_t* coder);



    ////////////////////////////////////////////////////////////////////////////
    // dspbptk free coder
    ////////////////////////////////////////////////////////////////////////////

    /**
     * @brief 释放coder使用的内存
     *
     * @param coder 待释放内存的编码/解码器
     */
    void dspbptk_free_coder(dspbptk_coder_t* coder);



    ////////////////////////////////////////////////////////////////////////////
    // dspbptk API
    ////////////////////////////////////////////////////////////////////////////

    typedef struct {
        i64_t id;
        i32_t index;
    }index_t;

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
    i32_t get_idx(i64_t* ObjIdx, index_t* id_lut, size_t BUILDING_NUM);



#ifdef __cplusplus
}
#endif

#endif