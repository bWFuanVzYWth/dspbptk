
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

#define BP_LEN 268435456 // 256mb // TODO 去掉这个宏

    typedef struct {
        uint64_t layout;            // layout，作用未知
        uint64_t icons[5];          // 蓝图图标
        uint64_t time;              // 时间戳
        uint64_t game_version[4];   // 创建蓝图的游戏版本
        size_t raw_len;             // 蓝图数据的长度
        char* short_desc;           // 蓝图简介，注意结尾已经带了','
        void* raw;                  // 指向蓝图数据
    } bp_data_t;

    // TODO 规范命名
    typedef enum {
        _index_bpbd = 0,
        _areaIndex = _index_bpbd + 4, // renamed
        _localOffset_x = _areaIndex + 1,
        _localOffset_y = _localOffset_x + 4,
        _localOffset_z = _localOffset_y + 4,
        _localOffset_x2 = _localOffset_z + 4,
        _localOffset_y2 = _localOffset_x2 + 4,
        _localOffset_z2 = _localOffset_y2 + 4,
        _yaw = _localOffset_z2 + 4,
        _yaw2 = _yaw + 4,
        _itemId = _yaw2 + 4,
        _modelIndex = _itemId + 2,
        _tempOutputObjIdx = _modelIndex + 2,
        _tempInputObjIdx = _tempOutputObjIdx + 4,
        _outputToSlot = _tempInputObjIdx + 4,
        _inputFromSlot = _outputToSlot + 1,
        _outputFromSlot = _inputFromSlot + 1,
        _inputToSlot = _outputFromSlot + 1,
        _outputOffset = _inputToSlot + 1,
        _inputOffset = _outputOffset + 1,
        _recipeId = _inputOffset + 1,
        _filterId = _recipeId + 2,
        _num_bpbd = _filterId + 2, // renamed
        _parameters_bpbd = _num_bpbd + 2 // renamed
    }BlueprintBuilding_offset_t;

    typedef enum {
        _index_area = 0, // renamed
        _parentIndex = _index_area + 1,
        _tropicAnchor = _parentIndex + 1,
        _areaSegments = _tropicAnchor + 2,
        _anchorLocalOffsetX = _areaSegments + 2,
        _anchorLocalOffsetY = _anchorLocalOffsetX + 2,
        _width = _anchorLocalOffsetY + 2,
        _height = _width + 2,
        _next_area = _height + 2
    }area_offset_t;

    typedef enum {
        _version = 0,
        _cursorOffset_x = _version + 4,
        _cursorOffset_y = _cursorOffset_x + 4,
        _cursorTargetArea = _cursorOffset_y + 4,
        _dragBoxSize_x = _cursorTargetArea + 4,
        _dragBoxSize_y = _dragBoxSize_x + 4,
        _primaryAreaIdx = _dragBoxSize_y + 4,
        _num_area = _primaryAreaIdx + 4,  // renamed
        _area_array = _num_area + 1
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
