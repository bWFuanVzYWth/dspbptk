use thiserror::Error;

#[derive(Error, Debug)]
pub enum DspbptkError<'a> {
    #[error("Can not read file: {path:?}, because {source}")]
    CanNotReadFile {
        path: std::ffi::OsString,
        source: std::io::Error,
    },
    #[error("Can not write file: {path:?}, because {source}")]
    CanNotWriteFile {
        path: std::ffi::OsString,
        source: std::io::Error,
    },
    #[error("not blueprint: {0:?}")]
    NotBlueprint(std::ffi::OsString),
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

#[derive(Error, Debug)]
pub enum DspbptkWarn<'a> {
    #[error("few unknown after blueprint: {0:?}")]
    FewUnknownAfterBlueprint(&'a str),
    #[error("lot unknown after blueprint: length = {0}")]
    LotUnknownAfterBlueprint(usize),
    #[error("few unknown after content: {0:?}")]
    FewUnknownAfterContent(&'a [u8]),
    #[error("lot unknown after content: length = {0}")]
    LotUnknownAfterContent(usize),
    #[error("unexpected MD5F: expected = {0:?}, actual = {1:?}")]
    UnexpectedMD5F(&'a str, &'a str),
}

#[derive(Error, Debug)]
pub enum DspbptkInfo {
    #[error("read file: {0:?}")]
    ReadFile(std::ffi::OsString),
    #[error("write file: {0:?}")]
    WriteFile(std::ffi::OsString),
}
