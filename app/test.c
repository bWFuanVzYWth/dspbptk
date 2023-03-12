#include "../lib/dspbptk.h"

int main(void) {

    // 读取蓝图
    char* blueprint;
    size_t length = file_to_blueprint("in.txt", &blueprint);
    printf("蓝图大小：%lld\n", length);

    // 解析蓝图
    bp_data_t bp_data;
    blueprint_to_data(&bp_data, blueprint);

    // 在这里修改蓝图
    FILE* fp = fopen("raw.bin", "wb");
    fwrite(bp_data.raw, 1, bp_data.raw_len, fp);
    fclose(fp);

    // free
    free(blueprint);
    free_bp_data(&bp_data);

    return 0;
}