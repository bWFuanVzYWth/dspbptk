#include "../lib/libdspbptk.h"

#define HELP_DOC "\nCopyright (c) 2023 @bWFuanVzYWth, MIT License.\n\
https://github.com/bWFuanVzYWth/dspbptk\n\
使用方法 | Useage:\n\
dspbptk [options]... [commands]...\n\
设置 | Options:\n\
-i=<file>           从文件读取蓝图。    | Read blueprint from file.\n\
-o=<file>           将蓝图写入文件。    | Write blueprint to file.\n\
命令 | Commands:\n\
c                   压缩蓝图。无命令时的默认操作。  | Compress blueprint. This is the default command.\n\
x*=[num]            沿x轴方向放缩。 | Zoom along the x-axis direction.\n\
x+=[num]            沿x轴方向平移。 | Translate along the x-axis direction.\n\
y*=[num]            沿y轴方向放缩。 | Zoom along the y-axis direction.\n\
y+=[num]            沿y轴方向平移。 | Translate along the y-axis direction.\n\
z*=[num]            沿z轴方向放缩。 | Zoom along the z-axis direction.\n\
z+=[num]            沿z轴方向平移。 | Translate along the z-axis direction.\n\
sp[id]:[n]=[num]    将某种建筑的第n个附加参数设置成特定值。 | Set parameter.\n\
示例 | Examples:\n\
dspbptk -i=i.txt -o=o.txt c sp2301:5=\n\
dspbptk -i=i.txt -o=o.txt x*=1.1 y+=-1 z+=0.05\n\
"

typedef enum {
    x_mul = 0,
    x_add,
    y_mul,
    y_add,
    z_mul,
    z_add,
    sp
}operate_t;

// time in ns
uint64_t get_timestamp(void) {
    struct timespec t;
    clock_gettime(0, &t);
    return (uint64_t)t.tv_sec * 1000000000 + (uint64_t)t.tv_nsec;
}

double ns_to_ms(int64_t ns) {
    return (double)ns / 1000000.0;
}

void lexer(int argc, char** argv,
    int* processed, dspbptk_err_t* state,
    operate_t* command, void* command_arg, size_t* command_count,
    char* file_i, char* file_o
) {
    double(*command_arg_f64)[32] = (double(*)[32])command_arg;
    int64_t(*command_arg_i64)[32] = (int64_t(*)[32])command_arg;

    // 第一遍扫描参数列表，录入所有设置，写入操作列表
#define ERROR_ARG {processed[i] = -1; *state = error_arg; break;}
#define VALID_ARG {processed[i] = 1; break;}

    for(int i = 1; i < argc; i++) {
        if(argv[i][0] == '-') {
            // 设置
            switch(argv[i][1]) {
            case 'i':
            if(strlen(argv[i]) < 4) ERROR_ARG;
            if(sscanf(argv[i], "-i=%s", file_i) < 1) ERROR_ARG;
            VALID_ARG;

            case 'o':
            if(strlen(argv[i]) < 4) ERROR_ARG;
            if(sscanf(argv[i], "-o=%s", file_o) < 1) ERROR_ARG;
            VALID_ARG;

            default:
            ERROR_ARG;
            }
        }
        else {
            switch(argv[i][0]) {
            case 'c':
            if(strlen(argv[i]) != 1) ERROR_ARG;
            VALID_ARG; // 就是个安慰剂命令，啥也不用干，因为本来就会压缩

            case 's':
            if(memcmp(argv[i], "sp", 2) == 0) {
                if(sscanf(argv[i], "sp%lld:%lld=%lld",
                    &command_arg_i64[(*command_count)][0],
                    &command_arg_i64[(*command_count)][1],
                    &command_arg_i64[(*command_count)][2]) < 3) ERROR_ARG;
                command[(*command_count)] = sp;
                (*command_count)++;
                VALID_ARG;
            }

            case 'x':
            if(strlen(argv[i]) < 4) ERROR_ARG;
            if(memcmp(argv[i], "x*=", 3) == 0) {
                if(sscanf(argv[i], "x*=%lf", &command_arg_f64[(*command_count)][0]) < 1) ERROR_ARG;
                command[(*command_count)] = x_mul;
                (*command_count)++;
                VALID_ARG;
            }
            else if(memcmp(argv[i], "x+=", 3) == 0) {
                if(sscanf(argv[i], "x+=%lf", &command_arg_f64[(*command_count)][0]) < 1) ERROR_ARG;
                command[(*command_count)] = x_add;
                (*command_count)++;
                VALID_ARG;
            }
            else {
                ERROR_ARG;
            }

            case 'y':
            if(strlen(argv[i]) < 4) ERROR_ARG;
            if(memcmp(argv[i], "y*=", 3) == 0) {
                if(sscanf(argv[i], "y*=%lf", &command_arg_f64[(*command_count)][0]) < 1) ERROR_ARG;
                command[(*command_count)] = y_mul;
                (*command_count)++;
                VALID_ARG;
            }
            else if(memcmp(argv[i], "y+=", 3) == 0) {
                if(sscanf(argv[i], "y+=%lf", &command_arg_f64[(*command_count)][0]) < 1) ERROR_ARG;
                command[(*command_count)] = y_add;
                (*command_count)++;
                VALID_ARG;
            }
            else {
                ERROR_ARG;
            }

            case 'z':
            if(strlen(argv[i]) < 4) ERROR_ARG;
            if(memcmp(argv[i], "z*=", 3) == 0) {
                if(sscanf(argv[i], "z*=%lf", &command_arg_f64[(*command_count)][0]) < 1) ERROR_ARG;
                command[(*command_count)] = z_mul;
                (*command_count)++;
                VALID_ARG;
            }
            else if(memcmp(argv[i], "z+=", 3) == 0) {
                if(sscanf(argv[i], "z+=%lf", &command_arg_f64[(*command_count)][0]) < 1) ERROR_ARG;
                command[(*command_count)] = z_add;
                (*command_count)++;
                VALID_ARG;
            }
            else {
                ERROR_ARG;
            }

            }
        }
    }
}

void evaluationor(operate_t* command, void* command_arg, size_t* command_count,
    bp_data_t* p_bp_data) {
    double(*command_arg_f64)[32] = (double(*)[32])command_arg;
    int64_t(*command_arg_i64)[32] = (int64_t(*)[32])command_arg;

// 遍历建筑
    for(int i = 0; i < p_bp_data->building_num; i++) {
        double pos1[3] = { 0 };
        double pos2[3] = { 0 };
        get_building_pos1(p_bp_data->building[i], pos1);
        get_building_pos2(p_bp_data->building[i], pos2);

        // 依次执行命令
        for(int j = 0; j < *command_count; j++) {
            switch(command[j]) {
            case sp:
            if(get_building_itemID(p_bp_data->building[j]) == (int16_t)command_arg_i64[j][0]) {
                size_t building_parameters_num = get_building_parameters_num(p_bp_data->building[j]);
                if(command_arg_i64[j][1] < building_parameters_num && command_arg_i64[j][1] >= 0)
                    set_building_parameter(p_bp_data->building[j], command_arg_i64[j][1], command_arg_i64[j][2]);
                else
                    fprintf(stderr, "Warning: at index=%d, ItemID%lld doesn't have parameter[%lld]. Ignore.\n",
                        i, command_arg_i64[j][0], command_arg_i64[j][1]);
            }
            break;

            case x_mul:
            pos1[0] *= command_arg_f64[j][0];
            pos2[0] *= command_arg_f64[j][0];
            break;

            case x_add:
            pos1[0] += command_arg_f64[j][0];
            pos2[0] += command_arg_f64[j][0];
            break;

            case y_mul:
            pos1[1] *= command_arg_f64[j][0];
            pos2[1] *= command_arg_f64[j][0];
            break;

            case y_add:
            pos1[1] += command_arg_f64[j][0];
            pos2[1] += command_arg_f64[j][0];
            break;

            case z_mul:
            pos1[2] *= command_arg_f64[j][0];
            pos2[2] *= command_arg_f64[j][0];
            break;

            case z_add:
            pos1[2] += command_arg_f64[j][0];
            pos2[2] += command_arg_f64[j][0];
            break;
            }
        }

        set_building_pos1(p_bp_data->building[i], pos1);
        set_building_pos2(p_bp_data->building[i], pos2);
    }
}

// TODO 异常检查
int main(int argc, char* argv[]) {

#ifdef DEBUG
    for(int i = 0; i < argc; i++)
        fprintf(log, "Info: arg%d=\"%s\"\n", i, argv[i]);
#endif

    FILE* log = stdout;
    dspbptk_err_t state = no_error;

    // 检查是不是忘了输入参数
    if(argc <= 1) {
        state = error_arg;
        goto error_arg;
    }

    // 初始化
    char file_i[4096] = { 0 };
    char file_o[4096] = { 0 };
    int* processed = (int*)calloc(argc, sizeof(int));
    processed[0] = 1;
    size_t command_count = 0;
    operate_t* command = (operate_t*)calloc(argc, sizeof(operate_t));
    void* command_arg = calloc(argc, sizeof(int64_t) * 32);

    // 解析命令
    uint64_t time_0_lexing = get_timestamp();
    lexer(argc, argv, processed, &state, command, command_arg, &command_count, file_i, file_o);
    uint64_t time_1_lexing = get_timestamp();
    fprintf(log, "Info: lexing command in %lf ms.\n", ns_to_ms(time_1_lexing - time_0_lexing));
    if(state == error_arg || strlen(file_i) == 0 || strlen(file_o) == 0) {
        state = error_arg;
        goto error_arg;
    }

    // 读取蓝图
    char* blueprint_in;
    char* blueprint_out = calloc(BLUEPRINT_MAX_LENGTH, 1);
    uint64_t time_0_file_to_blueprint = get_timestamp();
    size_t blueprint_in_size = file_to_blueprint(file_i, &blueprint_in);
    uint64_t time_1_file_to_blueprint = get_timestamp();
    fprintf(log, "Info: read file in %lf ms.\n", ns_to_ms(time_1_file_to_blueprint - time_0_file_to_blueprint));
    if(blueprint_in_size <= 0) {
        state = file_no_found;
        goto error_blueprint;
    }

    // 解析蓝图
    bp_data_t bp_data;
    uint64_t time_0_blueprint_to_data = get_timestamp();
    state = blueprint_to_data(&bp_data, blueprint_in);
    uint64_t time_1_blueprint_to_data = get_timestamp();
    fprintf(log, "Info: parsing blueprint in %lf ms.\n", ns_to_ms(time_1_blueprint_to_data - time_0_blueprint_to_data));
    if(state)
        goto error_blueprint;

    // 修改蓝图
    uint64_t time_0_edit = get_timestamp();
    evaluationor(command, command_arg, &command_count, &bp_data);
    uint64_t time_1_edit = get_timestamp();
    fprintf(log, "Info: edit blueprint in %lf ms.\n", ns_to_ms(time_1_edit - time_0_edit));

    // 编码蓝图
    uint64_t time_0_data_to_blueprint = get_timestamp();
    data_to_blueprint(&bp_data, blueprint_out);
    uint64_t time_1_data_to_blueprint = get_timestamp();
    fprintf(log, "Info: encode blueprint in %lf ms.\n", ns_to_ms(time_1_data_to_blueprint - time_0_data_to_blueprint));

    // 写入文件
    uint64_t time_0_blueprint_to_file = get_timestamp();
    state = blueprint_to_file(file_o, blueprint_out);
    uint64_t time_1_blueprint_to_file = get_timestamp();
    fprintf(log, "Info: write file in %lf ms.\n", ns_to_ms(time_1_blueprint_to_file - time_0_blueprint_to_file));
    if(state)
        goto error_blueprint;

    size_t blueprint_out_size = strlen(blueprint_out);
    int64_t size_change = blueprint_out_size - blueprint_in_size;
    fprintf(log, "Info: Finish. blueprint_out_size=%lld, size_change=%lld,%.2lf%%\n",
        blueprint_out_size, size_change, (double)size_change / (double)blueprint_in_size * 100.0);

    // free
    free(blueprint_out);
    free(blueprint_in);
    free_bp_data(&bp_data);



    return 0;

error_blueprint:
    fprintf(log, "Error: %d\n", state);
    return state;

error_arg:
    puts(HELP_DOC);
    for(int i = 0; i < argc; i++) {
        if(processed[i] == -1)
            fprintf(log, "Error: Options/Commands error at \"%s\"\n", argv[i]);
    }
    return state;
}