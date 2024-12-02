#[derive(Debug, PartialEq)]
pub enum DspbptkError<E> {
    ReadBrokenBase64(E),
    ReadBrokenGzip(E),

    CanNotParseBluePrint(E),
    CanNotParseHeader(E),
    CanNotParseContent(E),

    CanNotCompressGzip(E),
}

impl<E: std::error::Error + 'static> std::error::Error for DspbptkError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DspbptkError::ReadBrokenBase64(e) => Some(e),
            DspbptkError::ReadBrokenGzip(e) => Some(e),

            DspbptkError::CanNotParseBluePrint(e) => Some(e),
            DspbptkError::CanNotParseHeader(e) => Some(e),
            DspbptkError::CanNotParseContent(e) => Some(e),

            DspbptkError::CanNotCompressGzip(e) => Some(e),
        }
    }
}

impl<E: std::error::Error> std::fmt::Display for DspbptkError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DspbptkError::ReadBrokenBase64(e) => write!(f, "read broken base64: {:#?}", e),
            DspbptkError::ReadBrokenGzip(e) => write!(f, "read broken GZIP: {:#?}", e),

            DspbptkError::CanNotParseBluePrint(e) => write!(f, "can not parse blueprint: {:#?}", e),
            DspbptkError::CanNotParseHeader(e) => write!(f, "can not parse header: {:#?}", e),
            DspbptkError::CanNotParseContent(e) => write!(f, "can not parse content: {:#?}", e),

            DspbptkError::CanNotCompressGzip(e) => write!(f, "can not compress GZIP: {:#?}", e),
        }
    }
}
