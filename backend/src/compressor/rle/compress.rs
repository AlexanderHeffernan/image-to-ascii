use crate::converter::ascii_pixel::AsciiPixel;
use crate::compressor::rle::types::{RleEntry, CompressedGrid, CompressionError};

/// Compress an ASCII grid using run-length encoding
pub fn compress_grid(grid: &[Vec<AsciiPixel>]) -> Result<CompressedGrid, CompressionError> {
    if grid.is_empty() { return Ok(CompressedGrid::new(0, 0, false)); }

    let height = grid.len() as u32;
    let width = grid[0].len() as u32;
    
    // Check if any pixel has color
    let has_color = grid.iter().any(|row| row.iter().any(|pixel| pixel.rgb.is_some()));

    let mut compressed = CompressedGrid::new(width, height, has_color);

    for row in grid { compressed.rows.push(compress_row(row)); }

    Ok(compressed)
}

/// Compress a single row using run-length encoding
fn compress_row(row: &[AsciiPixel]) -> Vec<RleEntry> {
    let mut compressed_row = Vec::new();
    
    if row.is_empty() { return compressed_row; }

    let mut current_pixel = row[0].clone();
    let mut count = 1u32;

    for pixel in row.iter().skip(1) {
        // Check if pixels are the same (character AND color must match)
        if pixels_equal(pixel, &current_pixel) {
            count += 1;
        } else {
            // Save the current run
            compressed_row.push(RleEntry {
                count,
                pixel: current_pixel.clone(),
            });
            
            // Start new run
            current_pixel = pixel.clone();
            count = 1;
        }
    }

    compressed_row.push(RleEntry {
        count,
        pixel: current_pixel,
    });

    compressed_row
}

/// Check if two pixels are equal (character and color must match)
fn pixels_equal(a: &AsciiPixel, b: &AsciiPixel) -> bool {
    a.ch == b.ch && a.rgb == b.rgb
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::converter::ascii_pixel::AsciiPixel;

    #[test]
    fn test_empty_grid() {
        let grid = vec![];
        let compressed = compress_grid(&grid).unwrap();
        assert_eq!(compressed.width, 0);
        assert_eq!(compressed.height, 0);
        assert_eq!(compressed.rows.len(), 0);
    }

    #[test]
    fn test_single_pixel() {
        let grid = vec![vec![AsciiPixel{ch: 'A', rgb: None}]];
        let compressed = compress_grid(&grid).unwrap();
        assert_eq!(compressed.width, 1);
        assert_eq!(compressed.height, 1);
        assert_eq!(compressed.rows[0].len(), 1);
        assert_eq!(compressed.rows[0][0].count, 1);
        assert_eq!(compressed.rows[0][0].pixel.ch, 'A');
    }
}