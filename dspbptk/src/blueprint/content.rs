use crate::blueprint::error::BlueprintError;
use crate::blueprint::error::BlueprintError::*;

fn decode_base64(base64_string: &str) -> Result<Vec<u8>, BlueprintError<String>> {
    use base64::prelude::*;
    match BASE64_STANDARD.decode(base64_string) {
        Ok(result) => Ok(result),
        Err(why) => Err(ReadBrokenBase64(why.to_string())),
    }
}

fn encode_base64(bin: Vec<u8>) -> String {
    use base64::prelude::*;
    BASE64_STANDARD.encode(bin)
}

fn decompress_gzip(gzip_stream: Vec<u8>) -> Result<Vec<u8>, BlueprintError<String>> {
    use flate2::read::GzDecoder;
    use std::io::Read;
    let mut decoder = GzDecoder::new(gzip_stream.as_slice());
    let mut memory_stream = Vec::new();
    match decoder.read_to_end(&mut memory_stream) {
        Ok(_) => Ok(memory_stream),
        Err(why) => Err(ReadBrokenGzip(why.to_string())),
    }
}

fn compress_gzip_zopfli(
    bin: Vec<u8>,
    iteration_count: u64,
    iterations_without_improvement: u64,
    maximum_block_splits: u16,
) -> Result<Vec<u8>, BlueprintError<std::io::Error>> {
    use std::num::NonZero;
    let options = zopfli::Options {
        // 防呆不防傻，这两个expect只有用户瞎几把输入参数才会炸
        iteration_count: NonZero::new(iteration_count)
            .expect("Fatal error: iteration_count must greater than 0"),
        iterations_without_improvement: NonZero::new(iterations_without_improvement)
            .expect("Fatal error: iterations_without_improvement must greater than 0"),
        maximum_block_splits: maximum_block_splits,
    };

    let mut gzip_stream = Vec::new();

    match zopfli::compress(
        options,
        zopfli::Format::Gzip,
        bin.as_slice(),
        &mut gzip_stream,
    ) {
        Ok(_) => Ok(gzip_stream),
        Err(why) => Err(CanNotCompressGzip(why)),
    }
}

fn compress_gzip(bin: Vec<u8>) -> Result<Vec<u8>, BlueprintError<std::io::Error>> {
    compress_gzip_zopfli(bin, 256, u64::MAX, 0)
}

pub fn memory_stream_from_content(content: &str) -> Result<Vec<u8>, BlueprintError<String>> {
    decompress_gzip(decode_base64(content)?)
}

pub fn content_from_memory_stream(
    memory_stream: Vec<u8>,
) -> Result<String, BlueprintError<String>> {
    match compress_gzip(memory_stream) {
        Ok(gzip_stream) => Ok(encode_base64(gzip_stream)),
        Err(why) => Err(CanNotCompressGzip(why.to_string())),
    }
}
