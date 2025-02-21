use thiserror::Error;

#[derive(Error, Debug)]
pub enum DspbptkError<'a> {
    #[error("can not read file: {path:?}, because {source}")]
    CanNotReadFile {
        path: &'a std::path::Path,
        source: std::io::Error,
    },
    #[error("can not write file: {path:?}, because {source}")]
    CanNotWriteFile {
        path: &'a std::path::Path,
        source: std::io::Error,
    },
    #[error("unknown file type")]
    UnknownFileType,
    #[error("broken base64: {0}")]
    BrokenBase64(base64::DecodeError),
    #[error("broken gzip: {0}")]
    BrokenGzip(std::io::Error),
    #[error("broken blueprint")]
    BrokenBlueprint(nom::error::Error<&'a str>),
    #[error("broken header")]
    BrokenHeader(nom::error::Error<&'a str>),
    #[error("broken content")]
    BrokenContent(nom::error::Error<&'a [u8]>),
    #[error("can not compress gzip: {0}")]
    CanNotCompressGzip(std::io::Error),
    #[error("unexpect buildings count: {0}")]
    UnexpectBuildingsCount(<u32 as TryFrom<usize>>::Error),
    #[error("unexpect parameters length: {0}")]
    UnexpectParametersLength(<u16 as TryFrom<usize>>::Error),
    #[error("non-standard uuid: {0}")]
    NonStandardUuid(std::num::TryFromIntError),
    #[error("non-standard index: {0}")]
    NonStandardIndex(std::num::TryFromIntError),
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
pub enum DspbptkInfo<'a> {
    #[error("read file: {0:?}")]
    ReadFile(&'a std::path::PathBuf),
    #[error("write file: {0:?}")]
    WriteFile(&'a std::path::PathBuf),
}
