use std::path::PathBuf;

use crate::{
    blueprint::{
        self,
        content::{self, bin_from_data, string_from_data, ContentData},
        header::{self, HeaderData},
    },
    error::{
        DspbptkError::{self, *},
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

pub fn create_father_dir(path: &PathBuf) -> Result<(), DspbptkError> {
    let parent = match path.parent() {
        Some(p) => p.to_path_buf(),
        None => PathBuf::from("."),
    };
    std::fs::create_dir_all(&parent).map_err(|e| CanNotWriteFile {
        path: path,
        source: e,
    })
}

pub fn classify_file_type(entry: &std::path::PathBuf) -> FileType {
    if let Some(extension) = entry.extension() {
        match extension.to_str() {
            Some("txt") => FileType::Txt,
            Some("content") => FileType::Content,
            _ => FileType::Other,
        }
    } else {
        FileType::Unknown
    }
}

fn read_content_file(path: &std::path::PathBuf) -> Result<Vec<u8>, DspbptkError> {
    std::fs::read(path).map_err(|e| CanNotReadFile {
        path: path,
        source: e,
    })
}

fn read_blueprint_file(path: &std::path::PathBuf) -> Result<String, DspbptkError> {
    std::fs::read_to_string(path).map_err(|e| CanNotReadFile {
        path: path,
        source: e,
    })
}

pub fn read_file(path: &PathBuf) -> Result<BlueprintKind, DspbptkError> {
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
    std::fs::write(path, blueprint).map_err(|e| CanNotWriteFile {
        path: path,
        source: e,
    })
}

fn write_content_file(path: &PathBuf, content: Vec<u8>) -> Result<(), DspbptkError> {
    create_father_dir(path)?;
    std::fs::write(path, content).map_err(|e| CanNotWriteFile {
        path: path,
        source: e,
    })
}

pub fn write_file(path: &PathBuf, blueprint_kind: BlueprintKind) -> Result<(), DspbptkError> {
    match blueprint_kind {
        BlueprintKind::Txt(blueprint) => write_blueprint_file(path, blueprint),
        BlueprintKind::Content(content) => write_content_file(path, content),
    }
}

pub fn process_front_end<'a>(
    blueprint: &'a BlueprintKind,
    blueprint_content_bin: &'a mut Vec<u8>,
) -> Result<(HeaderData, ContentData, Vec<DspbptkWarn>), DspbptkError<'a>> {
    match blueprint {
        BlueprintKind::Txt(blueprint_string) => {
            let (blueprint_data, warns_blueprint) = blueprint::parse(&blueprint_string)?;
            content::bin_from_string(blueprint_content_bin, &blueprint_data.content)?;
            let (content_data, warns_content) =
                content::data_from_bin(blueprint_content_bin.as_slice())?;
            let (header_data, warns_header) = header::parse(&blueprint_data.header)?;
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
            let (content_data, warns_content) = blueprint::content::data_from_bin(&content_bin)?;
            const HEADER: &str = "BLUEPRINT:0,0,0,0,0,0,0,0,0,0.0.0.0,,";
            let (header_data, warns_header) = blueprint::header::parse(HEADER)?;
            Ok((
                header_data,
                content_data,
                [warns_content.as_slice(), warns_header.as_slice()].concat(),
            ))
        }
    }
}

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
        FileType::Content => Ok(BlueprintKind::Content(bin_from_data(content_data))),
        _ => Err(UnknownFileType),
    }
}