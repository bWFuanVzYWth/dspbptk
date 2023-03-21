#include "../lib/libdspbptk.h"

int main(void) {

    // 读取蓝图
    char* blueprint_in;
    char* blueprint_out = calloc(BLUEPRINT_MAX_LENGTH, 1);
    size_t length = file_to_blueprint("in.txt", &blueprint_in);
    printf("蓝图大小：%lld\n", length);

    // 解析蓝图
    bp_data_t bp_data;
    blueprint_to_data(&bp_data, blueprint_in);

    // 在这里修改蓝图


    // 输出
    data_to_blueprint(&bp_data, blueprint_out);
    blueprint_to_file("out.txt", blueprint_out);

    // free
    free(blueprint_out);
    free(blueprint_in);
    free_bp_data(&bp_data);

    return 0;
}