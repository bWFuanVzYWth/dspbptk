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

// #define DSPBPTK_DONT_SORT_BUILDING
// #define DSPBPTK_NO_WARNING
// #define DSPBPTK_NO_ERROR // 不建议

#define DSPBPTK_DEBUG

#ifdef DSPBPTK_DEBUG
#define MSG(x) {puts("Message:\t"x);}
#define DBG(x) {printf("Debug:\t\t"#x"=%"PRId64"\n",x);}
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
        size_t PARAMETERS_NUM;
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



    ////////////////////////////////////////////////////////////////////////////
    // dspbptk function
    ////////////////////////////////////////////////////////////////////////////

    dspbptk_error_t blueprint_decode(blueprint_t* blueprint, const char* string);
    dspbptk_error_t blueprint_encode(const blueprint_t* blueprint, char* string);
    void free_blueprint(blueprint_t* blueprint);



    ////////////////////////////////////////////////////////////////////////////
    // dspbptk API
    ////////////////////////////////////////////////////////////////////////////

    // TODO



#ifdef __cplusplus
}
#endif

#endif