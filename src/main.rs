use image::{GenericImageView, Rgb};
use std::error::Error;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

// Helper function to adjust RGB color for brightness
fn adjust_color(rgb: &Rgb<u8>, brightness_factor: f32) -> [u8; 3] {
    let [r, g, b] = [rgb[0] as f32, rgb[1] as f32, rgb[2] as f32];
    // Simple brightness scaling, preserving hue
    let new_r = (r * brightness_factor).min(255.0) as u8;
    let new_g = (g * brightness_factor).min(255.0) as u8;
    let new_b = (b * brightness_factor).min(255.0) as u8;
    [new_r, new_g, new_b]
}

fn main() -> Result<(), Box<dyn Error>> {
    // ASCII characters from darkest to lightest
    const ASCII_CHARS: &[char] = &[
        ' ', '.', ':', ',', '-', '=', '+', '*', '@', '#',
        '%', '&', 'o', '0', 'O', '8', 'B', '#', '▒', '█',
    ];

    // Configuration
    let img_path = "images/Portrait-small.jpg";
    let output_width: u32 = 50; // Reduced to avoid wrapping
    let brightness_factor = 1.5; // Adjust brightness (1.0 = no change, >1.0 = brighter)
    let use_background = false; // Set to true for contrasting backgrounds

    // Load and process image
    let img = image::open(img_path)?;
    let (width, height) = img.dimensions();

    // Convert to grayscale for ASCII mapping and saving
    let img_gray = img.to_luma8();

    // Calculate new height to preserve aspect ratio
    let aspect_ratio = height as f32 / width as f32;
    let output_height = (output_width as f32 * aspect_ratio * 0.55) as u32;

    // Resize grayscale image for ASCII mapping
    let img_gray = image::imageops::resize(
        &img_gray,
        output_width,
        output_height,
        image::imageops::FilterType::Nearest,
    );

    // Resize RGB image for color extraction
    let img_rgb = image::imageops::resize(
        &img.to_rgb8(),
        output_width,
        output_height,
        image::imageops::FilterType::Nearest,
    );

    // Initialize terminal output with color support
    let mut stdout = StandardStream::stdout(ColorChoice::Always); // Force true color
    let mut color_spec = ColorSpec::new();

    // Convert pixels to colored ASCII
    for y in 0..output_height {
        for x in 0..output_width {
            // Get grayscale intensity for ASCII character selection
            let pixel_gray = img_gray.get_pixel(x, y);
            let intensity = pixel_gray[0]; // 0-255
            let index = (intensity as usize * (ASCII_CHARS.len() - 1)) / 255;
            let ascii_char = ASCII_CHARS[index];

            // Get RGB color and adjust brightness
            let pixel_rgb = img_rgb.get_pixel(x, y);
            let [r, g, b] = adjust_color(pixel_rgb, brightness_factor);

            // Debug: Print RGB values for first few pixels
            if x < 5 && y == 0 {
                eprintln!("Pixel ({}, {}): Original RGB={:?}, Adjusted RGB=[{}, {}, {}]", x, y, pixel_rgb.0, r, g, b);
            }

            // Set terminal color
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