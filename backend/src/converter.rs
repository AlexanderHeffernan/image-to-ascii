use image::{GenericImageView, Rgb};
use std::error::Error;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct Converter;

impl Converter {
    // Helper function to adjust RGB color for brightness
    fn adjust_color(rgb: &Rgb<u8>, brightness_factor: f32) -> [u8; 3] {
        let [r, g, b] = [rgb[0] as f32, rgb[1] as f32, rgb[2] as f32];
        let new_r = (r * brightness_factor).min(255.0) as u8;
        let new_g = (g * brightness_factor).min(255.0) as u8;
        let new_b = (b * brightness_factor).min(255.0) as u8;
        [new_r, new_g, new_b]
    }

    pub fn convert(image_path: &str, output_width: u32, brightness_factor: f32, use_background: bool) -> Result<(), Box<dyn Error>> {
        // ASCII characters from darkest to lightest
        const ASCII_CHARS: &[char] = &[
            ' ', '.', ':', ',', '-', '=', '+', '*', '@', '#',
            '%', '&', 'o', '0', 'O', '8', 'B', '#', '▒', '█',
        ];

        let img = image::open(image_path)?;
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
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        let mut color_spec = ColorSpec::new();
        for y in 0..output_height {
            for x in 0..output_width {
                let pixel_gray = img_gray.get_pixel(x, y);
                let intensity = pixel_gray[0];
                let index = (intensity as usize * (ASCII_CHARS.len() - 1)) / 255;
                let ascii_char = ASCII_CHARS[index];
                let pixel_rgb = img_rgb.get_pixel(x, y);
                let [r, g, b] = Self::adjust_color(pixel_rgb, brightness_factor);
                color_spec.set_fg(Some(Color::Rgb(r, g, b)));
                if use_background {
                    let bg_r = 255 - r;
                    let bg_g = 255 - g;
                    let bg_b = 255 - b;
                    color_spec.set_bg(Some(Color::Rgb(bg_r, bg_g, bg_b)));
                } else {
                    color_spec.set_bg(None);
                }
                stdout.set_color(&color_spec)?;
                write!(&mut stdout, "{}", ascii_char)?;
            }
            stdout.reset()?;
            writeln!(&mut stdout)?;
        }
        Ok(())
    }
}