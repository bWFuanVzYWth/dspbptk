use dspbptk::{
    blueprint::{
        content::{building::BuildingData, ContentData},
        header::HeaderData,
    },
    error::DspbptkError,
    io::{BlueprintKind, FileType},
    item::Item,
};

fn new_ray_receiver(index: i32, local_offset: [f32; 3]) -> BuildingData {
    BuildingData {
        index: index,
        item_id: Item::射线接收站 as i16,
        model_index: Item::射线接收站.model()[0],
        local_offset_x: local_offset[0],
        local_offset_y: local_offset[1],
        local_offset_z: local_offset[2],
        parameters: vec![1208],
        parameters_length: 1,
        ..Default::default()
    }
}

fn main() -> Result<(), DspbptkError<'static>> {
    let header_data = HeaderData::default();
    let zopfli_options = zopfli::Options::default();

    // 基础行
    let base: Vec<_> = (0..=9)
        .map(|x| new_ray_receiver(x as i32, [15.0 * x as f32, 0.0, 0.0]))
        .collect();

    // 测试长轴碰撞
    let test_axis = (0..=9)
        .map(|x| {
            new_ray_receiver(
                x as i32,
                [15.0 * x as f32, 7.3072 + 0.00001 * x as f32, 0.0],
            )
        })
        .collect(); // (7.30725, 7.30726)

    // 测试角落碰撞
    let test_corner = (0..=9)
        .map(|x| {
            new_ray_receiver(
                x as i32,
                [15.0 * x as f32 + 7.2, -(4.1982 + 0.00001 * x as f32), 0.0],
            )
        })
        .collect(); // (4.19828, 4.19829)

    // 拼接所有建筑
    let buildings: Vec<_> = vec![base, test_axis, test_corner]
        .concat()
        .into_iter()
        .enumerate()
        .map(|(i, building)| BuildingData {
            index: i as i32,
            ..building
        })
        .collect();

    let content_data = ContentData {
        buildings_length: buildings.len() as i32,
        buildings: buildings,
        ..Default::default()
    };

    println!("{:#?}", content_data);

    if let BlueprintKind::Txt(blueprint) =
        dspbptk::io::process_back_end(&header_data, &content_data, &zopfli_options, &FileType::Txt)?
    {
        print!("{}", blueprint);
    }

    Ok(())
}
