use thiserror::Error;

#[derive(Error, Debug)]
pub enum DspbptkError<'a> {
    #[error("Can not read file: {path:?}, because {source}")]
    CanNotReadFile {
        path: &'a std::path::PathBuf,
        source: std::io::Error,
    },
    #[error("Can not write file: {path:?}, because {source}")]
    CanNotWriteFile {
        path: &'a std::path::PathBuf,
        source: std::io::Error,
    },
    #[error("not blueprint")]
    NotBlueprint,
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
}

#[derive(Error, Debug, PartialEq, Clone)]
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

#[derive(Error, Debug)]
pub enum DspbptkInfo<'a> {
    #[error("read file: {0:?}")]
    ReadFile(&'a std::path::PathBuf),
    #[error("write file: {0:?}")]
    WriteFile(&'a std::path::PathBuf),
}
