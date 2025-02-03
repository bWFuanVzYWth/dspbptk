#[derive(Debug, PartialEq)]
pub enum BlueprintError<E> {
    CanNotReadFile(E),
    CanNotWriteFile(E),

    ReadBrokenBase64(E),
    ReadBrokenGzip(E),

    CanNotDeserializationBluePrint(E),
    CanNotDeserializationHeader(E),
    CanNotDeserializationContent(E),

    CanNotCompressGzip(E),
}

// TODO 简化异常处理，考虑使用AnyHow之类的库
// TODO 出现异常的时候，打印导致错误的输入数据

impl<E: std::error::Error + 'static> std::error::Error for BlueprintError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BlueprintError::CanNotReadFile(e) => Some(e),
            BlueprintError::CanNotWriteFile(e) => Some(e),

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
            BlueprintError::CanNotWriteFile(e) => write!(f, "can not write file: {:#?}", e),
            BlueprintError::CanNotReadFile(e) => write!(f, "can not read file: {:#?}", e),

            BlueprintError::ReadBrokenBase64(e) => write!(f, "read broken base64: {:#?}", e),
            BlueprintError::ReadBrokenGzip(e) => write!(f, "read broken gzip: {:#?}", e),

            BlueprintError::CanNotDeserializationBluePrint(e) => {
                write!(f, "can not parse blueprint: {:#?}", e)
            }
            BlueprintError::CanNotDeserializationHeader(e) => {
                write!(f, "can not parse header: {:#?}", e)
            }
            BlueprintError::CanNotDeserializationContent(e) => {
                write!(f, "can not parse memory_stream: {:#?}", e)
            }

            BlueprintError::CanNotCompressGzip(e) => write!(f, "can not compress gzip: {:#?}", e),
        }
    }
}
