use image::{GenericImageView, Rgb};
use serde::Serialize;

/// Main converter struct (namespace only)
pub struct Converter;

/// Represents a single ASCII pixel, with optional color.
#[derive(Serialize)]
pub struct AsciiPixel {
    pub ch: char,
    pub rgb: Option<[u8; 3]>, // None for no-color output
}

/// Configuration for ASCII conversion.
#[derive(Debug, Clone)]
pub struct ConverterConfig {
    pub character_set: Vec<char>,       // User-defined character set
    pub output_width: u32,              // Output width in characters
    pub output_height: Option<u32>,     // Optional output height; if None, computed from aspect ratio
    pub brightness_factor: f32,         // Brightness adjustment (e.g., 1.0 = no change)
    pub contrast_factor: f32,           // Contrast adjustment (e.g., 1.0 = no change)
    pub is_color: bool,                 // Color or no-color output
    pub aspect_ratio_correction: f32,   // Correction factor for aspect ratio (default 0.55 for ASCII art)
}

/// Errors that can occur during conversion.
#[derive(Debug)]
pub enum ConverterError {
    ImageError(image::ImageError),
    InvalidParameter(String),
}

impl From<image::ImageError> for ConverterError {
    fn from(err: image::ImageError) -> Self {
        ConverterError::ImageError(err)
    }
}

impl std::fmt::Display for ConverterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConverterError::ImageError(err) => write!(f, "Image error: {}", err),
            ConverterError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
        }
    }
}

impl std::error::Error for ConverterError {}

impl Converter {
    // Default character set (dark to light)
    pub const DEFAULT_CHARS: &'static [char] = &[
        ' ', '.', ':', ',', '-', '=', '+', '*', '@', '#',
        '%', '&', 'o', '0', 'O', '8', 'B', '#', '▒', '█',
    ];

    /// Validates the configuration for sensible values.
    fn validate_config(config: &ConverterConfig) -> Result<(), ConverterError> {
        if config.output_width == 0 {
            return Err(ConverterError::InvalidParameter("Output width must be greater than 0".into()));
        }
        if config.brightness_factor <= 0.0 {
            return Err(ConverterError::InvalidParameter("Brightness factor must be positive".into()));
        }
        if config.contrast_factor <= 0.0 {
            return Err(ConverterError::InvalidParameter("Contrast factor must be positive".into()));
        }
        Ok(())
    }

    /// Adjusts an RGB color for brightness and contrast.
    /// Brightness is applied first, then contrast.
    fn adjust_color(rgb: &Rgb<u8>, brightness: f32, contrast: f32) -> [u8; 3] {
        let [r, g, b] = [rgb[0] as f32, rgb[1] as f32, rgb[2] as f32];
        // Apply brightness first, then contrast
        let adjust = |v: f32| (((v * brightness - 128.0) * contrast) + 128.0).clamp(0.0, 255.0);
        [
            adjust(r) as u8,
            adjust(g) as u8,
            adjust(b) as u8,
        ]
    }

    /// Maps a grayscale intensity (0-255) to a character from the set.
    fn intensity_to_char(intensity: u8, chars: &[char]) -> char {
        let index = (intensity as usize * (chars.len() - 1)) / 255;
        chars[index]
    }

    /// Builds the ASCII grid from a generic image buffer using a pixel getter closure.
    /// The closure should return (intensity, Optional<rgb>) for each (x, y).
    fn build_ascii_grid<F>(
        output_width: u32,
        output_height: u32,
        character_set: &[char],
        mut get_pixel: F,
    ) -> Vec<Vec<AsciiPixel>>
    where
        F: FnMut(u32, u32) -> (u8, Option<[u8; 3]>),
    {
        let mut ascii_grid = Vec::with_capacity(output_height as usize);
        for y in 0..output_height {
            let mut row = Vec::with_capacity(output_width as usize);
            for x in 0..output_width {
                let (intensity, rgb) = get_pixel(x, y);
                let ascii_char = Self::intensity_to_char(intensity, character_set);
                row.push(AsciiPixel { ch: ascii_char, rgb });
            }
            ascii_grid.push(row);
        }
        ascii_grid
    }
    

    /// Converts an image (as bytes) to a 2D ASCII grid.
    /// Returns a grid of AsciiPixel structs.
    pub fn convert_from_bytes(
        image_bytes: &[u8], 
        config: ConverterConfig
    ) -> Result<Vec<Vec<AsciiPixel>>, ConverterError> {
        Self::validate_config(&config)?;
        
        // Load image from bytes
        let mut img = image::load_from_memory(image_bytes)?;
        let (width, height) = img.dimensions();

        // Calculate output height if not specified
        let output_height = config.output_height.unwrap_or_else(|| {
            ((config.output_width as f32 * height as f32 / width as f32) * config.aspect_ratio_correction) as u32
        });

        if output_height == 0 {
            return Err(ConverterError::InvalidParameter("Calculated output height is 0".into()));
        }

        // Branch for color or grayscale processing, but use the same grid builder
        if config.is_color {
            let img_rgb = image::imageops::resize(
                &img.to_rgb8(), 
                config.output_width, 
                output_height, 
                image::imageops::FilterType::Lanczos3
            );
            Ok(Self::build_ascii_grid(
                config.output_width,
                output_height,
                &config.character_set,
                |x, y| {
                    let pixel = img_rgb.get_pixel(x, y);
                    let adjusted_rgb = Self::adjust_color(pixel, config.brightness_factor, config.contrast_factor);
                    let intensity = (0.299 * adjusted_rgb[0] as f32 + 0.587 * adjusted_rgb[1] as f32 + 0.114 * adjusted_rgb[2] as f32) as u8;
                    (intensity, Some(adjusted_rgb))
                },
            ))
        } else {
            let img_gray = image::imageops::resize(
                &img.to_luma8(),
                config.output_width,
                output_height,
                image::imageops::FilterType::Nearest,
            );
            Ok(Self::build_ascii_grid(
                config.output_width,
                output_height,
                &config.character_set,
                |x, y| {
                    let pixel = img_gray.get_pixel(x, y);
                    let intensity = pixel[0]; // Luma pixel intensity
                    (intensity, None) // No color for no-color output
                },
            ))
        }
    }
}