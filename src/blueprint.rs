struct area_t {
    index: i64,
    parentIndex: i64,
    tropicAnchor: i64,
    areaSegments: i64,
    anchorLocalOffsetX: i64,
    anchorLocalOffsetY: i64,
    width: i64,
    height: i64,
}

struct building_t {
    num: i64,
    index: i64,
    areaIndex: i64,
    localOffset: [f64; 4],
    localOffset2: [f64; 4],
    yaw: f64,
    yaw2: f64,
    tilt: f64,
    itemId: i64,
    modelIndex: i64,
    tempOutputObjIdx: i64,
    tempInputObjIdx: i64,
    outputToSlot: i64,
    inputFromSlot: i64,
    outputFromSlot: i64,
    inputToSlot: i64,
    outputOffset: i64,
    inputOffset: i64,
    recipeId: i64,
    filter_id: i64,
    parameters: Vec<u32>,
}

pub struct blueprint_t {
    // head
    layout: i64,
    icons: [i64; 5],
    time: i64,
    gameVersion: &'static str,
    shortDesc: &'static str,
    desc: &'static str,
    // base64
    version: i64,
    cursorOffsetX: i64,
    cursorOffsetY: i64,
    cursorTargetArea: i64,
    dragBoxSizeX: i64,
    dragBoxSizeY: i64,
    primaryAreaIdx: i64,
    areas: Vec<area_t>,
    buildings: Vec<building_t>,
    // md5f
    md5f: [u32; 4],
}
