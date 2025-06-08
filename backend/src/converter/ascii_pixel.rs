use serde::Serialize;

/// Represents a single ASCII pixel, with optional color.
#[derive(Serialize)]
pub struct AsciiPixel {
    pub ch: char,
    pub rgb: Option<[u8; 3]>, // None for no-color output
}