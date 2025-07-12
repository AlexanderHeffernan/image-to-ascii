use serde::{Serialize, Deserialize};

/// Represents a single ASCII pixel, with optional color.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AsciiPixel {
    pub ch: char,
    pub rgb: Option<[u8; 3]>, // None for no-color output
}