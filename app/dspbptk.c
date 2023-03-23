#include "../lib/libdspbptk.h"

// time in ns
uint64_t get_timestamp(void) {
    struct timespec t;
    clock_gettime(0, &t);
    return (uint64_t)t.tv_sec * 1000000000 + (uint64_t)t.tv_nsec;
}

double ns_to_ms(int64_t ns) {
    return (double)ns / 1000000.0;
}

// TODO 异常检查
int main(int argc, char* argv[]) {
    FILE* log = stderr;

    dspbptk_err_t state = no_error;

    if(argc <= 1) {
        state = error_argc;
        goto error;
    }

    char file_in[4096] = { 0 };
    char file_out[4096] = { 0 };

    for(int i = 0; i < argc; i++)
        fprintf(log, "arg%d=\"%s\"\n", i, argv[i]);

    int replace_flag = 0;
    int32_t replace_from;
    int32_t replace_to;

    // 命令格式检查
    for(int i = 0; i < argc; i++) {
        if(argv[i][0] == '-') {
            switch(argv[i][1]) {
            case 'i':
            sscanf(argv[i], "-i=%s", file_in);
            break;

            case 'o':
            sscanf(argv[i], "-o=%s", file_out);
            break;

            case 'r':
            sscanf(argv[i], "-r%d=>%d", &replace_from, &replace_to);
            replace_flag = 1;
            break;
            }
        }
    }

    char* blueprint_in;
    char* blueprint_out = calloc(BLUEPRINT_MAX_LENGTH, 1);

    uint64_t time_0_file_to_blueprint = get_timestamp();
    size_t blueprint_in_size = file_to_blueprint(file_in, &blueprint_in);
    uint64_t time_1_file_to_blueprint = get_timestamp();
    fprintf(log, "read file in %lf ms.\n", ns_to_ms(time_1_file_to_blueprint - time_0_file_to_blueprint));
    if(blueprint_in_size <= 0) {
        state = file_no_found;
        goto error;
    }
    fprintf(log, "blueprint_in_size=%lld\n", blueprint_in_size);
    fprintf(log, "md5f_old=%s\n", blueprint_in + blueprint_in_size - 32);


    // 解析蓝图
    bp_data_t bp_data;
    uint64_t time_0_blueprint_to_data = get_timestamp();
    state = blueprint_to_data(&bp_data, blueprint_in);
    uint64_t time_1_blueprint_to_data = get_timestamp();
    fprintf(log, "parsing blueprint in %lf ms.\n", ns_to_ms(time_1_blueprint_to_data - time_0_blueprint_to_data));

    // 输出蓝图信息
    if(state)
        goto error;
    fprintf(log, "building_num=%lld\n", bp_data.building_num);

    // 修改蓝图
    if(replace_flag) {
        size_t replace_count = building_replace(replace_from, replace_to, &bp_data);
        fprintf(stderr, "note: replaced %llu building.\n", replace_count);
    }

    // 编码蓝图
    uint64_t time_0_data_to_blueprint = get_timestamp();
    data_to_blueprint(&bp_data, blueprint_out);
    uint64_t time_1_data_to_blueprint = get_timestamp();
    fprintf(log, "encode blueprint in %lf ms.\n", ns_to_ms(time_1_data_to_blueprint - time_0_data_to_blueprint));

    size_t blueprint_out_size = strlen(blueprint_out);
    fprintf(log, "blueprint_out_size=%lld\n", blueprint_out_size);
    int64_t size_change = blueprint_out_size - blueprint_in_size;
    fprintf(log, "size_change=%lld,%.2lf%%\n", size_change, (double)size_change / (double)blueprint_in_size * 100.0);
    fprintf(log, "md5f_new=%s\n", blueprint_out + blueprint_out_size - 32);

    uint64_t time_0_blueprint_to_file = get_timestamp();
    state = blueprint_to_file(file_out, blueprint_out);
    uint64_t time_1_blueprint_to_file = get_timestamp();
    fprintf(log, "write file in %lf ms.\n", ns_to_ms(time_1_blueprint_to_file - time_0_blueprint_to_file));

    if(state)
        goto error;
    fprintf(log, "finish\n");

    // free
    free(blueprint_out);
    free(blueprint_in);
    free_bp_data(&bp_data);

    return 0;

error:
    fprintf(log, "error_code=%d\n", state);
    return state;
}