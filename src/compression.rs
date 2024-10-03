use base64::{engine::general_purpose, Engine as _};
use brotli::CompressorWriter;
use std::error::Error;
use std::io::{Cursor, Read, Write};

/// Decompresses the given brotli encoded string.
pub fn decompress_brotli_decode(encoded_data: &str) -> Result<String, Box<dyn Error>> {
    let decoded = general_purpose::STANDARD.decode(encoded_data)?;
    let mut decompressed_data = Vec::new();
    let mut decompressor = brotli::Decompressor::new(Cursor::new(decoded), 4096);
    decompressor.read_to_end(&mut decompressed_data)?;
    Ok(String::from_utf8(decompressed_data)?)
}

/// Decodes the base64 string from the given URL into a zip file and then extracts the file with the given filename from the zip file.
pub fn decompress_encoded_url(url: &str) -> Result<String, Box<dyn Error>> {
    let query_string = url.split('?').collect::<Vec<&str>>()[1];
    let z = query_string
        .split('&')
        .find(|&param| param.starts_with("z="))
        .ok_or("failed to extract")?;

    let z = &z[2..];
    decompress_brotli_decode(z)
}

/// Compresses the given string using brotli encoding and then encodes it in base64.
pub fn compress_brotli_encode(data: &str) -> Result<String, Box<dyn Error>> {
    let mut compressed_data = Vec::new();
    {
        let mut compressor = CompressorWriter::new(&mut compressed_data, 4096, 5, 22); // 4096 is the buffer size, 5 is quality, 22 is lgwin
        compressor.write_all(data.as_bytes())?;
    } // CompressorWriter is flushed and finished when it goes out of scope

    Ok(general_purpose::STANDARD.encode(compressed_data))
}
