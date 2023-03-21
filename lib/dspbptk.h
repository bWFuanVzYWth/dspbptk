#ifndef DSPBPTK
#define DSPBPTK

#ifdef __cplusplus
extern "C" {
#endif

#include <inttypes.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>

#include "libdeflate/libdeflate.h"
#include "Turbo-Base64/turbob64.h"
#include "zopfli-KrzYmod/src/zopfli/zopfli.h"
#include "yyjson/src/yyjson.h"

#include "md5f.h"

#define BLUEPRINT_MAX_LENGTH 134217728 // 128mb. 1048576 * 61 * 3/4 = 85284181.333 < 134217728.

    typedef struct {
        uint64_t layout;            // layout，作用未知
        uint64_t icons[5];          // 蓝图图标
        uint64_t time;              // 时间戳
        uint64_t gameVersion[4];   // 创建蓝图的游戏版本
        char* shortDesc;           // 蓝图简介
        size_t area_num;
        size_t building_num;
        void* bin;                  // 指向蓝图头，也是二进制数据流的起始
        void** area;                // 指向每一个区域
        void** building;            // 指向每一个建筑
    } bp_data_t;

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
