use dspbptk::{
    blueprint::{
        content::{building::BuildingData, ContentData},
        header::HeaderData,
    },
    error::DspbptkError,
    io::{BlueprintKind, FileType},
};

fn new_ray_receiver(index: i32, local_offset: [f32; 3]) -> BuildingData {
    BuildingData {
        index: index,
        item_id: 2208,
        model_index: 73,
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

    let base: Vec<_> = (0..=9)
        .map(|x| new_ray_receiver(x as i32, [15.0 * x as f32, 0.0, 0.0]))
        .collect();

    let test_axis = (0..=9)
        .map(|x| {
            new_ray_receiver(
                x as i32,
                [15.0 * x as f32, 7.3072 + 0.00001 * x as f32, 0.0],
            )
        })
        .collect(); // (7.30725, 7.30726)

    let test_corner = (0..=9)
        .map(|x| {
            new_ray_receiver(
                x as i32,
                [15.0 * x as f32 + 7.2, -(4.1982 + 0.00001 * x as f32), 0.0],
            )
        })
        .collect(); // (4.19828, 4.19829)

    let buildings = vec![base, test_axis, test_corner].concat();

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
