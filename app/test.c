#include "../lib/dspbptk.h"

int main(void) {

    // 读取蓝图
    char* blueprint_in;
    char* blueprint_out = calloc(BP_LEN, 1);
    size_t length = file_to_blueprint("in.txt", &blueprint_in);
    printf("蓝图大小：%lld\n", length);

    // 解析蓝图
    bp_data_t bp_data;
    blueprint_to_data(&bp_data, blueprint_in);

    // 在这里修改蓝图
    FILE* fp = fopen("raw.bin", "wb");
    fwrite(bp_data.raw, 1, bp_data.raw_len, fp);
    fclose(fp);

    // 输出
    data_to_blueprint(&bp_data, blueprint_out);
    blueprint_to_file("out.txt", blueprint_out);

    // free
    free(blueprint_out);
    free(blueprint_in);
    free_bp_data(&bp_data);

    return 0;
}