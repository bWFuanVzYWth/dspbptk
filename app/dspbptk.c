#include "../lib/dspbptk.h"

// TODO .log
// TODO 异常检查
int main(int argc, char* argv[]) {
    FILE* log = stdout;

    int error_flag = 0;
    char file_in[4096] = { 0 };
    char file_out[4096] = { 0 };

    for(int i = 0; i < argc; i++)
        fprintf(log, "arg%d = %s\n", i, argv[i]);

    for(int i = 0; i < argc; i++) {
        if(argv[i][0] == '-') {
            switch(argv[i][1]) {
            case 'i':
            sscanf(argv[i], "-i=%s", file_in);
            fprintf(log, "从%s读取文件\n", file_in);
            break;

            case 'o':
            sscanf(argv[i], "-o=%s", file_out);
            fprintf(log, "输出到%s\n", file_out);
            break;
            }
        }
    }

    if(strlen(file_in) < 1 || strlen(file_out) < 1)
        goto error;

    char* blueprint_in;
    char* blueprint_out = calloc(BLUEPRINT_MAX_LENGTH, 1);

    size_t length = file_to_blueprint(file_in, &blueprint_in);
    fprintf(log, "蓝图大小：%lld\n", length);

    // 解析蓝图
    bp_data_t bp_data;
    blueprint_to_data(&bp_data, blueprint_in);
    fprintf(log, "建筑数: %lld\n", bp_data.building_num);

    // 输出
    data_to_blueprint(&bp_data, blueprint_out);
    fprintf(log, "蓝图已生成\n");
    blueprint_to_file(file_out, blueprint_out);
    fprintf(log, "蓝图已输出\n");

    // free
    free(blueprint_out);
    free(blueprint_in);
    free_bp_data(&bp_data);

    return 0;

error:
    printf("error\n");
    return -1;
}