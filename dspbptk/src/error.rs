use thiserror::Error;

// FIXME nom::error::ErrorKind

#[derive(Error, Debug)]
pub enum DspbptkError<'a> {
    #[error("Can not read file: {0}")]
    CanNotReadFile(std::io::Error),
    #[error("Can not write file: {0}")]
    CanNotWriteFile(std::io::Error),
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
