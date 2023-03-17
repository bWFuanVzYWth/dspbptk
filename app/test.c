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

    char* json;
    data_to_json(&bp_data, &json);
    puts(json);

    unsigned char* p_raw = (unsigned char*)bp_data.raw;
    int area_num = *((int8_t*)(p_raw + _num_area));

    size_t offset = _area_array + area_num * _next_area + 4;
    printf("offset = %lld\n", offset);
    p_raw += offset; // 建筑列表的偏移值
    for(int i = 0; i < 20; i++) {
        int index = *((int32_t*)(p_raw + _index_bpbd));
        int itemid = *((int16_t*)(p_raw + _itemId));
        printf("index=%d,\titemid=%d\n", index, itemid);
        int para_num = *((int16_t*)(p_raw + _num_bpbd));
        printf("para_num=%d\n", para_num);
        p_raw += _parameters_bpbd + 4 * para_num;
    }

    // 输出
    data_to_blueprint(&bp_data, blueprint_out);
    blueprint_to_file("out.txt", blueprint_out);

    // free
    free(blueprint_out);
    free(blueprint_in);
    free_bp_data(&bp_data);

    return 0;
}