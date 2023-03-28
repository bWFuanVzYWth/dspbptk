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

#include "md5f.h"

#define BLUEPRINT_MAX_LENGTH 134217728  // 128mb. 1048576 * 61 * 3/4 = 85284181.333 < 134217728.

// 可选的宏

// #define DSPBPTK_DONT_SORT_BUILDING
// #define DSPBPTK_NO_WARNING

    typedef enum {
        error_arg = -1,

        no_error = 0,

        file_no_found,
        cannot_write,
        out_of_memory,
        not_a_blueprint,
        blueprint_head_broken,
        blueprint_base64_broken,
        blueprint_gzip_broken,
        blueprint_data_broken
    }dspbptk_err_t;

    typedef struct {
        uint64_t layout;            // layout，作用未知
        uint64_t icons[5];          // 蓝图图标
        uint64_t time;              // 时间
        uint64_t gameVersion[4];    // 创建蓝图的游戏版本
        char* shortDesc;            // 蓝图简介
        size_t area_num;            // 区域总数
        size_t building_num;        // 建筑总数
        void* bin;                  // 指向二进制流的头部
        void** area;                // 指向每一个区域
        void** building;            // 指向每一个建筑
    } bp_data_t;



    // API

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
     * @return dspbptk_err_t 错误类型
     */
    dspbptk_err_t blueprint_to_file(const char* file_name, const char* blueprint);

    /**
     * @brief 从蓝图字符串解析其中的数据到bp_data。会给bp_data分配内存，别忘了free_bp_data(&bp_data);
     *
     * @param p_bp_data 指向bp_data的指针
     * @param blueprint 蓝图字符串
     * @return dspbptk_err_t 错误类型：注意只检查数据结构是否合理，不检查数据是否合法
     */
    dspbptk_err_t blueprint_to_data(bp_data_t* p_bp_data, const char* blueprint);

    /**
     * @brief 将bp_data编码成蓝图字符串
     *
     * @param p_bp_data 指向bp_data的指针
     * @param blueprint 蓝图字符串
     * @return dspbptk_err_t 错误类型
     */
    dspbptk_err_t data_to_blueprint(const bp_data_t* p_bp_data, char* blueprint);

    /**
     * @brief 释放bp_data的内存
     *
     * @param p_bp_data 指向bp_data的指针
     */
    void free_bp_data(bp_data_t* p_bp_data);



    // 懒所以只加常用的，有需要可以自己添加（顺便pr）

    size_t get_area_num(void* p_area_num);
    void set_area_num(void* p_area_num, size_t num);

    size_t get_building_num(void* p_building_num);
    void set_building_num(void* p_building_num, size_t num);

    size_t get_building_size(void* p_building);

    int16_t get_building_itemID(void* p_building);
    void set_building_itemID(void* p_building, int16_t itemID);

    int32_t get_building_index(void* p_building);
    void set_building_index(void* p_building, int32_t index);

    void set_building_tempOutputObjIdx(void* p_building, int32_t index);
    void set_building_tempInputObjIdx(void* p_building, int32_t index);

    void get_building_pos1(void* p_building, double pos1[3]);
    void set_building_pos1(void* p_building, double pos1[3]);
    void get_building_pos2(void* p_building, double pos2[3]);
    void set_building_pos2(void* p_building, double pos2[3]);

    int16_t get_building_parameters_num(void* p_building);

    void set_building_parameter(void* p_building, size_t n, int32_t parameter);

#ifdef __cplusplus
}
#endif

#endif
