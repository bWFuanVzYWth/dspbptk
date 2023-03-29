#ifndef ENUM_OFFSET
#define ENUM_OFFSET

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    bin_offset_version              =                                   0,
    bin_offset_cursorOffset_x       = bin_offset_version                + 4,
    bin_offset_cursorOffset_y       = bin_offset_cursorOffset_x         + 4,
    bin_offset_cursorTargetArea     = bin_offset_cursorOffset_y         + 4,
    bin_offset_dragBoxSize_x        = bin_offset_cursorTargetArea       + 4,
    bin_offset_dragBoxSize_y        = bin_offset_dragBoxSize_x          + 4,
    bin_offset_primaryAreaIdx       = bin_offset_dragBoxSize_y          + 4,
    BIN_OFFSET_AREA_NUM             = bin_offset_primaryAreaIdx         + 4,
    BIN_OFFSET_AREA_ARRAY           = BIN_OFFSET_AREA_NUM               + 1
}bin_offset_t;

typedef enum {
    area_offset_index               =                                   0,
    area_offset_parentIndex         = area_offset_index                 + 1,
    area_offset_tropicAnchor        = area_offset_parentIndex           + 1,
    area_offset_areaSegments        = area_offset_tropicAnchor          + 2,
    area_offset_anchorLocalOffsetX  = area_offset_areaSegments          + 2,
    area_offset_anchorLocalOffsetY  = area_offset_anchorLocalOffsetX    + 2,
    area_offset_width               = area_offset_anchorLocalOffsetY    + 2,
    area_offset_height              = area_offset_width                 + 2,
    AREA_OFFSET_AREA_NEXT           = area_offset_height                + 2,
    AREA_OFFSET_BUILDING_ARRAY      = AREA_OFFSET_AREA_NEXT             + 4
}area_offset_t;

typedef enum {
    building_offset_index           =                                   0,
    building_offset_areaIndex       = building_offset_index             + 4,
    building_offset_localOffset_x   = building_offset_areaIndex         + 1,
    building_offset_localOffset_y   = building_offset_localOffset_x     + 4,
    building_offset_localOffset_z   = building_offset_localOffset_y     + 4,
    building_offset_localOffset_x2  = building_offset_localOffset_z     + 4,
    building_offset_localOffset_y2  = building_offset_localOffset_x2    + 4,
    building_offset_localOffset_z2  = building_offset_localOffset_y2    + 4,
    building_offset_yaw             = building_offset_localOffset_z2    + 4,
    building_offset_yaw2            = building_offset_yaw               + 4,
    building_offset_itemId          = building_offset_yaw2              + 4,
    building_offset_modelIndex      = building_offset_itemId            + 2,
    building_offset_tempOutputObjIdx= building_offset_modelIndex        + 2,
    building_offset_tempInputObjIdx = building_offset_tempOutputObjIdx  + 4,
    building_offset_outputToSlot    = building_offset_tempInputObjIdx   + 4,
    building_offset_inputFromSlot   = building_offset_outputToSlot      + 1,
    building_offset_outputFromSlot  = building_offset_inputFromSlot     + 1,
    building_offset_inputToSlot     = building_offset_outputFromSlot    + 1,
    building_offset_outputOffset    = building_offset_inputToSlot       + 1,
    building_offset_inputOffset     = building_offset_outputOffset      + 1,
    building_offset_recipeId        = building_offset_inputOffset       + 1,
    building_offset_filterId        = building_offset_recipeId          + 2,
    building_offset_num             = building_offset_filterId          + 2,
    building_offset_parameters      = building_offset_num               + 2
}building_offset_t;

#ifdef __cplusplus
}
#endif

#endif