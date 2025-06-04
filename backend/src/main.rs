mod converter;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Configuration
    let img_path = "images/image.png";
    let output_width: u32 = 200;
    let brightness_factor = 1.5;
    let use_background = false;

    // Call the Converter's static function
    converter::Converter::convert(img_path, output_width, brightness_factor, use_background)?;
    Ok(())
}