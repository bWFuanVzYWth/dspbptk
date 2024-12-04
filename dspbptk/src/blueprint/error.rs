#[derive(Debug, PartialEq)]
pub enum BlueprintError<E> {
    ReadBrokenBase64(E),
    ReadBrokenGzip(E),

    CanNotDeserializationBluePrint(E),
    CanNotDeserializationHeader(E),
    CanNotDeserializationContent(E),

    CanNotCompressGzip(E),
}

impl<E: std::error::Error + 'static> std::error::Error for BlueprintError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BlueprintError::ReadBrokenBase64(e) => Some(e),
            BlueprintError::ReadBrokenGzip(e) => Some(e),

            BlueprintError::CanNotDeserializationBluePrint(e) => Some(e),
            BlueprintError::CanNotDeserializationHeader(e) => Some(e),
            BlueprintError::CanNotDeserializationContent(e) => Some(e),

            BlueprintError::CanNotCompressGzip(e) => Some(e),
        }
    }
}

impl<E: std::error::Error> std::fmt::Display for BlueprintError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BlueprintError::ReadBrokenBase64(e) => write!(f, "read broken base64: {:#?}", e),
            BlueprintError::ReadBrokenGzip(e) => write!(f, "read broken gzip: {:#?}", e),

            BlueprintError::CanNotDeserializationBluePrint(e) => write!(f, "can not parse blueprint: {:#?}", e),
            BlueprintError::CanNotDeserializationHeader(e) => write!(f, "can not parse header: {:#?}", e),
            BlueprintError::CanNotDeserializationContent(e) => write!(f, "can not parse memory_stream: {:#?}", e),

            BlueprintError::CanNotCompressGzip(e) => write!(f, "can not compress gzip: {:#?}", e),
        }
    }
}
