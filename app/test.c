#include "../lib/dspbptk.h"

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
    FILE* fp = fopen("bin.bin", "wb");
    fwrite(bp_data.bin, 1, bp_data.bin_len, fp);
    fclose(fp);

    char* json;
    data_to_json(&bp_data, &json);
    puts(json);

    unsigned char* p_bin = (unsigned char*)bp_data.bin;
    int area_num = *((int8_t*)(p_bin + BIN_OFFSET_AREA_NUM));

    // size_t offset = BIN_OFFSET_AREA_ARRAY + area_num * AREA_OFFSET_AREA_NEXT + 4;
    // printf("offset = %lld\n", offset);
    // p_bin += offset; // 建筑列表的偏移值
    // for(int i = 0; i < 650; i++) {
    //     int index = *((int32_t*)(p_bin + building_offset_index));
    //     int itemid = *((int16_t*)(p_bin +building_offset_itemId));
    //     printf("index=%d,\titemid=%d\n", index, itemid);
    //     int para_num = *((int16_t*)(p_bin + building_offset_num));
    //     printf("para_num=%d\n", para_num);
    //     p_bin += building_offset_parameters + 4 * para_num;
    // }

    // 输出
    data_to_blueprint(&bp_data, blueprint_out);
    blueprint_to_file("out.txt", blueprint_out);

    // free
    free(blueprint_out);
    free(blueprint_in);
    free_bp_data(&bp_data);

    return 0;
}