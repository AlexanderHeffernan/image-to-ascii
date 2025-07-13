use crate::compressor::rle::CompressedGrid;
use flate2::read::GzEncoder;
use flate2::Compression;
use std::io::Read;

// Test-only imports for decompression functionality
#[cfg(test)]
use flate2::read::GzDecoder;

/// Error type for gzip operations.
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

/// Compresses raw byte data using gzip compression.
///
/// This function applies gzip compression to any byte array, used
/// as the final stage after JSON serialization of RLE-compressed data.
///
/// # Arguments
/// * `data` - Raw byte data to compress.
///
/// # Returns
/// * `Ok(Vec<u8>)` - Compressed byte data.
/// * `Err(GzipError)` - Compression failure.
pub fn compress(data: &[u8]) -> Result<Vec<u8>, GzipError> {
    // Create a gzip encoder with default compression level
    let mut encoder = GzEncoder::new(data, Compression::default());
    let mut compressed = Vec::new();
    
    // Read compressed data into buffer
    encoder.read_to_end(&mut compressed)
        .map_err(|e| GzipError::CompressionError(e.to_string()))?;
    
    Ok(compressed)
}

/// Decompresses gzip-compressed byte data.
///
/// This function is only available during testing for validation purposes.
/// Production code only needs compression for sending data to the frontend.
///
/// # Arguments
/// * `compressed` - Gzip-compressed byte data.
///
/// # Returns
/// * `Ok(Vec<u8>)` - Decompressed raw byte data.
/// * `Err(GzipError)` - Decompression failure.
#[cfg(test)]
pub fn decompress(compressed: &[u8]) -> Result<Vec<u8>, GzipError> {
    // Create a gzip decoder
    let mut decoder = GzDecoder::new(compressed);
    let mut decompressed = Vec::new();
    
    // Read decompressed data into buffer
    decoder.read_to_end(&mut decompressed)
        .map_err(|e| GzipError::DecompressionError(e.to_string()))?;
    
    Ok(decompressed)
}

/// Serializes a CompressedGrid to JSON bytes.
///
/// Converts the RLE-compressed grid structure into JSON format for
/// cross-platform compatibility. The JSON data is then passed to
/// the gzip compression function.
///
/// # Arguments
/// * `grid` - RLE-compressed grid data.
///
/// # Returns
/// * `Ok(Vec<u8>)` - JSON-serialized bytes.
/// * `Err(GzipError)` - Serialization failure.
pub fn serialize_compressed_grid(grid: &CompressedGrid) -> Result<Vec<u8>, GzipError> {
    serde_json::to_vec(grid)
        .map_err(|e| GzipError::SerializationError(e.to_string()))
}

/// Deserializes JSON bytes back to CompressedGrid.
///
/// This function is only available during testing for validation purposes.
/// It reverses the JSON serialization process to reconstruct the original
/// RLE-compressed grid structure.
///
/// # Arguments
/// * `data` - JSON-serialized bytes.
///
/// # Returns
/// * `Ok(CompressedGrid)` - Reconstructed grid data.
/// * `Err(GzipError)` - Deserialization failure.
#[cfg(test)]
pub fn deserialize_compressed_grid(data: &[u8]) -> Result<CompressedGrid, GzipError> {
    serde_json::from_slice(data)
        .map_err(|e| GzipError::DeserializationError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test basic compression and decompression roundtrip.
    /// This verifies that data can be compressed and decompressed without loss.
    #[test]
    fn test_basic_compression() {
        let data = b"Hello, world!";
        let compressed = compress(data).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(data, decompressed.as_slice());
    }

    /// Test compression of empty data.
    /// This ensures the compression algorithm can handle the edge case of empty input.
    #[test]
    fn test_empty_data() {
        let data = b"";
        let compressed = compress(data).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(data, decompressed.as_slice());
    }
}