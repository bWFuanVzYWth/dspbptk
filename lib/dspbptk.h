
#include <inttypes.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>

#include "libdeflate/libdeflate.h"
#include "Turbo-Base64/turbob64.h"
#include "zopfli/zopfli.h"
#include "yyjson/src/yyjson.h"

#include "md5f.h"

#ifndef DSPBPTK
#define DSPBPTK

#ifdef __cplusplus
extern "C" {
#endif

#define BLUEPRINT_MAX_LENGTH 134217728 // 128mb. 1048576 * 61 * 3/4 = 85284181.333 < 134217728.

    typedef struct {
        uint64_t layout;            // layout，作用未知
        uint64_t icons[5];          // 蓝图图标
        uint64_t time;              // 时间戳
        uint64_t gameVersion[4];   // 创建蓝图的游戏版本
        size_t bin_len;             // 蓝图数据的长度
        char* shortDesc;           // 蓝图简介
        size_t area_num;
        size_t building_num;
        void* bin;                  // 指向蓝图头，也是二进制数据流的起始
        void** area;                // 指向每一个区域
        void** building;            // 指向每一个建筑
    } bp_data_t;

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

    // 内部函数

    /**
     * @brief 从文件读取蓝图，不检查蓝图正确性。会给blueprint分配内存，别忘了free(blueprint);
     *
     * @param file_name 待读取的文件名
     * @param p_blueprint 指向蓝图字符串的指针
     * @return size_t 如果成功返回蓝图的尺寸；如果失败返回0
     */
    size_t file_to_blueprint(const char* file_name, char** p_blueprint);

    /**
     * @brief 将蓝图写入文件，不检查蓝图正确性
     *
     * @param file_name 待写入的文件名
     * @param blueprint 蓝图字符串
     * @return int 如果成功返回0；如果失败返回-1
     */
    int blueprint_to_file(const char* file_name, const char* blueprint);

    /**
     * @brief 从蓝图字符串解析其中的数据到bp_data。会给bp_data分配内存，别忘了free_bp_data(&bp_data);
     *
     * @param p_bp_data 指向bp_data的指针
     * @param blueprint 蓝图字符串
     * @return int 解析是否成功
     */
    int blueprint_to_data(bp_data_t* p_bp_data, const char* blueprint);

    /**
     * @brief 将bp_data编码成蓝图字符串
     *
     * @param p_bp_data 指向bp_data的指针
     * @param blueprint 蓝图字符串
     * @return int 编码是否成功
     */
    int data_to_blueprint(const bp_data_t* p_bp_data, char* blueprint);

    int data_to_json(const bp_data_t* p_bp_data, char** json);

    /**
     * @brief 释放bp_data的内存
     *
     * @param p_bp_data 指向bp_data的指针
     */
    void free_bp_data(bp_data_t* p_bp_data);

#ifdef __cplusplus
}
#endif

#endif
