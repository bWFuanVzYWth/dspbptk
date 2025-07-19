use std::path::{Path, PathBuf};

use crate::{
    error::DspbptkError::{self, CanNotReadFile, CanNotWriteFile, UnknownFileType},
    workflow::{BlueprintKind, FileType, LegalBlueprintFileType},
};

/// # Errors
/// 可能的原因：
/// 无法为将要输出的文件创建父文件夹，一般是权限之类的问题
pub fn create_father_dir(path: &Path) -> Result<(), DspbptkError> {
    let parent = path
        .parent()
        .map_or_else(|| PathBuf::from("."), std::path::Path::to_path_buf);
    std::fs::create_dir_all(&parent).map_err(|e| CanNotWriteFile {
        path: path.to_path_buf(),
        source: e,
    })
}

#[must_use]
pub fn classify_file_type(entry: &Path) -> FileType {
    entry
        .extension()
        .map_or(FileType::Unknown, |extension| match extension.to_str() {
            Some("txt") => FileType::Blueprint(LegalBlueprintFileType::Txt),
            Some("content") => FileType::Blueprint(LegalBlueprintFileType::Content),
            _ => FileType::Other,
        })
}

fn read_content_file(path: &Path) -> Result<Vec<u8>, DspbptkError> {
    std::fs::read(path).map_err(|e| CanNotReadFile {
        path: path.to_path_buf(),
        source: e,
    })
}

fn read_blueprint_file(path: &Path) -> Result<String, DspbptkError> {
    std::fs::read_to_string(path).map_err(|e| CanNotReadFile {
        path: path.to_path_buf(),
        source: e,
    })
}

/// # Errors
/// 可能的原因：
/// * 文件的后缀名不受支持
pub fn read_file(path: &Path) -> Result<BlueprintKind, DspbptkError> {
    match classify_file_type(path) {
        FileType::Blueprint(LegalBlueprintFileType::Txt) => {
            let blueprint_string = read_blueprint_file(path)?;
            Ok(BlueprintKind::Txt(blueprint_string))
        }
        FileType::Blueprint(LegalBlueprintFileType::Content) => {
            let content_bin = read_content_file(path)?;
            Ok(BlueprintKind::Content(content_bin))
        }
        _ => Err(UnknownFileType),
    }
}

fn write_blueprint_file(path: &Path, blueprint: String) -> Result<(), DspbptkError> {
    create_father_dir(path)?;
    std::fs::write(path, blueprint).map_err(|e| CanNotWriteFile {
        path: path.to_path_buf(),
        source: e,
    })
}

fn write_content_file(path: &Path, content: Vec<u8>) -> Result<(), DspbptkError> {
    create_father_dir(path)?;
    std::fs::write(path, content).map_err(|e| CanNotWriteFile {
        path: path.to_path_buf(),
        source: e,
    })
}

/// # Errors
/// 可能的错误：
/// * 无法为待写入硬盘的文件创建文件夹，一般是权限之类的问题
pub fn write_file(path: &Path, blueprint_kind: BlueprintKind) -> Result<(), DspbptkError> {
    match blueprint_kind {
        BlueprintKind::Txt(blueprint) => write_blueprint_file(path, blueprint),
        BlueprintKind::Content(content) => write_content_file(path, content),
    }
}
