pub mod io;
pub mod process;

use std::path::{Path, PathBuf};

use clap::ValueEnum;

use crate::{
    blueprint::{
        self, codec,
        data::{content::Content, header::Header},
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

#[derive(ValueEnum, Clone, Debug)]
pub enum LegalBlueprintFileType {
    Txt,
    Content,
}

pub enum FileType {
    Blueprint(LegalBlueprintFileType),
    Unknown,
    Other,
}
