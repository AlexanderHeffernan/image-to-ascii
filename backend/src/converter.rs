use image::{GenericImageView, Rgb};
use serde::Serialize;

pub struct Converter;

#[derive(Serialize)]
pub struct AsciiPixel {
    pub ch: char,
    pub rgb: Option<[u8; 3]>, // None for no-color output
}

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

    // Validate configuration
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

    // Adjust RGB color for brightness and contrast
    fn adjust_color(rgb: &Rgb<u8>, brightness: f32, contrast: f32) -> [u8; 3] {
        let [r, g, b] = [rgb[0] as f32, rgb[1] as f32, rgb[2] as f32];
        // Apply contrast (centered around 128)
        let contrast_adj = |v: f32| ((v - 128.0) * contrast + 128.0).clamp(0.0, 255.0);
        // Apply brightness
        let new_r = (contrast_adj(r) * brightness).clamp(0.0, 255.0) as u8;
        let new_g = (contrast_adj(g) * brightness).clamp(0.0, 255.0) as u8;
        let new_b = (contrast_adj(b) * brightness).clamp(0.0, 255.0) as u8;
        [new_r, new_g, new_b]
    }

    // Map pixel intensity to a character
    fn intensity_to_char(intensity: u8, chars: &[char]) -> char {
        let index = (intensity as usize * (chars.len() - 1)) / 255;
        chars[index]
    }

    pub fn convert_from_bytes(
        image_bytes: &[u8], 
        config: ConverterConfig
    ) -> Result<Vec<Vec<AsciiPixel>>, ConverterError> {
        Self::validate_config(&config)?;

        // Load and process image
        let img = image::load_from_memory(image_bytes)?;
        let (width, height) = img.dimensions();

        // Calculate output height if not specified
        let output_height = config.output_height.unwrap_or_else(|| {
            ((config.output_width as f32 * height as f32 / width as f32) * config.aspect_ratio_correction) as u32
        });

        if output_height == 0 {
            return Err(ConverterError::InvalidParameter("Calculated output height is 0".into()));
        }

        // Resize grayscale image for intensity
        let img_gray = img.to_luma8();
        let img_gray = image::imageops::resize(
            &img_gray,
            config.output_width,
            output_height,
            image::imageops::FilterType::Nearest,
        );

        // Resize RGB image only if color mode is enabled
        let img_rgb = if config.is_color {
            Some(image::imageops::resize(
                &img.to_rgb8(),
                config.output_width,
                output_height,
                image::imageops::FilterType::Nearest,
            ))
        } else {
            None
        };

        // Convert to ASCII
        let mut ascii_grid = Vec::with_capacity(output_height as usize);
        for y in 0..output_height {
            let mut row = Vec::with_capacity(config.output_width as usize);
            for x in 0..config.output_width {
                let pixel_gray = img_gray.get_pixel(x, y);
                let intensity = pixel_gray[0];
                let ascii_char = Self::intensity_to_char(intensity, &config.character_set);
                let rgb = if config.is_color {
                    let pixel_rgb = img_rgb.as_ref().unwrap().get_pixel(x, y);
                    Some(Self::adjust_color(pixel_rgb, config.brightness_factor, config.contrast_factor))
                } else {
                    None
                };
                row.push(AsciiPixel { ch: ascii_char, rgb });
            }
            ascii_grid.push(row);
        }
        Ok(ascii_grid)
    }
}