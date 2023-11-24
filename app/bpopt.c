#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#include "../lib/libdspbptk.h"

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
    if(tmp_itemId != 0)
        return tmp_itemId;
    int tmp_modelIndex = a->modelIndex - b->modelIndex;
    if(tmp_modelIndex != 0)
        return tmp_modelIndex;

    // 其次根据公式排序
    int tmp_recipeId = a->recipeId - b->recipeId;
    if(tmp_recipeId != 0)
        return tmp_recipeId;

    // 然后根据所在纬度区域排序
    int tmp_areaIndex = a->areaIndex - b->areaIndex;
    if(tmp_areaIndex != 0)
        return tmp_areaIndex;

    // 区域也相同时，根据y>x>z的优先级排序
    const double Ky = 256.0;
    const double Kx = 1024.0;
    double score_pos_a = (a->localOffset.y * Ky + a->localOffset.x) * Kx + a->localOffset.z;
    double score_pos_b = (b->localOffset.y * Ky + b->localOffset.x) * Kx + b->localOffset.z;
    return score_pos_a < score_pos_b ? 1 : -1;
}

int main(int argc, char* argv[]) {

    // dspbptk的错误值
    dspbptk_error_t errorlevel;

    // 检查用户是否输入了文件名
    if(argc <= 1) {
        fprintf(stderr, "Usage: bpopt FileName\n");
        errorlevel = -1;
        goto error;
    }

    // 打开蓝图文件
    FILE* fpi = fopen(argv[1], "r");
    if(fpi == NULL) {
        fprintf(stderr, "Error: Cannot read file:\"%s\".\n", argv[1]);
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
    errorlevel = blueprint_decode(&coder, &bp, str_i);
    uint64_t t_dec_1 = get_timestamp();
    fprintf(stderr, "dec time = %.3lf ms\n", d_t(t_dec_1, t_dec_0));
    if(errorlevel) {
        goto error;
    }

    // 对建筑按建筑类型排序，有利于进一步压缩，非必要步骤
#ifndef DSPBPTK_DONT_SORT_BUILDING
    uint64_t t_opt_0 = get_timestamp();
    qsort(bp.building, bp.BUILDING_NUM, sizeof(building_t), cmp_building);
    uint64_t t_opt_1 = get_timestamp();
    fprintf(stderr, "opt time = %.3lf ms\n", d_t(t_opt_1, t_opt_0));
#endif

    // 蓝图编码
    uint64_t t_enc_0 = get_timestamp();
    errorlevel = blueprint_encode(&coder, &bp, str_o);
    uint64_t t_enc_1 = get_timestamp();
    fprintf(stderr, "enc time = %.3lf ms\n", d_t(t_enc_1, t_enc_0));
    if(errorlevel) {
        goto error;
    }

    // 比较压缩前后的蓝图变化
    size_t strlen_i = strlen(str_i);
    size_t strlen_o = strlen(str_o);
    fprintf(stderr, "strlen_i = %zu\nstrlen_o = %zu (%.3lf%%)\n",
        strlen_i, strlen_o, ((double)strlen_o / (double)strlen_i - 1.0) * 100.0);
    if(strlen_o < strlen_i) {
        FILE* fpo = fopen(argv[1], "w");
        if(fpo == NULL)
            return -1;
        fprintf(fpo, "%s", str_o);
        fclose(fpo);
        fprintf(stderr, "Over write blueprint.\n");
    }
    else {
        fprintf(stderr, "Origin blueprint is smaller. Nothing Changed.\n");
    }

    // 释放蓝图数据和蓝图编解码器
    dspbptk_free_blueprint(&bp);
    dspbptk_free_coder(&coder);
    // 释放字符串内存空间
    free(str_o);
    free(str_i);

    // 退出程序
    printf("Finish.\n");
    return 0;

error:
    fprintf(stderr, "errorlevel = %d\n", errorlevel);
    return errorlevel;
}