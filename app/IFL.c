#include <float.h>
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

void print_help_doc() {
    printf(
        "Usage:   IFL options...\n"
        "Example: IFL -i unit.txt -o final.txt -l list.txt -N 130\n"
        "\n"
        "Options:\n"
        "h             Show this help doc.\n"
        "i<file>       Input file(blueprint).\n"
        "l<file>       Coordinate List File.\n"
        "N[num]        Num of Coordinate.\n"
        "o<file>       Output file(blueprint).\n"
        "\n");
}

int main(int argc, char* argv[]) {
    // dspbptk的错误值
    dspbptk_error_t errorlevel;

    if (argc <= 1) {
        errorlevel = -1;
        goto error;
    }

    // 启动参数
    i64_t num_list = 0;
    char* bp_path_i = NULL;
    char* bp_path_o = NULL;
    char* list_path = NULL;

    // 处理启动参数
    const char* options = "hi:l:N:o:";

    char arg;

    while ((arg = getopt(argc, argv, options)) != -1) {
        switch (arg) {
            case 'h': {
                errorlevel = 0;
                goto error;
            }

            case 'i': {
                bp_path_i = optarg;
                break;
            }

            case 'l': {
                list_path = optarg;
                break;
            }

            case 'N': {
                sscanf(optarg, "%lld", &num_list);
                if (num_list < 1) {
                    fprintf(stderr, "非法参数N=%lld", num_list);
                    errorlevel = -1;
                    goto error;
                }
                break;
            }
            case 'o': {
                bp_path_o = optarg;
                break;
            }
        }
    }

    // 读取坐标列表
    FILE* fpl = fopen(list_path, "r");
    if (fpl == NULL) {
        fprintf(stderr, "Error: Cannot read file:\"%s\".\n", list_path);
        errorlevel = -1;
        goto error;
    }
    vec4* pos_list = (vec4*)calloc(num_list, sizeof(vec4));
    for (i64_t i = 0; i < num_list; i++) {
        fscanf(fpl, "%lf", &pos_list[i][0]);
        fscanf(fpl, "%lf", &pos_list[i][1]);
        fscanf(fpl, "%lf", &pos_list[i][2]);
    }
    fclose(fpl);

    // 分配并初始化蓝图数据和蓝图编解码器
    blueprint_t bp;
    dspbptk_coder_t coder;
    dspbptk_init_coder(&coder);

    // 蓝图解码
    FILE* fpi = fopen(bp_path_i, "r");
    if (fpi == NULL) {
        fprintf(stderr, "Error: Cannot read file:\"%s\".\n", bp_path_i);
        errorlevel = -1;
        goto error;
    }
    uint64_t t_dec_0 = get_timestamp();
    errorlevel = blueprint_decode_file(&coder, &bp, fpi);
    size_t strlen_i = coder.string_length;
    uint64_t t_dec_1 = get_timestamp();
    fprintf(stderr, "dec time = %.3lf ms\n", d_t(t_dec_1, t_dec_0));
    fclose(fpi);
    if (errorlevel) {
        goto error;
    }

    uint64_t t_edt_0 = get_timestamp();

    // 调整蓝图大小
    i64_t old_numBuildings = bp.numBuildings;
    dspbptk_resize(&bp, bp.numBuildings * num_list);

    // 蓝图处理
    for (i64_t i = 1; i < num_list; i++) {
        i64_t index_base = i * old_numBuildings;
        dspbptk_building_copy(&bp.buildings[index_base], &bp.buildings[0], old_numBuildings, index_base);
    }

    for (i64_t i = 0; i < num_list; i++) {
        i64_t index_base = i * old_numBuildings;
        mat4x4 rot = {{0.0}};
        set_rot_mat(pos_list[i], rot);
        for (int j = index_base; j < index_base + old_numBuildings; j++) {
            dspbptk_building_localOffset_rotation(&bp.buildings[j], rot);
        }
    }

    uint64_t t_edt_1 = get_timestamp();
    fprintf(stderr, "edt time = %.3lf ms\n", d_t(t_edt_1, t_edt_0));

    // 蓝图编码
    FILE* fpo = fopen(bp_path_o, "w");
    if (fpo == NULL) {
        fprintf(stderr, "Error: Cannot overwrite file:\"%s\".\n", bp_path_i);
        errorlevel = -1;
        goto error;
    }
    uint64_t t_enc_0 = get_timestamp();
    errorlevel = blueprint_encode_file(&coder, &bp, fpo);
    size_t strlen_o = coder.string_length;
    uint64_t t_enc_1 = get_timestamp();
    fprintf(stderr, "enc time = %.3lf ms\n", d_t(t_enc_1, t_enc_0));
    fclose(fpo);
    if (errorlevel) {
        goto error;
    }

    // 比较处理前后的蓝图变化
    fprintf(stderr, "strlen_i = %zu\nstrlen_o = %zu (%.3lf%%)\n",
            strlen_i, strlen_o, ((double)strlen_o / (double)strlen_i - 1.0) * 100.0);

    // 释放蓝图数据和蓝图编解码器
    dspbptk_free_blueprint(&bp);
    dspbptk_free_coder(&coder);

    // 释放坐标列表
    free(pos_list);

    // 退出程序
    fprintf(stderr, "Finish.\n");
    return 0;

error:
    print_help_doc();
    fprintf(stderr, "errorlevel = %d\n", errorlevel);
    return errorlevel;
}
