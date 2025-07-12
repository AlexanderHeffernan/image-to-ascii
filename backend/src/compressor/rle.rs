use crate::converter::ascii_pixel::AsciiPixel;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct RleEntry {
    pub count: u32,
    pub pixel: AsciiPixel,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompressedGrid {
    pub width: u32,
    pub height: u32,
    pub has_color: bool,
    pub rows: Vec<Vec<RleEntry>>,
}

impl CompressedGrid {
    pub fn new(width: u32, height: u32, has_color: bool) -> Self {
        Self {
            width,
            height,
            has_color,
            rows: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum CompressionError {
    InvalidGrid(String),
    DecompressionError(String),
}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompressionError::InvalidGrid(msg) => write!(f, "Invalid grid: {}", msg),
            CompressionError::DecompressionError(msg) => write!(f, "Decompression error: {}", msg),
        }
    }
}

impl std::error::Error for CompressionError {}

// Main compression function expected by tests
pub fn compress_grid(grid: &[Vec<AsciiPixel>]) -> Result<CompressedGrid, CompressionError> {
    if grid.is_empty() {
        return Ok(CompressedGrid::new(0, 0, false));
    }

    let height = grid.len() as u32;
    let width = grid[0].len() as u32;
    
    // Check if any pixel has color
    let has_color = grid.iter()
        .any(|row| row.iter().any(|pixel| pixel.rgb.is_some()));

    let mut compressed = CompressedGrid::new(width, height, has_color);

    for row in grid {
        let mut compressed_row = Vec::new();
        
        if row.is_empty() {
            compressed.rows.push(compressed_row);
            continue;
        }

        let mut current_pixel = row[0].clone();
        let mut count = 1u32;

        for pixel in row.iter().skip(1) {
            // Check if pixels are the same (character AND color must match)
            if pixel.ch == current_pixel.ch && pixel.rgb == current_pixel.rgb {
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

        // Don't forget the last run
        compressed_row.push(RleEntry {
            count,
            pixel: current_pixel,
        });

        compressed.rows.push(compressed_row);
    }

    Ok(compressed)
}

// Main decompression function expected by tests
pub fn decompress_grid(compressed: &CompressedGrid) -> Result<Vec<Vec<AsciiPixel>>, CompressionError> {
    if compressed.width == 0 || compressed.height == 0 {
        return Ok(vec![]);
    }

    let mut result = Vec::new();

    for (row_idx, compressed_row) in compressed.rows.iter().enumerate() {
        let mut row = Vec::new();
        
        for entry in compressed_row {
            for _ in 0..entry.count {
                row.push(entry.pixel.clone());
            }
        }

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

// Aliases for backward compatibility
pub fn compress_ascii_grid(grid: &[Vec<AsciiPixel>]) -> Result<CompressedGrid, CompressionError> {
    compress_grid(grid)
}

pub fn decompress_ascii_grid(compressed: &CompressedGrid) -> Result<Vec<Vec<AsciiPixel>>, CompressionError> {
    decompress_grid(compressed)
}

// Keep the old implementation for simple char-based compression
#[derive(Debug, Clone)]
pub struct RleCompressed {
    pub data: Vec<(u32, char)>,
    pub width: usize,
    pub height: usize,
}

impl RleCompressed {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: Vec::new(),
            width,
            height,
        }
    }

    pub fn compressed_size(&self) -> usize {
        self.data.len()
    }

    pub fn compression_ratio(&self) -> f64 {
        let original_size = self.width * self.height;
        if original_size == 0 {
            return 0.0;
        }
        self.compressed_size() as f64 / original_size as f64
    }
}

impl fmt::Display for RleCompressed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RLE({}x{}, {} runs)", self.width, self.height, self.data.len())
    }
}

pub fn compress(grid: &[Vec<char>]) -> RleCompressed {
    if grid.is_empty() {
        return RleCompressed::new(0, 0);
    }

    let height = grid.len();
    let width = grid[0].len();
    let mut compressed = RleCompressed::new(width, height);

    if width == 0 {
        return compressed;
    }

    // Process the grid as a flat sequence
    let mut chars_iter = grid.iter().flatten();
    
    if let Some(&first_char) = chars_iter.next() {
        let mut current_char = first_char;
        let mut count = 1u32;

        for &ch in chars_iter {
            if ch == current_char {
                count += 1;
            } else {
                compressed.data.push((count, current_char));
                current_char = ch;
                count = 1;
            }
        }

        // Don't forget the last run
        compressed.data.push((count, current_char));
    }

    compressed
}

pub fn decompress(compressed: &RleCompressed) -> Vec<Vec<char>> {
    if compressed.width == 0 || compressed.height == 0 {
        return vec![];
    }

    let mut result = vec![vec![' '; compressed.width]; compressed.height];
    let mut pos = 0;

    for &(count, ch) in &compressed.data {
        for _ in 0..count {
            if pos >= compressed.width * compressed.height {
                break;
            }
            let row = pos / compressed.width;
            let col = pos % compressed.width;
            result[row][col] = ch;
            pos += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_grid() {
        let grid = vec![];
        let compressed = compress(&grid);
        assert_eq!(compressed.width, 0);
        assert_eq!(compressed.height, 0);
        assert_eq!(compressed.data.len(), 0);
    }

    #[test]
    fn test_single_character() {
        let grid = vec![vec!['A']];
        let compressed = compress(&grid);
        assert_eq!(compressed.width, 1);
        assert_eq!(compressed.height, 1);
        assert_eq!(compressed.data, vec![(1, 'A')]);
    }

    #[test]
    fn test_compression_decompression() {
        let original = vec![
            vec!['A', 'A', 'B', 'B', 'B'],
            vec!['A', 'A', 'A', 'C', 'C'],
        ];
        let compressed = compress(&original);
        let decompressed = decompress(&compressed);
        assert_eq!(original, decompressed);
    }

    #[test]
    fn test_compression_ratio() {
        let grid = vec![
            vec!['A', 'A', 'A', 'A'],
            vec!['B', 'B', 'B', 'B'],
        ];
        let compressed = compress(&grid);
        assert_eq!(compressed.compression_ratio(), 0.25); // 2 runs vs 8 original chars
    }
}