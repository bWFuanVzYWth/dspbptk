use std::path::{Path, PathBuf};

use crate::{
    blueprint::{
        self,
        content::{self, ContentData, string_from_data},
        header::{self, HeaderData},
    },
    error::{
        DspbptkError::{self, CanNotReadFile, CanNotWriteFile, UnknownFileType},
        DspbptkWarn,
    },
};

pub enum BlueprintKind {
    Txt(String),
    Content(Vec<u8>),
}

pub enum FileType {
    Unknown,
    Other,
    Txt,
    Content,
}

/// # Errors
/// 可能的原因：
/// 无法为将要输出的文件创建父文件夹，一般是权限之类的问题
pub fn create_father_dir(path: &PathBuf) -> Result<(), DspbptkError> {
    let parent = path
        .parent()
        .map_or_else(|| PathBuf::from("."), std::path::Path::to_path_buf);
    std::fs::create_dir_all(&parent).map_err(|e| CanNotWriteFile { path, source: e })
}

#[must_use]
pub fn classify_file_type(entry: &Path) -> FileType {
    entry
        .extension()
        .map_or(FileType::Unknown, |extension| match extension.to_str() {
            Some("txt") => FileType::Txt,
            Some("content") => FileType::Content,
            _ => FileType::Other,
        })
}

fn read_content_file(path: &Path) -> Result<Vec<u8>, DspbptkError> {
    std::fs::read(path).map_err(|e| CanNotReadFile { path, source: e })
}

fn read_blueprint_file(path: &Path) -> Result<String, DspbptkError> {
    std::fs::read_to_string(path).map_err(|e| CanNotReadFile { path, source: e })
}

/// # Errors
/// 可能的原因：
/// * 文件的后缀名不受支持
pub fn read_file(path: &Path) -> Result<BlueprintKind, DspbptkError> {
    match classify_file_type(path) {
        FileType::Txt => {
            let blueprint_string = read_blueprint_file(path)?;
            Ok(BlueprintKind::Txt(blueprint_string))
        }
        FileType::Content => {
            let content_bin = read_content_file(path)?;
            Ok(BlueprintKind::Content(content_bin))
        }
        _ => Err(UnknownFileType),
    }
}

fn write_blueprint_file(path: &PathBuf, blueprint: String) -> Result<(), DspbptkError> {
    create_father_dir(path)?;
    std::fs::write(path, blueprint).map_err(|e| CanNotWriteFile { path, source: e })
}

fn write_content_file(path: &PathBuf, content: Vec<u8>) -> Result<(), DspbptkError> {
    create_father_dir(path)?;
    std::fs::write(path, content).map_err(|e| CanNotWriteFile { path, source: e })
}

/// # Errors
/// 可能的错误：
/// * 无法为待写入硬盘的文件创建文件夹，一般是权限之类的问题
pub fn write_file(path: &PathBuf, blueprint_kind: BlueprintKind) -> Result<(), DspbptkError> {
    match blueprint_kind {
        BlueprintKind::Txt(blueprint) => write_blueprint_file(path, blueprint),
        BlueprintKind::Content(content) => write_content_file(path, content),
    }
}

/// 蓝图工具的前端，可读取并解码多种格式的蓝图数据
///
/// # Errors
/// 所有读取或解码时发生的错误在此汇总
pub fn process_front_end<'a>(
    blueprint: &'a BlueprintKind,
    blueprint_content_bin: &'a mut Vec<u8>,
) -> Result<(HeaderData, ContentData, Vec<DspbptkWarn>), DspbptkError<'a>> {
    match blueprint {
        BlueprintKind::Txt(blueprint_string) => {
            // let start = std::time::Instant::now();

            let (blueprint_data, warns_blueprint) = blueprint::parse(blueprint_string)?;
            content::bin_from_string(blueprint_content_bin, blueprint_data.content)?;
            let (content_data, warns_content) =
                ContentData::from_bin(blueprint_content_bin.as_slice())?;
            let (header_data, warns_header) = header::parse(blueprint_data.header)?;

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
            let (content_data, warns_content) = ContentData::from_bin(content_bin)?;
            let header_data = HeaderData::default();
            Ok((header_data, content_data, warns_content))
        }
    }
}

/// 蓝图工具的后端，可编码并输出多种格式的蓝图数据
///
/// # Errors
/// 所有编码或输出时发生的错误在此汇总
pub fn process_back_end<'a>(
    header_data: &HeaderData,
    content_data: &ContentData,
    zopfli_options: &zopfli::Options,
    output_type: &FileType,
) -> Result<BlueprintKind, DspbptkError<'a>> {
    match output_type {
        FileType::Txt => {
            let header_string = header::serialization(header_data);
            let content_string = string_from_data(content_data, zopfli_options)?;
            Ok(BlueprintKind::Txt(blueprint::serialization(
                &header_string,
                &content_string,
            )))
        }
        FileType::Content => Ok(BlueprintKind::Content(content_data.to_bin())),
        _ => Err(UnknownFileType),
    }
}
