pub mod io;
pub mod process;

use clap::ValueEnum;

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
