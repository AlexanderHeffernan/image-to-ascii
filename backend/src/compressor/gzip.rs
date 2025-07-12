use crate::compressor::rle::CompressedGrid;
use flate2::read::GzEncoder;
use flate2::Compression;
use std::io::Read;

#[cfg(test)]
use crate::converter::AsciiPixel;

#[cfg(test)]
use flate2::read::GzDecoder;

#[derive(Debug)]
#[allow(dead_code)]
pub enum GzipError {
    CompressionError(String),
    DecompressionError(String),
    SerializationError(String),
    DeserializationError(String),
}

impl std::fmt::Display for GzipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GzipError::CompressionError(msg) => write!(f, "Compression error: {}", msg),
            GzipError::DecompressionError(msg) => write!(f, "Decompression error: {}", msg),
            GzipError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            GzipError::DeserializationError(msg) => write!(f, "Deserialization error: {}", msg),
        }
    }
}

impl std::error::Error for GzipError {}

// Basic gzip compression of byte data
pub fn compress(data: &[u8]) -> Result<Vec<u8>, GzipError> {
    let mut encoder = GzEncoder::new(data, Compression::default());
    let mut compressed = Vec::new();
    
    encoder.read_to_end(&mut compressed)
        .map_err(|e| GzipError::CompressionError(e.to_string()))?;
    
    Ok(compressed)
}

// Basic gzip decompression of byte data
#[cfg(test)]
pub fn decompress(compressed: &[u8]) -> Result<Vec<u8>, GzipError> {
    let mut decoder = GzDecoder::new(compressed);
    let mut decompressed = Vec::new();
    
    decoder.read_to_end(&mut decompressed)
        .map_err(|e| GzipError::DecompressionError(e.to_string()))?;
    
    Ok(decompressed)
}

// Serialize a CompressedGrid to JSON bytes (changed from bincode)
pub fn serialize_compressed_grid(grid: &CompressedGrid) -> Result<Vec<u8>, GzipError> {
    serde_json::to_vec(grid)
        .map_err(|e| GzipError::SerializationError(e.to_string()))
}

// Deserialize JSON bytes back to CompressedGrid (changed from bincode)
#[cfg(test)]
pub fn deserialize_compressed_grid(data: &[u8]) -> Result<CompressedGrid, GzipError> {
    serde_json::from_slice(data)
        .map_err(|e| GzipError::DeserializationError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_compression() {
        let data = b"Hello, world!";
        let compressed = compress(data).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(data, decompressed.as_slice());
    }

    #[test]
    fn test_empty_data() {
        let data = b"";
        let compressed = compress(data).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(data, decompressed.as_slice());
    }
}