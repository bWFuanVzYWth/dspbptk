#[derive(Debug, PartialEq)]
pub enum BlueprintError<E> {
    ReadBrokenBase64(E),
    ReadBrokenGzip(E),

    CanNotParseBluePrint(E),
    CanNotParseHeader(E),
    CanNotParseMemoryStream(E),

    CanNotCompressGzip(E),
}

impl<E: std::error::Error + 'static> std::error::Error for BlueprintError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BlueprintError::ReadBrokenBase64(e) => Some(e),
            BlueprintError::ReadBrokenGzip(e) => Some(e),

            BlueprintError::CanNotParseBluePrint(e) => Some(e),
            BlueprintError::CanNotParseHeader(e) => Some(e),
            BlueprintError::CanNotParseMemoryStream(e) => Some(e),

            BlueprintError::CanNotCompressGzip(e) => Some(e),
        }
    }
}

impl<E: std::error::Error> std::fmt::Display for BlueprintError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BlueprintError::ReadBrokenBase64(e) => write!(f, "read broken base64: {:#?}", e),
            BlueprintError::ReadBrokenGzip(e) => write!(f, "read broken GZIP: {:#?}", e),

            BlueprintError::CanNotParseBluePrint(e) => write!(f, "can not parse blueprint: {:#?}", e),
            BlueprintError::CanNotParseHeader(e) => write!(f, "can not parse header: {:#?}", e),
            BlueprintError::CanNotParseMemoryStream(e) => write!(f, "can not parse memory_stream: {:#?}", e),

            BlueprintError::CanNotCompressGzip(e) => write!(f, "can not compress GZIP: {:#?}", e),
        }
    }
}
