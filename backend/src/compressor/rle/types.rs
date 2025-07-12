use crate::converter::AsciiPixel;
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
#[allow(dead_code)]
pub enum CompressionError {
    DecompressionError(String),
}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompressionError::DecompressionError(msg) => write!(f, "Decompression error: {}", msg),
        }
    }
}

impl std::error::Error for CompressionError {}