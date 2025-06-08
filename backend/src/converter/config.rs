use serde::Serialize;

/// Configuration for ASCII conversion.
#[derive(Debug, Clone, Serialize)]
pub struct ConverterConfig {
    pub character_set: Vec<char>,       // User-defined character set
    pub output_width: u32,              // Output width in characters
    pub output_height: Option<u32>,     // Optional output height; if None, computed from aspect ratio
    pub brightness_factor: f32,         // Brightness adjustment (e.g., 1.0 = no change)
    pub contrast_factor: f32,           // Contrast adjustment (e.g., 1.0 = no change)
    pub is_color: bool,                 // Color or no-color output
    pub aspect_ratio_correction: f32,   // Correction factor for aspect ratio (default 0.55 for ASCII art)
}