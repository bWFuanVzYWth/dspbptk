use std::path::PathBuf;

use crate::error::{DspbptkError, DspbptkError::*};

pub enum BlueprintKind {
    Blueprint(String),
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
            Ok(BlueprintKind::Blueprint(blueprint_string))
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
        BlueprintKind::Blueprint(blueprint) => write_blueprint_file(path, blueprint),
        BlueprintKind::Content(content) => write_content_file(path, content),
    }
}
