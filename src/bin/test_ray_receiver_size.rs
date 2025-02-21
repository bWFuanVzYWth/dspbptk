use dspbptk::{
    blueprint::{
        content::{building::DspbptkBuildingData, ContentData},
        header::HeaderData,
    },
    error::DspbptkError::{self},
    io::{BlueprintKind, FileType},
    item::Item,
    toolkit::fix_dspbptk_buildings_index,
};
use uuid::Uuid;

fn new_receiver(local_offset: [f64; 3]) -> DspbptkBuildingData {
    DspbptkBuildingData {
        uuid: Some(Uuid::new_v4().to_u128_le()),
        item_id: Item::射线接收站 as i16,
        model_index: Item::射线接收站.model()[0],
        local_offset,
        parameters: vec![1208],
        ..Default::default()
    }
}

fn main() -> Result<(), DspbptkError<'static>> {
    let header_data = HeaderData::default();
    let zopfli_options = zopfli::Options::default();

    // 基础行
    let base = (0..=9)
        .map(|x| new_receiver([15.0 * f64::from(x), 0.0, 0.0]))
        .collect::<Vec<_>>();

    // 测试长轴碰撞
    let test_axis = (0..=9)
        .map(|x| {
            new_receiver([
                15.0 * f64::from(x),
                f64::from(x).mul_add(0.00001, 7.3072),
                0.0,
            ])
        })
        .collect(); // (7.30725, 7.30726)

    // 测试角落碰撞
    let test_corner = (0..=9)
        .map(|x| {
            new_receiver([
                f64::from(x).mul_add(15.0, 7.2),
                f64::from(x).mul_add(-0.00001, 4.1982),
                0.0,
            ])
        })
        .collect(); // (4.19828, 4.19829)

    // 拼接所有建筑
    let buildings = [base, test_axis, test_corner].concat();
    let buildings = fix_dspbptk_buildings_index(buildings);

    let content_data = ContentData {
        buildings_length: u32::try_from(buildings.len()).unwrap(),
        buildings: buildings
            .iter()
            .map(|dspbptk_building| dspbptk_building.to_building_data().unwrap())
            .collect(),
        ..Default::default()
    };

    println!("{content_data:#?}");

    if let BlueprintKind::Txt(blueprint) =
        dspbptk::io::process_back_end(&header_data, &content_data, &zopfli_options, &FileType::Txt)?
    {
        print!("{blueprint}");
    }

    Ok(())
}
