use crate::{
    blueprint::{
        self, Building, codec,
        data::{content::Content, header::Header},
        editor::{fix_index::fix_buildings_index, sort::sort_buildings},
    },
    error::{DspbptkError, DspbptkWarn},
    workflow::{BlueprintKind, LegalBlueprintFileType},
};

/// 蓝图工具的前端，可读取并解码多种格式的蓝图数据
///
/// # Errors
/// 所有读取或解码时发生的错误在此汇总
pub fn process_front_end(
    blueprint: &BlueprintKind,
) -> Result<(Header, Content, Vec<DspbptkWarn>), DspbptkError> {
    match blueprint {
        BlueprintKind::Txt(blueprint_string) => {
            // let start = std::time::Instant::now();

            let (blueprint_data, warns_blueprint) = codec::parse(blueprint_string)?;
            let blueprint_content_bin = codec::content::bin_from_string(blueprint_data.content)?;
            let (content_data, warns_content) =
                Content::from_bin(blueprint_content_bin.as_slice())?;
            let (header_data, warns_header) = codec::header::parse(blueprint_data.header)?;

            // log::info!("parse in {:?} sec.", start.elapsed());

            Ok((
                header_data,
                content_data,
                [
                    warns_blueprint.as_slice(),
                    warns_content.as_slice(),
                    warns_header.as_slice(),
                ]
                .concat(),
            ))
        }
        BlueprintKind::Content(content_bin) => {
            let (content_data, warns_content) = Content::from_bin(content_bin)?;
            let header_data = Header::default();
            Ok((header_data, content_data, warns_content))
        }
    }
}

pub trait DspbptkMap {
    fn apply(&self, content_in: Content) -> Content;
}

/// 蓝图工具的中间层，对蓝图应用修改
pub fn process_middle_layer(
    header_data_in: Header,
    content_data_in: Content,
    sorting_buildings: bool,
    rounding_local_offset: bool,
    func_args: &impl DspbptkMap,
) -> (blueprint::Header, Content) {
    let (header_data_out, mut content_data_out) =
        (header_data_in, func_args.apply(content_data_in));

    if rounding_local_offset {
        content_data_out.buildings = content_data_out
            .buildings
            .into_iter()
            .map(Building::round_float)
            .collect();
    }

    if sorting_buildings {
        content_data_out.buildings = sort_buildings(content_data_out.buildings, true);
        content_data_out.buildings = fix_buildings_index(content_data_out.buildings);
    }

    (header_data_out, content_data_out)
}

/// 蓝图工具的后端，可编码并输出多种格式的蓝图数据
///
/// # Errors
/// 所有编码或输出时发生的错误在此汇总
pub fn process_back_end(
    header_data: &Header,
    content_data: &Content,
    zopfli_options: &zopfli::Options,
    output_type: &LegalBlueprintFileType,
) -> Result<BlueprintKind, DspbptkError> {
    match output_type {
        LegalBlueprintFileType::Txt => {
            let header_string = codec::header::serialization(header_data);
            let content_string = codec::content::string_from_data(content_data, zopfli_options)?;
            Ok(BlueprintKind::Txt(codec::serialization(
                &header_string,
                &content_string,
            )))
        }
        LegalBlueprintFileType::Content => Ok(BlueprintKind::Content(content_data.to_bin())),
    }
}
