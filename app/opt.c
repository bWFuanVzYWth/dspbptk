#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>

#include "libdspbptk.h"

uint64_t get_timestamp(void) {
    struct timespec t;
    clock_gettime(0, &t);
    return (uint64_t)t.tv_sec * 1000000000 + (uint64_t)t.tv_nsec;
}

double d_t(uint64_t t1, uint64_t t0) {
    return (double)(t1 - t0) / 1000000.0;
}

int cmp_building(const void* p_a, const void* p_b) {
    building_t* a = (building_t*)p_a;
    building_t* b = (building_t*)p_b;

    // 优先根据建筑种类排序
    int tmp_itemId = a->itemId - b->itemId;
    if (tmp_itemId != 0)
        return tmp_itemId;
    int tmp_modelIndex = a->modelIndex - b->modelIndex;
    if (tmp_modelIndex != 0)
        return tmp_modelIndex;

    // 其次根据公式排序
    int tmp_recipeId = a->recipeId - b->recipeId;
    if (tmp_recipeId != 0)
        return tmp_recipeId;

    // 然后根据所在纬度区域排序
    int tmp_areaIndex = a->areaIndex - b->areaIndex;
    if (tmp_areaIndex != 0)
        return tmp_areaIndex;

    // 区域也相同时，根据y>x>z的优先级排序
    const double Ky = 256.0;
    const double Kx = 1024.0;
    double score_pos_a = (a->localOffset[1] * Ky + a->localOffset[0]) * Kx + a->localOffset[2];
    double score_pos_b = (b->localOffset[1] * Ky + b->localOffset[0]) * Kx + b->localOffset[2];
    return score_pos_a < score_pos_b ? 1 : -1;
}

double try_round(double x, uint64_t error) {
    return round(x * error) / error;
}

void print_help_doc() {
    printf(
        "Usage:   opt [Options] FileName\n"
        "Example: opt -O2 -f -o final.txt demo.txt\n"
        "\n"
        "Options:\n"
        "f            Force write even new blueprint is larger.\n"
        "h            Show this help doc.\n"
        "o [file]     Output blueprint to [file].\n"
        "O {0123}     Optimization level.\n"
        "\n");
}

int main(int argc, char* argv[]) {
    // dspbptk的错误值
    dspbptk_error_t errorlevel;

    // 检查用户是否输入了文件名
    if (argc <= 1) {
        errorlevel = -1;
        goto error;
    }

    // 声明启动参数
    char* bp_path_i = argv[argc - 1];
    char* bp_path_o = argv[argc - 1];
    i64_t opt_level = 2;
    i64_t force_overwrite = 0;

    // 处理启动参数
    const char* options = "fho:O:";
    char arg;
    while ((arg = getopt(argc, argv, options)) != -1) {
        switch (arg) {
            case 'f': {  // 强制覆写
                force_overwrite = 1;
                break;
            }

            case 'h': {
                errorlevel = 0;
                goto error;
            }

            case 'o': {  // 重定向输出文件
                bp_path_o = optarg;
                break;
            }

            case 'O': {  // 优化级别，数字越大优化级别越高
                sscanf(optarg, "%lld", &opt_level);
                // TODO 这里应该静默吗？
                opt_level = opt_level > 3 ? 3 : opt_level;
                opt_level = opt_level < 0 ? 0 : opt_level;
                break;
            }
        }
    }

    fprintf(stderr, "file_i：%s\n", bp_path_i);
    fprintf(stderr, "file_o：%s\n", bp_path_o);

    // 打开蓝图文件
    FILE* fpi = fopen(bp_path_i, "r");
    if (fpi == NULL) {
        fprintf(stderr, "Error: Cannot read file:\"%s\".\n", bp_path_i);
        errorlevel = -1;
        goto error;
    }

    // 分配字符串内存空间
    char* str_i = (char*)calloc(BLUEPRINT_MAX_LENGTH, sizeof(char));
    char* str_o = (char*)calloc(BLUEPRINT_MAX_LENGTH, sizeof(char));

    // 读取字符串
    fscanf(fpi, "%s", str_i);
    fclose(fpi);

    // 分配并初始化蓝图数据和蓝图编解码器
    blueprint_t bp;
    dspbptk_coder_t coder;
    dspbptk_init_coder(&coder);

    // 蓝图解码
    uint64_t t_dec_0 = get_timestamp();
    errorlevel = blueprint_decode(&coder, &bp, str_i, strlen(str_i));
    uint64_t t_dec_1 = get_timestamp();
    fprintf(stderr, "dec time = %.3lf ms\n", d_t(t_dec_1, t_dec_0));
    if (errorlevel) {
        goto error;
    }

    // 优化：坐标归正
    if (opt_level >= 3) {
        for (uint64_t i = 0; i < bp.numBuildings; i++) {
            bp.buildings[i].localOffset[0] = try_round(bp.buildings[i].localOffset[0], 300);
            bp.buildings[i].localOffset[1] = try_round(bp.buildings[i].localOffset[1], 300);
            bp.buildings[i].localOffset[2] = try_round(bp.buildings[i].localOffset[2], 8);
            bp.buildings[i].localOffset2[0] = try_round(bp.buildings[i].localOffset2[0], 300);
            bp.buildings[i].localOffset2[1] = try_round(bp.buildings[i].localOffset2[1], 300);
            bp.buildings[i].localOffset2[2] = try_round(bp.buildings[i].localOffset2[2], 8);
            bp.buildings[i].yaw = try_round(bp.buildings[i].yaw, 1);
            bp.buildings[i].yaw2 = try_round(bp.buildings[i].yaw2, 1);
            bp.buildings[i].tilt = try_round(bp.buildings[i].tilt, 1);
        }
    }

    // 优化：建筑排序
    if (opt_level >= 2) {
        uint64_t t_opt_0 = get_timestamp();
        qsort(bp.buildings, bp.numBuildings, sizeof(building_t), cmp_building);
        uint64_t t_opt_1 = get_timestamp();
        fprintf(stderr, "opt time = %.3lf ms\n", d_t(t_opt_1, t_opt_0));
    }

    // 蓝图编码
    uint64_t t_enc_0 = get_timestamp();
    errorlevel = blueprint_encode(&coder, &bp, str_o);
    uint64_t t_enc_1 = get_timestamp();
    fprintf(stderr, "enc time = %.3lf ms\n", d_t(t_enc_1, t_enc_0));
    if (errorlevel) {
        goto error;
    }

    // 比较压缩前后的蓝图变化
    size_t strlen_i = strlen(str_i);
    size_t strlen_o = strlen(str_o);
    fprintf(stderr, "strlen_i = %zu\nstrlen_o = %zu (%.3lf%%)\n",
            strlen_i, strlen_o, ((double)strlen_o / (double)strlen_i - 1.0) * 100.0);
    if (strlen_o < strlen_i || force_overwrite) {
        FILE* fpo = fopen(bp_path_o, "w");
        if (fpo == NULL) {
            fprintf(stderr, "Error: Cannot overwrite file:\"%s\".\n", bp_path_o);
            errorlevel = -1;
            goto error;
        }
        fprintf(fpo, "%s", str_o);
        fclose(fpo);
        fprintf(stderr, "Over write blueprint.\n");
    } else {
        fprintf(stderr, "Origin blueprint is better. Nothing Changed.\n");
    }

    // 释放蓝图数据和蓝图编解码器
    dspbptk_free_blueprint(&bp);
    dspbptk_free_coder(&coder);
    // 释放字符串内存空间
    free(str_o);
    free(str_i);

    // 退出程序
    return 0;

error:
    print_help_doc();
    fprintf(stderr, "errorlevel = %d\n", errorlevel);
    return errorlevel;
}
