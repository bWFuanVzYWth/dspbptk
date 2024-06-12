#include <omp.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "libdspbptk.h"

#define SIZE_CACHE_CHROMOSOME 4096
#define CHROMOSOME_LENGTH 64

// 枚举所有结构，按建筑的itemId顺序
typedef enum {
    assembler,
    // refine,
    // collider,
    smelter,
    // chemical,
    // lab,
    MODULE_COUNT
} module_enum_t;

const char* MODULE_PATH[MODULE_COUNT] = {
    ".\\module\\unit_assembler_1i1o.txt",
    // "module\\unit_refine_1i1o.txt",
    // "module\\unit_collider_1i1o.txt",
    ".\\module\\unit_smelter_1i1o.txt",
    // "module\\unit_chemical_1i1o.txt",
    // "module\\unit_lab_1i1o.txt",
};

typedef struct {
    double dx;
    double dy;
    double area;
    double pow2_half_dx;
    blueprint_t blueprint;
} module_t;

size_t read_dspbptk_module(module_t* module) {
    size_t argc = sscanf(module->blueprint.desc, "dspbptkmodule_-x_%lf_-y_%lf", &module->dx, &module->dy);
    return argc;
}

void init_module(module_t* module, dspbptk_coder_t* coder) {
    for (module_enum_t typ = 0; typ < MODULE_COUNT; typ++) {
        // 读取蓝图
        FILE* fp = fopen(MODULE_PATH[typ], "r");
        dspbptk_error_t errorlevel = blueprint_decode_file(coder, &module[typ].blueprint, fp);
        fclose(fp);
        // 从简介读取模块信息
        size_t argc = read_dspbptk_module(&module[typ]);
        if (argc == 2) {
            fprintf(stderr, "Registered dspbptk module: %s\n", MODULE_PATH[typ]);
        } else {
            fprintf(stderr, "Error in registering dspbptk module \"%s\": %s\n", MODULE_PATH[typ], module[typ].blueprint.desc);
        }
    }

    for (module_enum_t typ = 0; typ < MODULE_COUNT; typ++) {
        module[typ].pow2_half_dx = 0.25 * module[typ].dx * module[typ].dx;
        module[typ].area = module[typ].dx * module[typ].dy;
    }
}

/**
 * @brief 计算一个结构因纬度导致变形后的y跨度，单位格
 *
 * @param pow2_half_dx 预计算参数，pow2_half_dx = (dx/2)^2
 * @param dy 结构在赤道时的y跨度
 * @param row_y_min 当前行y最小值点
 * @return double 结构变形后的y跨度
 */
double row_height_direct(double pow2_half_dx, double dy, double row_y_min) {
    // 计算当前位置距离地轴的半径长度
    double r_row_y_min = cos(row_y_min * (M_PI / 500.0)) * (500.0 / M_PI);
    // 计算y跨度补偿值
    // TODO 这里使用最保守的补偿方法，或许可以考虑进一步优化
    double y_corrected = r_row_y_min - sqrt(r_row_y_min * r_row_y_min - pow2_half_dx);
    return dy + y_corrected;
}

/**
 * @brief 估算在一个蓝图中，某种结构最多能放几个
 *
 * @param max_module_count 计算结果：每个模块最多放几个
 * @param module 模块的信息
 * @param need 每种模块的需求比例
 * @param max_x_span 允许的最大x跨度
 * @param max_y_span 允许的最大y跨度
 */
void compute_max_module_count(size_t* max_module_count, module_t* module, double* need, double max_x_span, double max_y_span) {
    double area_max = (max_x_span * (500.0 / M_PI)) /*扇面面积*/ * sin(max_y_span * (M_PI / 500.0)) /*低纬度占比*/;
    double area_module_sum = 0.0;
    for (module_enum_t typ = 0; typ < MODULE_COUNT; typ++) {
        area_module_sum += module[typ].area * need[typ];
    }
    double k = area_max / area_module_sum;
    for (module_enum_t typ = 0; typ < MODULE_COUNT; typ++) {
        max_module_count[typ] = (size_t)ceil(k * need[typ]);
    }
}

/**
 * @brief 估算在特定纬度的行最多能放下多少特定结构
 *
 * @param row_y_max 当前行y坐标最大处的值
 * @param dx 模块尺寸x
 * @param max_x_span 整个蓝图允许的赤道处最大x跨度
 * @return size_t 最多能放下多少特定结构
 */
size_t compute_row_module_count(double row_y_max, double dx, double max_x_span) {
    return (size_t)(cos(row_y_max * (M_PI / 500.0)) * (max_x_span / dx));
}

/**
 * @brief 客观评分，只考虑密度
 *
 * @param cache_module_sum 缓存当前染色体表达出排列的各种模块总数，这个列表在search()中被维护
 * @param need 对不同模块的需求量
 * @param row_y_max 当前行y坐标最大处的值
 * @param max_y_span 整个蓝图允许的最大y跨度
 * @return double 评分，正比于理论最大产能，越高越好
 */
double objective_score(size_t cache_module_sum[MODULE_COUNT], const double need[MODULE_COUNT], double row_y_max, double max_y_span) {
    // y超标的直接判零分，拍不下去==产能为零
    if (row_y_max > max_y_span)
        return 0.0;
    double min_score = cache_module_sum[0] / need[0];
    // 找出最缺的建筑作为分数
    for (module_enum_t typ = 1; typ < MODULE_COUNT; typ++) {
        double tmp_objective_score = cache_module_sum[typ] / need[typ];
        if (tmp_objective_score < min_score)
            min_score = tmp_objective_score;
    }
    return min_score;
}

/**
 * @brief 主观评分，这个随便写都行不影响搜索
 *
 * @param chromosome 染色体，可以表达一个唯一的建筑排列（遗传算法那个）
 * @param length 染色体的有效长度
 * @return size_t 主观评分，正比于排列的复杂度，越低越简洁（主观上越好看）
 */
size_t subjective_score(const module_enum_t chromosome[CHROMOSOME_LENGTH], size_t length) {
    size_t score = 0;
    for (size_t idx = 1; idx < length; idx++) {
        if (chromosome[idx - 1] != chromosome[idx])
            score++;
    }
    return score;
}

void update_row_data(double* row_max_y, size_t* row_module_num, size_t* module_sum, size_t idx, const module_enum_t typ, const module_t module[MODULE_COUNT], const double max_x_span) {
    row_max_y[idx] = row_max_y[idx - 1] + row_height_direct(module[typ].pow2_half_dx, module[typ].dy, row_max_y[idx - 1]);
    row_module_num[idx] = compute_row_module_count(row_max_y[idx], module[typ].dx, max_x_span);
    module_sum[typ] += row_module_num[idx];
}

void print_detail(double score, size_t score2, module_enum_t* chromosome, size_t length, size_t cache_module_sum[MODULE_COUNT]) {
    printf("score = (%lf, %lld)\tchromosome = ", score, score2);
    for (size_t idx = 0; idx < length; idx++)
        printf("%d", chromosome[idx]);
    printf("\tmodule_sum = ");
    for (size_t typ = 0; typ < MODULE_COUNT; typ++)
        printf("%lld,", cache_module_sum[typ]);
    putchar('\n');
}

void output_blueprint(dspbptk_coder_t* coder, const module_t module[MODULE_COUNT], const module_enum_t* chromosome, size_t length, double max_x_span, double score, size_t score2) {
    // 声明变量，申请内存，或者一些统计数据
    blueprint_t blueprint;
    dspbptk_blueprint_init(&blueprint);

    // 注意这里的length+1是了为了读-1的下标，不能去掉
    double _row_max_y[CHROMOSOME_LENGTH + 1] = {0.0};
    size_t _row_module_num[CHROMOSOME_LENGTH + 1] = {0};
    size_t module_sum[MODULE_COUNT] = {0};
    double* row_max_y = _row_max_y + 1;
    size_t* row_module_num = _row_module_num + 1;

    // 更新行数据
    for (int idx = 0; idx < length; idx++) {
        update_row_data(row_max_y, row_module_num, module_sum, idx, chromosome[idx], module, max_x_span);
    }

    // 计算这些模块等价多少建筑
    size_t building_count = 0;
    for (module_enum_t typ = 0; typ < MODULE_COUNT; typ++) {
        building_count += module[typ].blueprint.numBuildings * module_sum[typ];
    }

    // 生成蓝图
    dspbptk_resize(&blueprint, building_count);

    size_t building_index = 0;
    for (int idx = 0; idx < length; idx++) {  // 每一行
        double module_pos_y = row_max_y[idx] - 0.5 * module[chromosome[idx]].dy;
        double x_spacing = max_x_span / row_module_num[idx];
        for (int j = 0; j < row_module_num[idx]; j++) {  // 每个模块
            // 复制并旋转建筑
            dspbptk_building_copy(&blueprint.buildings[building_index], module[chromosome[idx]].blueprint.buildings, module[chromosome[idx]].blueprint.numBuildings, building_index);
            double module_pos_x = (j + 0.5) * x_spacing;
            vec4 sph = {module_pos_x, module_pos_y};
            vec4 rct;
            sph_to_rct(sph, rct);
            mat4x4 rot;
            set_rot_mat(rct, rot);
            for (int k = building_index; k < building_index + module[chromosome[idx]].blueprint.numBuildings; k++) {  // 当前模块每个建筑
                dspbptk_building_localOffset_rotation(&blueprint.buildings[k], rot);
            }
            // 自动处理传送带吸附
            const int auto_adsorption = 1;
            if (auto_adsorption && j > 0) {
                for (int k = building_index - module[chromosome[idx]].blueprint.numBuildings; k < building_index; k++) {  // 上个模块的每个建筑
                    const module_enum_t belt_mk3 = 2003;
                    if (blueprint.buildings[k].itemId != belt_mk3 || blueprint.buildings[k].tempOutputObjIdx != OBJ_NULL)
                        continue;
                    double min_dis2 = 4000000.0;
                    size_t best_l = OBJ_NULL;
                    for (int l = building_index; l < building_index + module[chromosome[idx]].blueprint.numBuildings; l++) {
                        if (blueprint.buildings[l].itemId != belt_mk3)
                            continue;
                        double tmp_dis2 = vec3_distance_2(blueprint.buildings[k].localOffset, blueprint.buildings[l].localOffset);
                        if (tmp_dis2 < min_dis2) {
                            min_dis2 = tmp_dis2;
                            best_l = l;
                        }
                    }
                    memcpy(blueprint.buildings[best_l].localOffset, blueprint.buildings[k].localOffset, sizeof(vec4));
                    memcpy(blueprint.buildings[best_l].localOffset2, blueprint.buildings[k].localOffset2, sizeof(vec4));
                }
            }
            building_index += module[chromosome[idx]].blueprint.numBuildings;
        }
    }

    // 自动生成文件名
    char file_name[256] = {0};
    char* ptr_file_name = file_name;
    ptr_file_name += sprintf(ptr_file_name, "(%lf,%lld)", score, score2);
    for (size_t i = 0; i < length; i++)
        ptr_file_name += sprintf(ptr_file_name, "%d", chromosome[i]);
    sprintf(ptr_file_name, ".txt");

    // 输出蓝图
    printf("Output blueprint to \"%s\"\n", file_name);
    FILE* fp = fopen(file_name, "w");
    blueprint_encode_file(coder, &blueprint, fp);
    fclose(fp);

    // 释放内存
    dspbptk_free_blueprint(&blueprint);
}

typedef struct {
    double objective_score;
    size_t subjective_score;
    size_t length;
    module_enum_t chromosome[CHROMOSOME_LENGTH];
} best_chromosome_t;

/**
 * @brief 穷举所有可能的排列，然后计算当前排列的理论最大产能。注意这是一个递归的函数
 *
 * @param coder 用于输出计算结果的蓝图编码器
 * @param need 对不同模块的需求比例，由用户输入
 * @param max_x_span 允许的赤道处最大x跨度，由用户输入
 * @param max_y_span 允许的最大y跨度，由用户输入
 * @param module 模块的数据，程序启动时自动读取生成
 * @param chromosome 当前排列的编码(染色体)
 * @param max_module_count 当前输入参数的约束下，每种模块的数量上界，搜索环境初始化时自动计算
 * @param cache_row_y_max 缓存某一行的y最大值，在此函数中被自动维护
 * @param cache_module_sum 缓存当前排列的模块总数，在此函数中被自动维护
 * @param idx 搜索深度，也就是当前回溯到了哪一行
 * @param best_chromosome_count 最优解有多少个
 * @param best_chromosome 最优解列表，搜索完以后的最优解记录在此表中
 */
void search(dspbptk_coder_t* coder, const double need[MODULE_COUNT], double max_x_span, double max_y_span, const module_t module[MODULE_COUNT], const size_t max_module_count[MODULE_COUNT], module_enum_t chromosome[CHROMOSOME_LENGTH], double cache_row_y_max[CHROMOSOME_LENGTH], size_t cache_row_module_num[CHROMOSOME_LENGTH], size_t cache_module_sum[CHROMOSOME_LENGTH][MODULE_COUNT], int64_t idx, size_t* best_chromosome_count, best_chromosome_t best_chromosome[SIZE_CACHE_CHROMOSOME]) {  // 2024/01/30 喜欢一行400+字符吗，我故意的 :)
    for (module_enum_t typ = 0; typ < MODULE_COUNT; typ++) {
        // 剪枝1：检查某种建筑是否过多
        if (cache_module_sum[idx - 1][typ] >= max_module_count[typ])
            continue;

        // 更新行数据
        chromosome[idx] = typ;
        memcpy(cache_module_sum[idx], cache_module_sum[idx - 1], sizeof(size_t) * MODULE_COUNT);
        update_row_data(cache_row_y_max, cache_row_module_num, cache_module_sum[idx], idx, typ, module, max_x_span);

        // 评分并记录
        static double max_objective_score = 0.0;
        double tmp_objective_score = objective_score(cache_module_sum[idx], need, cache_row_y_max[idx], max_y_span);
        if (tmp_objective_score >= max_objective_score) {
            size_t tmp_subjective_score = subjective_score(chromosome, idx + 1);
            print_detail(tmp_objective_score, tmp_subjective_score, chromosome, idx + 1, cache_module_sum[idx]);
            if (tmp_objective_score > max_objective_score) {
                max_objective_score = tmp_objective_score;
                (*best_chromosome_count) = 0;
            }
            if (*best_chromosome_count < SIZE_CACHE_CHROMOSOME) {
                best_chromosome[*best_chromosome_count].objective_score = tmp_objective_score;
                best_chromosome[*best_chromosome_count].subjective_score = tmp_subjective_score;
                best_chromosome[*best_chromosome_count].length = idx + 1;
                memcpy(best_chromosome[*best_chromosome_count].chromosome, chromosome, CHROMOSOME_LENGTH * sizeof(module_enum_t));
                (*best_chromosome_count)++;
            } else {
                fprintf(stderr, "Warning: SIZE_CACHE_CHROMOSOME is no enough to record all results.\n");
            }
        }

        // 剪枝2：检查y跨度是否过大
        if (cache_row_y_max[idx] < max_y_span && idx + 1 < CHROMOSOME_LENGTH) {
            search(coder, need, max_x_span, max_y_span, module, max_module_count, chromosome, cache_row_y_max, cache_row_module_num, cache_module_sum, idx + 1, best_chromosome_count, best_chromosome);  // 尾递归，gcc -O2或更高的优化等级可以优化成循环
        }
    }
}

int main(void) {
    // TODO 输入参数处理
    double max_x_span = 100.0;
    double max_y_span = 170.0;
    double need[MODULE_COUNT] = {635.0,87.0};

    // const char* options = "hx:y:";

    // 初始化蓝图编码器
    dspbptk_coder_t coder;
    dspbptk_init_coder(&coder);

    // 读取模块
    module_t module[MODULE_COUNT];
    init_module(module, &coder);

    // 预计算数据
    size_t max_module_count[MODULE_COUNT];
    compute_max_module_count(max_module_count, module, need, max_x_span, max_y_span);
    printf("max_module_count:");
    for (int i = 0; i < MODULE_COUNT; i++)
        printf("%lld,", max_module_count[i]);
    putchar('\n');

    // 构造所有排列
    size_t best_chromosome_count = 0;
    best_chromosome_t best_chromosome[SIZE_CACHE_CHROMOSOME] = {{0.0, 0, 0, {0}}};

    module_enum_t chromosome[CHROMOSOME_LENGTH] = {0};
    // 注意这里的length+1是了为了读-1的下标，不能去掉
    double cache_row_y_max[CHROMOSOME_LENGTH + 1] = {0.0};
    size_t cache_row_module_num[CHROMOSOME_LENGTH + 1] = {0};
    size_t cache_module_sum[CHROMOSOME_LENGTH + 1][MODULE_COUNT] = {0};
    search(&coder, need, max_x_span, max_y_span, module, max_module_count, chromosome, cache_row_y_max + 1, cache_row_module_num + 1, cache_module_sum + 1, 0, &best_chromosome_count, best_chromosome);

    // 输出蓝图
    fprintf(stderr, "best_chromosome_count = %lld\n", best_chromosome_count);
    for (int i = 0; i < best_chromosome_count; i++) {
        output_blueprint(&coder, module, best_chromosome[i].chromosome, best_chromosome[i].length, max_x_span, best_chromosome[i].objective_score, best_chromosome[i].subjective_score);
    }

    // 销毁蓝图编码器
    dspbptk_free_coder(&coder);

    return 0;
}