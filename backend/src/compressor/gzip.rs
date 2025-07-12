use crate::converter::ascii_pixel::AsciiPixel;
use crate::compressor::rle::CompressedGrid;
use flate2::read::{GzDecoder, GzEncoder};
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug)]
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
pub fn decompress(compressed: &[u8]) -> Result<Vec<u8>, GzipError> {
    let mut decoder = GzDecoder::new(compressed);
    let mut decompressed = Vec::new();
    
    decoder.read_to_end(&mut decompressed)
        .map_err(|e| GzipError::DecompressionError(e.to_string()))?;
    
    Ok(decompressed)
}

// Serialize a CompressedGrid to bytes
pub fn serialize_compressed_grid(grid: &CompressedGrid) -> Result<Vec<u8>, GzipError> {
    bincode::serialize(grid)
        .map_err(|e| GzipError::SerializationError(e.to_string()))
}

// Deserialize bytes back to CompressedGrid
pub fn deserialize_compressed_grid(data: &[u8]) -> Result<CompressedGrid, GzipError> {
    bincode::deserialize(data)
        .map_err(|e| GzipError::DeserializationError(e.to_string()))
}

// Serialize a raw grid to bytes (for gzip-only compression)
pub fn serialize_grid(grid: &[Vec<AsciiPixel>]) -> Result<Vec<u8>, GzipError> {
    bincode::serialize(grid)
        .map_err(|e| GzipError::SerializationError(e.to_string()))
}

// Calculate compression ratio
pub fn compression_ratio(original: &[u8], compressed: &[u8]) -> f64 {
    if original.is_empty() {
        return 0.0;
    }
    compressed.len() as f64 / original.len() as f64
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

    #[test]
    fn test_compression_ratio() {
        let data = b"AAAAAAAAAAAAAAAAAAAAAAAA";
        let compressed = compress(data).unwrap();
        let ratio = compression_ratio(data, &compressed);
        assert!(ratio > 0.0 && ratio < 1.0);
    }
}