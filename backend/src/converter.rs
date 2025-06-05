use image::{GenericImageView, Rgb};
use std::error::Error;
use serde::Serialize;

pub struct Converter;

#[derive(Serialize)]
pub struct AsciiPixel {
    pub ch: char,
    pub rgb: [u8; 3],
}

impl Converter {
    // Helper function to adjust RGB color for brightness
    fn adjust_color(rgb: &Rgb<u8>, brightness_factor: f32) -> [u8; 3] {
        let [r, g, b] = [rgb[0] as f32, rgb[1] as f32, rgb[2] as f32];
        let new_r = (r * brightness_factor).min(255.0) as u8;
        let new_g = (g * brightness_factor).min(255.0) as u8;
        let new_b = (b * brightness_factor).min(255.0) as u8;
        [new_r, new_g, new_b]
    }

    pub fn convert_from_bytes(image_bytes: &[u8], output_width: u32, brightness_factor: f32, _use_background: bool) -> Result<Vec<Vec<AsciiPixel>>, Box<dyn Error>> {
        // ASCII characters from darkest to lightest
        const ASCII_CHARS: &[char] = &[
            ' ', '.', ':', ',', '-', '=', '+', '*', '@', '#',
            '%', '&', 'o', '0', 'O', '8', 'B', '#', '▒', '█',
        ];

        let img = image::load_from_memory(image_bytes)?;
        let (width, height) = img.dimensions();
        let img_gray = img.to_luma8();
        let aspect_ratio = height as f32 / width as f32;
        let output_height = (output_width as f32 * aspect_ratio * 0.55) as u32;
        let img_gray = image::imageops::resize(
            &img_gray,
            output_width,
            output_height,
            image::imageops::FilterType::Nearest,
        );
        let img_rgb = image::imageops::resize(
            &img.to_rgb8(),
            output_width,
            output_height,
            image::imageops::FilterType::Nearest,
        );
        let mut ascii_grid = Vec::with_capacity(output_height as usize);
        for y in 0..output_height {
            let mut row = Vec::with_capacity(output_width as usize);
            for x in 0..output_width {
                let pixel_gray = img_gray.get_pixel(x, y);
                let intensity = pixel_gray[0];
                let index = (intensity as usize * (ASCII_CHARS.len() - 1)) / 255;
                let ascii_char = ASCII_CHARS[index];
                let pixel_rgb = img_rgb.get_pixel(x, y);
                let rgb = Self::adjust_color(pixel_rgb, brightness_factor);
                row.push(AsciiPixel { ch: ascii_char, rgb });
            }
            ascii_grid.push(row);
        }
        Ok(ascii_grid)
    }
}