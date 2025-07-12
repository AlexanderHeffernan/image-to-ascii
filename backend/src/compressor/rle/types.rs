use crate::converter::ascii_pixel::AsciiPixel;
use std::fmt;
use serde::{Serialize, Deserialize};

/// A single run-length encoded entry containing a count and a pixel
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RleEntry {
    pub count: u32,
    pub pixel: AsciiPixel,
}

/// A compressed ASCII grid using run-length encoding
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompressedGrid {
    pub width: u32,
    pub height: u32,
    pub has_color: bool,
    pub rows: Vec<Vec<RleEntry>>,
}

impl CompressedGrid {
    /// Create a new empty compressed grid
    pub fn new(width: u32, height: u32, has_color: bool) -> Self {
        Self {
            width,
            height,
            has_color,
            rows: Vec::new(),
        }
    }
}

/// Errors that can occur during compression/decompression
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