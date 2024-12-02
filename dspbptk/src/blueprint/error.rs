#[derive(Debug, PartialEq)]
pub enum DspbptkError<E> {
    NomError(E),
    ReadBrokenBase64,
    ReadBrokenGzip,
    CanNotCompressGzip,
    CanNotParseBluePrint,
    CanNotParseContent,
    CanNotParseHeader,
    IllegalCompressParameters,
}

impl<E: std::error::Error + 'static> std::error::Error for DspbptkError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DspbptkError::NomError(e) => Some(e),
            _ => None,
        }
    }
}

impl<E: std::error::Error> std::fmt::Display for DspbptkError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DspbptkError::NomError(e) => write!(f, "NomError: {:#?}", e),
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
