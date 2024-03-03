use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use base64::{Engine as _, engine::general_purpose};


use brotli::CompressorWriter;


pub fn decompress_brotli_decode(encoded_data: &str) -> Option<String> {
    let decoded = general_purpose::STANDARD.decode(encoded_data);
    if !decoded.is_ok() {
        return None
    }
    let mut decompressed_data = Vec::new();
    let mut decompressor = brotli::Decompressor::new(Cursor::new(decoded.unwrap()), 4096);
    decompressor.read_to_end(&mut decompressed_data).unwrap();
    Option::from(String::from_utf8(decompressed_data).unwrap())
}

/// Decodes the base64 string from the given URL into a zip file and then extracts the file with the given filename from the zip file.
pub fn decompress_encoded_url(url: &str) -> Option<String> {
    let query_string = url.split('?').collect::<Vec<&str>>()[1];
    let z = query_string
        .split('&')
        .find(|&param| param.starts_with("z="))?;
    let z = &z[2..];

    decompress_brotli_decode(z)
}

pub fn compress_brotli_encode(data: &str) -> String {
    let mut compressed_data = Vec::new();
    {
        let mut compressor = CompressorWriter::new(&mut compressed_data, 4096, 5, 22); // 4096 is the buffer size, 5 is quality, 22 is lgwin
        compressor.write_all(data.as_bytes()).unwrap();
    } // CompressorWriter is flushed and finished when it goes out of scope

    general_purpose::STANDARD.encode(compressed_data)
}

#[cfg(test)]
mod tests {
    use crate::fixtures::{DINING_PHILOSOPHERS};

    use super::*;

    #[test]
    fn test_unzip_base64_encoded() {
        let encoded = compress_brotli_encode(DINING_PHILOSOPHERS);
        println!("encoded: http://localhost:3000/?z={:}", encoded);
        let decoded = decompress_brotli_decode(&encoded).unwrap();
        assert_eq!(decoded, DINING_PHILOSOPHERS);
    }
}
