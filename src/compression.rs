use base64::{engine::general_purpose, Engine as _};
use std::io::{Cursor, Read, Write};
use brotli::CompressorWriter;
use std::error::Error;

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

#[cfg(test)]
mod tests {
    use crate::fixtures::DINING_PHILOSOPHERS;
    use crate::petri_net::PetriNet;

    use super::*;

    #[test]
    fn test_unzip_base64_encoded() {
        let encoded = compress_brotli_encode(DINING_PHILOSOPHERS).expect("Compression failed");
        println!("encoded: http://localhost:3000/?z={encoded:}");
        let decoded = decompress_brotli_decode(&encoded).expect("Decompression failed");
        assert_eq!(decoded, DINING_PHILOSOPHERS);
    }

    #[test]
    fn test_unzip_url() {
        let url = "http://localhost:3000/?z=GzkCIBwHdqMPWUYyo7XgaT/B09w+1fHywu1u31IMRQwiCxaRsTAxQRT6UodF4e9vcmthITygLrPfojnB4nxsskw21O/iE3GRG82+n/aPgzT++TW8fY5765PjEAvRHLk1fa0Atw8uCVzrgniE9AOCxwJt0eNbZxX3GlCwKSXlDBVIj2qWMSpoWCuQ0SZF4WJKQu7IYz8DzVzPNGg5hqbWWqtzXBixNz9qkiODzShUClkETwDocbjtBJp9Wh5QW8T8PXrgq9nCDI3qaA==";
        let decoded = decompress_encoded_url(url).expect("Decompression failed");
        let net = PetriNet::from_json_str(decoded).expect("Failed to parse the given JSON.");
        assert_eq!(net.places.len(), 4);
        assert_eq!(net.transitions.len(), 1);
    }
}