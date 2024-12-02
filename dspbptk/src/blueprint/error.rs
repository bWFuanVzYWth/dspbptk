#[derive(Debug, PartialEq)]
pub enum DspbptkError {
    ReadBrokenBase64,
    ReadBrokenGzip,
    CanNotCompressGzip,
    CanNotParseBluePrint,
    CanNotParseContent,
    CanNotParseHeader,
    IllegalCompressParameters,
}

impl std::error::Error for DspbptkError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for DspbptkError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DspbptkError::ReadBrokenBase64 => write!(f, "Read broken base64"),
            DspbptkError::ReadBrokenGzip => write!(f, "Read broken GZIP"),
            DspbptkError::IllegalCompressParameters => write!(f, "Illegal compress parameters"),
            DspbptkError::CanNotCompressGzip => write!(f, "Can not compress GZIP"),
            DspbptkError::CanNotParseBluePrint => write!(f, "Can not parse blueprint"),
            DspbptkError::CanNotParseContent => write!(f, "Can not parse content"),
            DspbptkError::CanNotParseHeader => write!(f, "Can not parse header"),
        }
    }
}
