use thiserror::Error;

#[derive(Error, Debug)]
pub enum DspbptkError {
    #[error("can not read file: {path:?}, because {source}")]
    CanNotReadFile {
        path: std::path::PathBuf,
        source: std::io::Error,
    },
    #[error("can not write file: {path:?}, because {source}")]
    CanNotWriteFile {
        path: std::path::PathBuf,
        source: std::io::Error,
    },
    #[error("unknown file type")]
    UnknownFileType,
    #[error("broken base64: {0}")]
    BrokenBase64(base64::DecodeError),
    #[error("broken gzip: {0}")]
    BrokenGzip(std::io::Error),
    #[error("broken blueprint")]
    BrokenBlueprint(nom::error::Error<String>),
    #[error("broken header")]
    BrokenHeader(nom::error::Error<String>),
    #[error("broken content")]
    BrokenContent(nom::error::Error<Vec<u8>>),
    #[error("can not compress gzip: {0}")]
    CanNotCompressGzip(std::io::Error),
    #[error("unexpect buildings count: {0}")]
    UnexpectBuildingsCount(<u32 as TryFrom<usize>>::Error),
    #[error("unexpect parameters length: {0}")]
    UnexpectParametersLength(<u16 as TryFrom<usize>>::Error),
    #[error("out range uuid: {0}")]
    TryFromUuidError(std::num::TryFromIntError),
    #[error("out range index: {0}")]
    TryFromIndexError(std::num::TryFromIntError),
}

#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum DspbptkWarn {
    #[error("few unknown after blueprint: {0:?}")]
    FewUnknownAfterBlueprint(String),
    #[error("lot unknown after blueprint: length = {0}")]
    LotUnknownAfterBlueprint(usize),
    #[error("few unknown after content: {0:?}")]
    FewUnknownAfterContent(Vec<u8>),
    #[error("lot unknown after content: length = {0}")]
    LotUnknownAfterContent(usize),
    #[error("unknown after header")]
    UnknownAfterHeader,
    #[error("unexpected MD5F: expected = {0:?}, actual = {1:?}")]
    UnexpectedMD5F(String, String),
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum DspbptkEditWarn {
    #[error("non-standard local_offset: {0:?}")]
    NonStandardLocalOffset([f64; 3]),
}

#[derive(Error, Debug)]
pub enum DspbptkInfo {
    #[error("read file: {0:?}")]
    ReadFile(std::path::PathBuf),
    #[error("write file: {0:?}")]
    WriteFile(std::path::PathBuf),
}
