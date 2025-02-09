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
        ..Default::default()
    }
}

fn main() -> Result<(), DspbptkError<'static>> {
    let header_data = HeaderData::default();
    let zopfli_options = zopfli::Options::default();

    let building_0 = new_ray_receiver(0, [0.0, 0.0, 0.0]);
    let buildings = vec![building_0];
    let content_data = ContentData {
        buildings_length: buildings.len() as i32,
        buildings: buildings,
        ..Default::default()
    };

    if let BlueprintKind::Txt(blueprint) =
        dspbptk::io::process_back_end(&header_data, &content_data, &zopfli_options, &FileType::Txt)?
    {
        print!("{}", blueprint);
    }

    Ok(())
}
