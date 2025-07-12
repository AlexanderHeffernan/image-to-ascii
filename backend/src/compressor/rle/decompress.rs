use crate::converter::ascii_pixel::AsciiPixel;
use crate::compressor::rle::types::{CompressedGrid, CompressionError};

/// Decompress a compressed grid back to the original ASCII grid
pub fn decompress_grid(compressed: &CompressedGrid) -> Result<Vec<Vec<AsciiPixel>>, CompressionError> {
    if compressed.width == 0 || compressed.height == 0 {
        return Ok(vec![]);
    }

    let mut result = Vec::new();

    for (row_idx, compressed_row) in compressed.rows.iter().enumerate() {
        let row = decompress_row(compressed_row)?;
        
        // Verify row length matches expected width
        if row.len() != compressed.width as usize {
            return Err(CompressionError::DecompressionError(
                format!("Row {} has length {} but expected {}", row_idx, row.len(), compressed.width)
            ));
        }

        result.push(row);
    }

    // Verify we have the right number of rows
    if result.len() != compressed.height as usize {
        return Err(CompressionError::DecompressionError(
            format!("Grid has {} rows but expected {}", result.len(), compressed.height)
        ));
    }

    Ok(result)
}

/// Decompress a single row
fn decompress_row(compressed_row: &[super::types::RleEntry]) -> Result<Vec<AsciiPixel>, CompressionError> {
    let mut row = Vec::new();
    
    for entry in compressed_row {
        if entry.count == 0 {
            return Err(CompressionError::DecompressionError(
                "Invalid RLE entry with count 0".to_string()
            ));
        }
        
        for _ in 0..entry.count {
            row.push(entry.pixel.clone());
        }
    }

    Ok(row)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::converter::ascii_pixel::AsciiPixel;
    use crate::compressor::rle::types::{CompressedGrid, RleEntry};
    use crate::compressor::rle::compress::compress_grid;

    #[test]
    fn test_empty_grid() {
        let compressed = CompressedGrid::new(0, 0, false);
        let decompressed = decompress_grid(&compressed).unwrap();
        assert_eq!(decompressed.len(), 0);
    }

    #[test]
    fn test_roundtrip_compression() {
        let original = vec![
            vec![
                AsciiPixel{ch: 'A', rgb: None},
                AsciiPixel{ch: 'A', rgb: None},
                AsciiPixel{ch: 'B', rgb: None},
            ],
            vec![
                AsciiPixel{ch: 'C', rgb: None},
                AsciiPixel{ch: 'C', rgb: None},
                AsciiPixel{ch: 'C', rgb: None},
            ],
        ];
        
        let compressed = compress_grid(&original).unwrap();
        let decompressed = decompress_grid(&compressed).unwrap();
        
        assert_eq!(original, decompressed);
    }

    #[test]
    fn test_invalid_count() {
        let mut compressed = CompressedGrid::new(1, 1, false);
        compressed.rows.push(vec![RleEntry {
            count: 0,
            pixel: AsciiPixel{ch: 'A', rgb: None},
        }]);
        
        assert!(decompress_grid(&compressed).is_err());
    }
}